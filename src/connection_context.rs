// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::commands::{ConnectionCommand, ServerCommand};
use ruo::base::{FixedHeader, FromNetPacket, PacketType, ToNetPacket};
use ruo::connect_ack_packet::{ConnectAckPacket, ConnectReturnCode};
use ruo::connect_packet::ConnectPacket;
use ruo::ping_request_packet::PingRequestPacket;
use ruo::ping_response_packet::PingResponsePacket;
use ruo::publish_ack_packet::PublishAckPacket;
use ruo::publish_packet::PublishPacket;
use ruo::subscribe_ack_packet::SubscribeAckPacket;
use ruo::subscribe_packet::SubscribePacket;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

#[derive(Debug)]
enum Status {
    Invalid,
    Connected,
    Disconnecting,
    Disconnected,
}

#[derive(Debug)]
pub struct ConnectionContext {
    stream: TcpStream,
    remote_address: SocketAddr,
    sender: Sender<ConnectionCommand>,
    receiver: Receiver<ServerCommand>,
    status: Status,
    client_id: String,
}

impl ConnectionContext {
    pub fn new(
        stream: TcpStream,
        remote_address: SocketAddr,
        sender: Sender<ConnectionCommand>,
        receiver: Receiver<ServerCommand>,
    ) -> ConnectionContext {
        ConnectionContext {
            remote_address,
            stream,
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
                        self.recv_router(&buf).await;
                        buf.clear();
                    }
                }
                _ = timer.tick() => {
                    log::info!("tick()");
                },
            }
        }
    }

    async fn send<P: ToNetPacket>(&mut self, packet: P) {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        self.stream.write(&buf).await.unwrap();
    }

    async fn recv_router(&mut self, buf: &[u8]) {
        let mut offset: usize = 0;
        match FixedHeader::from_net(&buf, &mut offset) {
            Ok(fixed_header) => {
                //log::info!("fixed header: {:?}", fixed_header);
                match fixed_header.packet_type {
                    PacketType::Connect => self.connect(&buf).await,
                    PacketType::PingRequest => self.ping(&buf).await,
                    PacketType::Publish => self.publish(&buf).await,
                    PacketType::Subscribe => self.subscribe(&buf).await,
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
                let packet = ConnectAckPacket::new(ConnectReturnCode::Accepted, true);
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
            }
            Err(err) => log::warn!("Failed to parse publish packet: {:?}, {:?}", err, buf),
        }
    }

    async fn subscribe(&mut self, buf: &[u8]) {
        log::info!("subscribe()");
        let mut offset: usize = 0;
        match SubscribePacket::from_net(&buf, &mut offset) {
            Ok(packet) => {
                let failed = false;
                let subscribe_ack_packet =
                    SubscribeAckPacket::new(packet.qos(), failed, packet.packet_id());
                self.send(subscribe_ack_packet).await;
            }
            Err(err) => log::warn!("Failed to parse subscribe packet: {:?}, {:?}", err, buf),
        }
    }
}
