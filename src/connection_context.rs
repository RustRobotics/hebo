// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::net::SocketAddr;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

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

use crate::commands::{ConnectionCommand, ConnectionId, ServerCommand};

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
// TODO(Shaohua): Hanle Will Message
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
                        self.handle_client_packet(&buf).await;
                        buf.clear();
                    }
                }
                _ = timer.tick() => {
                    log::info!("tick()");
                },
                Some(cmd) = self.receiver.recv() => {
                    self.cmd_router(cmd).await;
                },
            }
        }
    }

    async fn send<P: ToNetPacket>(&mut self, packet: P) {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        self.stream.write(&buf).await.unwrap();
    }

    async fn handle_client_packet(&mut self, buf: &[u8]) {
        let mut offset: usize = 0;
        match FixedHeader::from_net(&buf, &mut offset) {
            Ok(fixed_header) => {
                //log::info!("fixed header: {:?}", fixed_header);
                match fixed_header.packet_type {
                    PacketType::Connect => self.connect(&buf).await,
                    PacketType::PingRequest => self.ping(&buf).await,
                    PacketType::Publish => self.publish(&buf).await,
                    PacketType::Subscribe => self.subscribe(&buf).await,
                    PacketType::Unsubscribe => self.unsubscribe(&buf).await,
                    PacketType::Disconnect => self.disconnect(&buf).await,
                    t => log::warn!("Unhandled msg: {:?}", t),
                }
            }
            Err(err) => log::warn!("err: {:?}", err),
        }
    }

    async fn connect(&mut self, buf: &[u8]) {
        log::info!("connect()");
        let mut offset = 0;
        match ConnectPacket::from_net(&buf, &mut offset) {
            Ok(packet) => {
                self.client_id = packet.client_id().to_string();
                // TODO(Shaohua): Check connection status first.
                // TODO(Shaohua): If this client is already connected, send disconnect packet.
                let packet = ConnectAckPacket::new(true, ConnectReturnCode::Accepted);
                self.send(packet).await;
                self.status = Status::Connected;
            }
            Err(err) => log::warn!("Failed to parse connect packet: {:?}, {:?}", err, buf),
        }
    }

    async fn ping(&mut self, buf: &[u8]) {
        log::info!("ping()");
        let mut offset = 0;
        match PingRequestPacket::from_net(&buf, &mut offset) {
            Ok(_packet) => {
                log::info!("Will send ping response packet");
                let ping_resp_packet = PingResponsePacket::new();
                self.send(ping_resp_packet).await;
            }
            Err(err) => log::warn!("Failed to parse ping packet: {:?}, {:?}", err, buf),
        }
    }

    async fn publish(&mut self, buf: &[u8]) {
        log::info!("publish()");
        let mut offset: usize = 0;
        match PublishPacket::from_net(&buf, &mut offset) {
            Ok(packet) => {
                let publish_ack_packet = PublishAckPacket::new(packet.packet_id());
                self.send(publish_ack_packet).await;
                // TODO(Shaohua): Send PublishAck if qos == 0
                if let Err(err) = self.sender.send(ConnectionCommand::Publish(packet)).await {
                    log::warn!("pubish() failed to send packet to server, {:?}", err);
                }
            }
            Err(err) => log::warn!("Failed to parse publish packet: {:?}, {:?}", err, buf),
        }
    }

    async fn subscribe(&mut self, buf: &[u8]) {
        log::info!("subscribe()");
        let mut offset: usize = 0;
        match SubscribePacket::from_net(&buf, &mut offset) {
            Ok(packet) => {
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
                    ack = SubscribeAck::QoS(packet.topics()[0].qos);
                }
                let subscribe_ack_packet = SubscribeAckPacket::new(ack, packet.packet_id());
                self.send(subscribe_ack_packet).await;
            }
            Err(err) => log::warn!("Failed to parse subscribe packet: {:?}, {:?}", err, buf),
        }
    }

    async fn unsubscribe(&mut self, buf: &[u8]) {
        log::info!("unsubscribe()");
        let mut offset: usize = 0;
        match UnsubscribePacket::from_net(&buf, &mut offset) {
            Ok(packet) => {
                // TODO(Shaohua): Send msg to command channel
                let unsubscribe_ack_packet = UnsubscribeAckPacket::new(packet.packet_id());
                self.send(unsubscribe_ack_packet).await;
            }
            Err(err) => log::warn!("Failed to parse subscribe packet: {:?}, {:?}", err, buf),
        }
    }

    async fn disconnect(&mut self, _buf: &[u8]) {
        log::info!("disconnect");
        self.status = Status::Disconnected;
        // TODO(Shaohua): Send disconnect to server to unsubscribe any topics.
    }

    async fn cmd_router(&mut self, cmd: ServerCommand) {
        match cmd {
            ServerCommand::Publish(packet) => {
                self.server_publish(packet).await;
            }
        }
    }

    async fn server_publish(&mut self, packet: PublishPacket) {
        log::info!("server publish: {:?}", packet);
        self.send(packet).await;
    }
}
