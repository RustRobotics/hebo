// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::base::{FixedHeader, FromNetPacket, PacketType, ToNetPacket};
use codec::connect_ack_packet::{ConnectAckPacket, ConnectReturnCode};
use codec::connect_packet::ConnectPacket;
use codec::ping_request_packet::PingRequestPacket;
use codec::ping_response_packet::PingResponsePacket;
use codec::publish_ack_packet::PublishAckPacket;
use codec::publish_packet::PublishPacket;
use codec::subscribe_ack_packet::{SubscribeAck, SubscribeAckPacket};
use codec::subscribe_packet::SubscribePacket;
use codec::unsubscribe_ack_packet::UnsubscribeAckPacket;
use codec::unsubscribe_packet::UnsubscribePacket;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

use crate::commands::{ConnectionCommand, ConnectionId, ServerCommand};
use crate::error;

#[derive(Debug)]
enum Status {
    Invalid,
    Connected,
    Disconnected,
}

/// ConnectionContext represents a client connection.
/// All the status of this client is mantinaed in this struct.
///
// TODO(Shaohua): Handle Session State
// TODO(Shaohua): Handle Clean Session operation
// TODO(Shaohua): Handle Will Message
// TODO(Shaohua): Disconnect the network if ClientId is inuse
// TODO(Shaohua): Disconnect the network if Connect Packet is invalid
// TODO(Shaohua): Disconnect the network if Connect Packet is not received within a reasonable
// amount of time.
#[derive(Debug)]
pub struct ConnectionContext {
    stream: TcpStream,
    remote_address: SocketAddr,
    connection_id: ConnectionId,
    sender: Sender<ConnectionCommand>,
    receiver: Receiver<ServerCommand>,
    status: Status,
    client_id: String,
}

impl ConnectionContext {
    pub fn new(
        stream: TcpStream,
        remote_address: SocketAddr,
        connection_id: ConnectionId,
        sender: Sender<ConnectionCommand>,
        receiver: Receiver<ServerCommand>,
    ) -> ConnectionContext {
        ConnectionContext {
            remote_address,
            stream,
            connection_id,
            sender,
            receiver,
            status: Status::Invalid,
            client_id: String::new(),
        }
    }

    pub async fn run_loop(mut self) {
        let mut buf = Vec::new();
        // TODO(Shaohua): Handle timeout
        let mut timer = interval(Duration::from_secs(10));
        loop {
            tokio::select! {
                Ok(n_recv) = self.stream.read_buf(&mut buf) => {
                    if n_recv > 0 {
                        log::info!("n_recv: {}", n_recv);
                        // TODO(Shaohua): Handle errors
                        let _result = self.handle_client_packet(&buf).await;
                        buf.clear();
                    }
                }
                _ = timer.tick() => {
                    log::info!("tick()");
                },
                Some(cmd) = self.receiver.recv() => {
                    // TODO(Shaohua): Handle errors
                    let _result = self.cmd_router(cmd).await;
                },
            }
        }
    }

    async fn send<P: ToNetPacket>(&mut self, packet: P) -> error::Result<()> {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        self.stream
            .write(&buf)
            .await
            .map(drop)
            .map_err(|err| err.into())
    }

    async fn handle_client_packet(&mut self, buf: &[u8]) -> error::Result<()> {
        let mut offset: usize = 0;
        let fixed_header = FixedHeader::from_net(&buf, &mut offset)?;

        match fixed_header.packet_type {
            PacketType::Connect => self.connect(&buf).await,
            PacketType::PingRequest => self.ping(&buf).await,
            PacketType::Publish => self.publish(&buf).await,
            PacketType::Subscribe => self.subscribe(&buf).await,
            PacketType::Unsubscribe => self.unsubscribe(&buf).await,
            PacketType::Disconnect => self.disconnect(&buf).await,
            t => {
                log::warn!("Unhandled msg: {:?}", t);
                return Ok(());
            }
        }
    }

    async fn connect(&mut self, buf: &[u8]) -> error::Result<()> {
        log::info!("connect()");
        let mut offset = 0;
        let packet = ConnectPacket::from_net(&buf, &mut offset)?;
        self.client_id = packet.client_id().to_string();
        // TODO(Shaohua): Check connection status first.
        // TODO(Shaohua): If this client is already connected, send disconnect packet.
        let packet = ConnectAckPacket::new(true, ConnectReturnCode::Accepted);
        self.status = Status::Connected;
        self.send(packet).await.map(|_size| ())
    }

    async fn ping(&mut self, buf: &[u8]) -> error::Result<()> {
        log::info!("ping()");
        let mut offset = 0;
        let _packet = PingRequestPacket::from_net(&buf, &mut offset)?;
        log::info!("Will send ping response packet");
        let ping_resp_packet = PingResponsePacket::new();
        self.send(ping_resp_packet).await
    }

    async fn publish(&mut self, buf: &[u8]) -> error::Result<()> {
        log::info!("publish()");
        let mut offset: usize = 0;
        let packet = PublishPacket::from_net(&buf, &mut offset)?;
        let publish_ack_packet = PublishAckPacket::new(packet.packet_id());
        self.send(publish_ack_packet).await?;
        // TODO(Shaohua): Send PublishAck if qos == 0
        self.sender
            .send(ConnectionCommand::Publish(packet))
            .await
            .map(drop)
            .map_err(|_err| error::Error::SendError)
    }

    async fn subscribe(&mut self, buf: &[u8]) -> error::Result<()> {
        log::info!("subscribe()");
        let mut offset: usize = 0;
        let packet = SubscribePacket::from_net(&buf, &mut offset)?;
        let ack;
        if let Err(err) = self
            .sender
            .send(ConnectionCommand::Subscribe(
                self.connection_id,
                packet.clone(),
            ))
            .await
        {
            ack = SubscribeAck::Failed;
            log::warn!("Failed to send subscribe command to server: {:?}", err);
        } else {
            // TODO(Shaohua): Handle all of topics.
            ack = SubscribeAck::QoS(packet.topics()[0].qos);
        }
        let subscribe_ack_packet = SubscribeAckPacket::new(ack, packet.packet_id());
        self.send(subscribe_ack_packet).await
    }

    async fn unsubscribe(&mut self, buf: &[u8]) -> error::Result<()> {
        log::info!("unsubscribe()");
        let mut offset: usize = 0;
        let packet = UnsubscribePacket::from_net(&buf, &mut offset)?;
        // TODO(Shaohua): Send msg to command channel
        let unsubscribe_ack_packet = UnsubscribeAckPacket::new(packet.packet_id());
        self.send(unsubscribe_ack_packet).await
    }

    async fn disconnect(&mut self, _buf: &[u8]) -> error::Result<()> {
        log::info!("disconnect");
        self.status = Status::Disconnected;
        // TODO(Shaohua): Send disconnect to server to unsubscribe any topics.
        Ok(())
    }

    async fn cmd_router(&mut self, cmd: ServerCommand) -> error::Result<()> {
        match cmd {
            ServerCommand::Publish(packet) => self.server_publish(packet).await?,
        }
        Ok(())
    }

    async fn server_publish(&mut self, packet: PublishPacket) -> error::Result<()> {
        log::info!("server publish: {:?}", packet);
        self.send(packet).await
    }
}
