// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{
    ByteArray, ConnectAckPacket, ConnectPacket, ConnectReturnCode, DecodePacket, EncodePacket,
    FixedHeader, PacketType, PingRequestPacket, PingResponsePacket, PublishPacket, SubscribeAck,
    SubscribeAckPacket, SubscribePacket, UnsubscribeAckPacket, UnsubscribePacket,
};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;

use crate::commands::{ListenerToSessionCmd, SessionId, SessionToListenerCmd};
use crate::error::Error;
use crate::stream::Stream;

#[derive(Debug)]
enum Status {
    Invalid,
    Connected,
    Disconnected,
}

/// ConnectionContext represents a client connection.
/// All the status of this client is maintained in this struct.
///
// TODO(Shaohua): Handle Session State
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
    sender: mpsc::Sender<SessionToListenerCmd>,
    receiver: mpsc::Receiver<ListenerToSessionCmd>,
    status: Status,
    client_id: String,
    // TODO(Shaohua): Add session flag
    // TODO(Shaohua): Add keep alive
}

impl Session {
    pub fn new(
        id: SessionId,
        stream: Stream,
        sender: mpsc::Sender<SessionToListenerCmd>,
        receiver: mpsc::Receiver<ListenerToSessionCmd>,
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
        packet.encode(&mut buf).unwrap();
        self.stream
            .write(&buf)
            .await
            .map(drop)
            .map_err(|err| err.into())
    }

    async fn handle_client_packet(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let fixed_header = FixedHeader::decode(&mut ba)?;

        match fixed_header.packet_type {
            PacketType::Connect => {
                if let Err(err) = self.connect(&buf).await {
                    log::warn!("connect() failed! {:?}", err);
                    return Err(err);
                }
            }
            PacketType::PingRequest => {
                if let Err(err) = self.ping(&buf).await {
                    log::warn!("ping() failed! {:?}", err);
                    return Err(err);
                }
            }
            PacketType::Publish { .. } => {
                if let Err(err) = self.publish(&buf).await {
                    log::warn!("publish() failed! {:?}", err);
                    return Err(err);
                }
            }
            PacketType::PublishRelease { .. } => {
                // Do nothing currently
            }
            PacketType::Subscribe => {
                if let Err(err) = self.subscribe(&buf).await {
                    log::warn!("subscribe() failed! {:?}", err);
                    return Err(err);
                }
            }
            PacketType::Unsubscribe => {
                if let Err(err) = self.unsubscribe(&buf).await {
                    log::warn!("unsubscribe() failed! {:?}", err);
                    return Err(err);
                }
            }
            PacketType::Disconnect => {
                if let Err(err) = self.disconnect(&buf).await {
                    log::warn!("disconnect() failed! {:?}", err);
                    return Err(err);
                }
            }
            t => {
                log::warn!("Unhandled msg: {:?}", t);
            }
        }
        return Ok(());
    }

    async fn connect(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = ConnectPacket::decode(&mut ba)?;
        self.client_id = packet.client_id().to_string();
        // TODO(Shaohua): Handle keep alive
        // TODO(Shaohua): Check connection status first.
        // TODO(Shaohua): If this client is already connected, send disconnect packet.
        let packet = ConnectAckPacket::new(true, ConnectReturnCode::Accepted);
        self.status = Status::Connected;
        self.send(packet).await.map(drop)
    }

    async fn ping(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let _packet = PingRequestPacket::decode(&mut ba)?;
        let ping_resp_packet = PingResponsePacket::new();
        self.send(ping_resp_packet).await
    }

    async fn publish(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = PublishPacket::decode(&mut ba)?;
        // TODO(Shaohua): Send PublishAck if qos == 0
        //let publish_ack_packet = PublishAckPacket::new(packet.packet_id());
        //self.send(publish_ack_packet).await?;
        self.sender
            .send(SessionToListenerCmd::Publish(packet))
            .await
            .map(drop)?;
        Ok(())
    }

    async fn subscribe(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = SubscribePacket::decode(&mut ba)?;

        // Send to listener, which will check auth.
        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::Subscribe(self.id, packet.clone()))
            .await
        {
            log::error!("Failed to send subscribe command to server: {:?}", err);
            let ack = SubscribeAck::Failed;

            let subscribe_ack_packet = SubscribeAckPacket::new(ack, packet.packet_id());
            self.send(subscribe_ack_packet).await
        } else {
            Ok(())
        }
    }

    async fn unsubscribe(&mut self, buf: &[u8]) -> Result<(), Error> {
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

    async fn disconnect(&mut self, _buf: &[u8]) -> Result<(), Error> {
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
            ListenerToSessionCmd::Publish(packet) => self.listener_publish(packet).await?,
        }
        Ok(())
    }

    async fn listener_publish(&mut self, packet: PublishPacket) -> Result<(), Error> {
        self.send(packet).await
    }
}
