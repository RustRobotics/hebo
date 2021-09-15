// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{
    ByteArray, ConnectPacket, ConnectReturnCode, DecodeError, DecodePacket, DisconnectPacket,
    EncodePacket, FixedHeader, PacketType, PingRequestPacket, PingResponsePacket, PublishAckPacket,
    PublishPacket, SubscribeAck, SubscribeAckPacket, SubscribePacket, UnsubscribeAckPacket,
    UnsubscribePacket,
};
use std::convert::Into;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{ListenerToSessionCmd, SessionToListenerCmd};
use crate::error::Error;
use crate::stream::Stream;
use crate::types::SessionId;

#[derive(Debug, PartialEq)]
enum Status {
    Invalid,
    Connecting,
    Connected,
    Disconnecting,
    Disconnected,
}

/// ConnectionContext represents a client connection.
/// All the status of this client is maintained in this struct.
///
// TODO(Shaohua): Handle Clean Session operation
// TODO(Shaohua): Handle Will Message
#[derive(Debug)]
pub struct Session {
    id: SessionId,
    stream: Stream,
    status: Status,
    client_id: String,
    // TODO(Shaohua): Add session flag
    keep_alive: u64,
    instant: Instant,

    sender: Sender<SessionToListenerCmd>,
    receiver: Receiver<ListenerToSessionCmd>,
}

impl Session {
    pub fn new(
        id: SessionId,
        stream: Stream,
        sender: Sender<SessionToListenerCmd>,
        receiver: Receiver<ListenerToSessionCmd>,
    ) -> Session {
        Session {
            id,
            stream,

            status: Status::Invalid,
            client_id: String::new(),
            keep_alive: 0,
            instant: Instant::now(),

            sender,
            receiver,
        }
    }

    pub async fn run_loop(mut self) {
        // TODO(Shaohua): Set buffer cap based on settings
        let mut buf = Vec::with_capacity(1024);

        loop {
            if self.status == Status::Disconnected {
                break;
            }

            tokio::select! {
                Ok(n_recv) = self.stream.read_buf(&mut buf) => {
                    log::info!("n_recv: {}", n_recv);
                    if n_recv > 0 {
                        if let Err(err) = self.handle_client_packet(&buf).await {
                            log::error!("handle_client_packet() failed: {:?}", err);
                            break;
                        }
                        buf.clear();

                    } else {
                        log::info!("session: Empty packet received, disconnect client, {}", self.id);
                        self.send_disconnect().await;
                        break;
                    }
                }
                Some(cmd) = self.receiver.recv() => {
                    if let Err(err) = self.handle_listener_packet(cmd).await {
                        log::error!("Failed to handle server packet: {:?}", err);
                    }
                },
            }

            if self.keep_alive > 0 && self.instant.elapsed().as_secs() > self.keep_alive {
                log::warn!("sessoin: keep_alive time reached, disconnect client!");
                self.send_disconnect().await;
                break;
            }
        }

        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::Disconnect(self.id))
            .await
        {
            log::error!(
                "Failed to send disconnect cmd to server, id: {}, err: {:?}",
                self.id,
                err
            );
        }
    }

    /// Reset instant if packet is send to or receive from client.
    fn reset_instant(&mut self) {
        self.instant = Instant::now();
    }

    async fn send<P: EncodePacket>(&mut self, packet: P) -> Result<(), Error> {
        let mut buf = Vec::new();
        packet.encode(&mut buf)?;
        self.stream.write(&buf).await.map(drop)?;
        self.reset_instant();
        Ok(())
    }

    /// Send disconnect packet to client and update status.
    async fn send_disconnect(&mut self) {
        self.status = Status::Disconnecting;
        let packet = DisconnectPacket::new();
        if let Err(err) = self.send(packet).await.map(drop) {
            log::error!("session: Failed to send disconnect packet, {}", self.id);
        }
        self.status = Status::Disconnected;
    }

    async fn handle_client_packet(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let fixed_header = match FixedHeader::decode(&mut ba) {
            Ok(fixed_header) => fixed_header,
            Err(err) => {
                // Disconnect the network if Connect Packet is invalid.
                log::error!("session: Invalid packet: {:?}, content: {:?}", err, buf);
                self.send_disconnect().await;
                return Ok(());
            }
        };

        self.reset_instant();

        match fixed_header.packet_type {
            PacketType::Connect => self.on_client_connect(&buf).await,
            PacketType::PingRequest => self.on_client_ping(&buf).await,
            PacketType::Publish { .. } => self.on_client_publish(&buf).await,
            PacketType::PublishRelease { .. } => {
                // Do nothing currently
                Ok(())
            }
            PacketType::Subscribe => self.on_client_subscribe(&buf).await,
            PacketType::Unsubscribe => self.on_client_unsubscribe(&buf).await,
            PacketType::Disconnect => self.on_client_disconnect(&buf).await,
            t => {
                log::warn!("Unhandled msg: {:?}", t);
                Ok(())
            }
        }
    }

    async fn on_client_connect(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = match ConnectPacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => match err {
                // The Server MUST respond to the CONNECT Packet with a CONNACK return code
                // 0x01 (unacceptable protocol level) and then disconnect
                // the Client if the Protocol Level is not supported by the Server
                DecodeError::InvalidProtocolName | DecodeError::InvalidProtocolLevel => {
                    let ack_packet =
                        ConnectAckPacket::new(false, ConnectReturnCode::UnacceptedProtocol);
                    self.send(ack_packet).await?;
                    return Err(err.into());
                }
                _ => {
                    return Err(err.into());
                }
            },
        };
        self.client_id = packet.client_id().to_string();

        // Update keep_alive timer.
        self.keep_alive = packet.keep_alive as u64;

        // Check connection status first.
        // If this client is already connected, send disconnect packet.
        if self.status == Status::Connected || self.status == Status::Connecting {
            self.send_disconnect().await;
            return Ok(());
        }

        // Send the connect packet to listener.
        self.status = Status::Connecting;
        self.sender
            .send(SessionToListenerCmd::Connect(self.id, packet))
            .await
            .map(drop)?;
        Ok(())
    }

    async fn on_client_ping(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let _packet = PingRequestPacket::decode(&mut ba)?;

        // Send ping resp packet to client.
        let ping_resp_packet = PingResponsePacket::new();
        self.send(ping_resp_packet).await
    }

    async fn on_client_publish(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("on_client_publish()");
        let mut ba = ByteArray::new(buf);
        let packet = PublishPacket::decode(&mut ba)?;

        // Send publish ack packet to client.
        let ack_packet = PublishAckPacket::new(packet.packet_id());
        self.send(ack_packet).await?;

        // Send the publish packet to listener.
        self.sender
            .send(SessionToListenerCmd::Publish(packet))
            .await
            .map(drop)?;
        Ok(())
    }

    async fn on_client_subscribe(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = SubscribePacket::decode(&mut ba)?;

        // Send subscribe packet to listener, which will check auth.
        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::Subscribe(self.id, packet.clone()))
            .await
        {
            // Send subscribe ack (failed) to client.
            log::error!("Failed to send subscribe command to server: {:?}", err);
            let ack = SubscribeAck::Failed;

            let subscribe_ack_packet = SubscribeAckPacket::new(ack, packet.packet_id());
            self.send(subscribe_ack_packet).await
        } else {
            Ok(())
        }
    }

    async fn on_client_unsubscribe(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = UnsubscribePacket::decode(&mut ba)?;
        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::Unsubscribe(self.id, packet.clone()))
            .await
        {
            log::warn!("Failed to send unsubscribe command to server: {:?}", err);
        }

        let unsubscribe_ack_packet = UnsubscribeAckPacket::new(packet.packet_id());
        self.send(unsubscribe_ack_packet).await
    }

    async fn on_client_disconnect(&mut self, _buf: &[u8]) -> Result<(), Error> {
        self.status = Status::Disconnected;
        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::Disconnect(self.id))
            .await
        {
            log::warn!("Failed to send disconnect command to server: {:?}", err);
        }
        Ok(())
    }

    async fn handle_listener_packet(&mut self, cmd: ListenerToSessionCmd) -> Result<(), Error> {
        match cmd {
            ListenerToSessionCmd::ConnectAck(packet) => {
                self.status = if packet.return_code() == ConnectReturnCode::Accepted {
                    Status::Connected
                } else {
                    Status::Disconnected
                };

                self.send(packet).await
            }
            ListenerToSessionCmd::Publish(packet) => self.send(packet).await,
            ListenerToSessionCmd::SubscribeAck(packet) => self.send(packet).await,
        }
    }
}
