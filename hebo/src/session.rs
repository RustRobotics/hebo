// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{
    ByteArray, ConnectPacket, ConnectReturnCode, DecodePacket, DisconnectPacket, EncodePacket,
    FixedHeader, PacketType, PingRequestPacket, PingResponsePacket, PublishAckPacket,
    PublishPacket, SubscribeAck, SubscribeAckPacket, SubscribePacket, UnsubscribeAckPacket,
    UnsubscribePacket,
};
use std::convert::Into;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

use crate::commands::{ListenerToSessionCmd, SessionId, SessionToListenerCmd};
use crate::error::Error;
use crate::stream::Stream;

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
// TODO(Shaohua): Disconnect the network if ClientId is inuse
// TODO(Shaohua): Disconnect the network if Connect Packet is invalid
// TODO(Shaohua): Disconnect the network if Connect Packet is not received within a reasonable
// amount of time.
#[derive(Debug)]
pub struct Session {
    id: SessionId,
    stream: Stream,
    sender: Sender<SessionToListenerCmd>,
    receiver: Receiver<ListenerToSessionCmd>,
    status: Status,
    client_id: String,
    // TODO(Shaohua): Add session flag
    // TODO(Shaohua): Add keep alive
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
            sender,
            receiver,
            status: Status::Invalid,
            client_id: String::new(),
        }
    }

    pub async fn run_loop(mut self) {
        // TODO(Shaohua): Set buffer cap based on settings
        let mut buf = Vec::with_capacity(512);
        // TODO(Shaohua): Handle timeout
        let mut timer = interval(Duration::from_secs(20));
        loop {
            tokio::select! {
                Ok(n_recv) = self.stream.read_buf(&mut buf) => {
                    log::info!("n_recv: {}", n_recv);
                    if n_recv > 0 {
                        if let Err(err) = self.handle_client_packet(&buf).await {
                            log::error!("handle_client_packet() failed: {:?}", err);
                        }
                        buf.clear();
                    } else {
                        break;
                    }
                }
                _ = timer.tick() => {
                    // TODO(Shaohua): Send ping
                    //log::info!("tick()");
                },
                Some(cmd) = self.receiver.recv() => {
                    if let Err(err) = self.handle_listener_packet(cmd).await {
                        log::error!("Failed to handle server packet: {:?}", err);
                    }
                },
                else => break,
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

    async fn send<P: EncodePacket>(&mut self, packet: P) -> Result<(), Error> {
        let mut buf = Vec::new();
        packet.encode(&mut buf)?;
        self.stream.write(&buf).await.map(drop).map_err(Into::into)
    }

    async fn handle_client_packet(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let fixed_header = FixedHeader::decode(&mut ba)?;

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
        let packet = ConnectPacket::decode(&mut ba)?;
        self.client_id = packet.client_id().to_string();

        // TODO(Shaohua): Handle keep alive

        // Check connection status first.
        // If this client is already connected, send disconnect packet.
        if self.status == Status::Connected || self.status == Status::Connecting {
            let packet = DisconnectPacket::new();
            self.status = Status::Disconnecting;
            return self.send(packet).await.map(drop);
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
        // TODO(Shaohua): Update last_message_timestamp.
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
