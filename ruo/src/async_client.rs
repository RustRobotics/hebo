// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::*;
use super::connect_ack_packet::{ConnectAckPacket, ConnectReturnCode};
use super::connect_options::*;
use super::connect_packet::ConnectPacket;
use super::disconnect_packet::DisconnectPacket;
use super::ping_request_packet::PingRequestPacket;
use super::publish_ack_packet::PublishAckPacket;
use super::publish_packet::PublishPacket;
use super::subscribe_ack_packet::SubscribeAckPacket;
use super::subscribe_packet::SubscribePacket;
use super::unsubscribe_ack_packet::UnsubscribeAckPacket;
use super::unsubscribe_packet::UnsubscribePacket;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::interval;

#[derive(Debug, Hash, PartialEq)]
enum StreamStatus {
    Invalid,
    Connecting,
    Connected,
    ConnectFailed,
    Disconnecting,
    Disconnected,
}

#[derive(Debug)]
pub struct AsyncClient {
    connect_options: ConnectOptions,
    socket: TcpStream,
    status: StreamStatus,
    topics: HashMap<String, PacketId>,
    packet_id: PacketId,
    subscribing_packets: HashMap<PacketId, SubscribePacket>,
    unsubscribing_packets: HashMap<PacketId, UnsubscribePacket>,
    publishing_qos1_packets: HashMap<PacketId, PublishPacket>,
    publishing_qos2_packets: HashMap<PacketId, PublishPacket>,
}

impl AsyncClient {
    pub async fn new(connect_options: ConnectOptions) -> AsyncClient {
        let socket = TcpStream::connect(connect_options.address()).await.unwrap();
        let mut client = AsyncClient {
            connect_options,
            socket: socket,
            status: StreamStatus::Connecting,
            topics: HashMap::new(),
            packet_id: 1,
            subscribing_packets: HashMap::new(),
            unsubscribing_packets: HashMap::new(),
            publishing_qos1_packets: HashMap::new(),
            publishing_qos2_packets: HashMap::new(),
        };

        let conn_packet = ConnectPacket::new();
        client.send(conn_packet).await;
        log::info!("send conn packet");

        client
    }

    pub async fn start(&mut self) {
        log::info!("client.start()");

        let mut buf: Vec<u8> = Vec::with_capacity(1024);
        log::info!("reader loop");
        // FIXME(Shaohua): Fix panic when keep_alive is 0
        let mut timer = interval(*self.connect_options.keep_alive());

        loop {
            tokio::select! {
                Ok(n_recv) = self.socket.read_buf(&mut buf) => {
                    if n_recv > 0 {
                        self.recv_router(&mut buf).await;
                        buf.clear();
                    }
                }
                _ = timer.tick() => {
                    log::info!("tick()");
                    self.ping().await;
                },
            }
        }
    }

    async fn recv_router(&mut self, buf: &mut Vec<u8>) {
        match FixedHeader::from_net(&buf) {
            Ok(fixed_header) => {
                //log::info!("fixed header: {:?}", fixed_header);
                match fixed_header.packet_type {
                    PacketType::ConnectAck => self.connect_ack(&buf).await,
                    PacketType::Publish => self.on_message(&buf).await,
                    PacketType::PublishAck => self.publish_ack(&buf),
                    PacketType::SubscribeAck => self.subscribe_ack(&buf),
                    PacketType::UnsubscribeAck => self.unsubscribe_ack(&buf),
                    PacketType::PingResp => self.on_ping_resp().await,
                    t => log::info!("Unhandled msg: {:?}", t),
                }
            }
            Err(err) => log::warn!("err: {:?}", err),
        }
    }

    async fn send<P: ToNetPacket>(&mut self, packet: P) {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        self.socket.write_all(&buf).await.unwrap();
    }

    pub async fn publish(&mut self, topic: &str, qos: QoS, data: &[u8]) {
        log::info!("Send publish packet");
        let mut packet = PublishPacket::new(topic, qos, data);
        match qos {
            QoS::AtLeastOnce => {
                let packet_id = self.next_packet_id();
                packet.set_packet_id(packet_id);
                // TODO(Shaohua): Tuning memory usage.
                self.publishing_qos1_packets
                    .insert(packet_id, packet.clone());
            }
            QoS::ExactOnce => {
                let packet_id = self.next_packet_id();
                packet.set_packet_id(packet_id);
                self.publishing_qos2_packets
                    .insert(packet_id, packet.clone());
            }
            _ => (),
        }
        self.send(packet).await;
    }

    pub async fn subscribe(&mut self, topic: &str, qos: QoS) {
        log::info!("subscribe to: {}", topic);
        let packet_id = self.next_packet_id();
        self.topics.insert(topic.to_string(), packet_id);
        let packet = SubscribePacket::new(topic, qos, packet_id);
        self.subscribing_packets.insert(packet_id, packet.clone());
        self.send(packet).await;
    }

    pub async fn unsubscribe(&mut self, topics: &[&str]) {
        log::info!("unsubscribe to: {:?}", topics);
        let packet_id = self.next_packet_id();
        let packet = UnsubscribePacket::new(topics, packet_id);
        self.unsubscribing_packets.insert(packet_id, packet.clone());
        self.send(packet).await;
    }

    pub async fn disconnect(&mut self) {
        if self.status == StreamStatus::Connected {
            self.status = StreamStatus::Disconnecting;
            let packet = DisconnectPacket::new();
            self.send(packet).await;
        }
    }

    async fn on_connect(&mut self) {
        log::info!("On connect()");
        self.subscribe("hello", QoS::AtMostOnce).await;
        self.publish("hello", QoS::AtMostOnce, b"Hello, world")
            .await;
        self.subscribe("hello2", QoS::AtLeastOnce).await;
        self.publish("hello2", QoS::AtLeastOnce, b"Hello, qos1")
            .await;
    }

    async fn ping(&mut self) {
        log::info!("ping()");
        if self.status == StreamStatus::Connected {
            log::info!("Send ping packet");
            let packet = PingRequestPacket::new();
            self.send(packet).await;
        }
    }

    fn on_disconnect(&mut self) {
        self.status = StreamStatus::Disconnected;
    }

    async fn on_message(&self, buf: &Vec<u8>) {
        log::info!("on_message()");
        match PublishPacket::from_net(buf) {
            Ok(packet) => {
                log::info!("packet: {:?}", packet);
                log::info!("message: {:?}", std::str::from_utf8(packet.message()));
            }
            Err(err) => log::warn!("Failed to parse publish msg: {:?}", err),
        }
    }

    async fn on_ping_resp(&self) {
        log::info!("on ping resp");
        // TODO(Shaohua): Reset reconnect timer.
    }

    async fn connect_ack(&mut self, buf: &[u8]) {
        log::info!("connect_ack()");
        match ConnectAckPacket::from_net(&buf) {
            Ok(packet) => match packet.return_code() {
                ConnectReturnCode::Accepted => {
                    self.status = StreamStatus::Connected;
                    self.on_connect().await;
                }
                _ => {
                    log::warn!("Failed to connect to server, {:?}", packet.return_code());
                    self.status = StreamStatus::ConnectFailed;
                }
            },
            Err(err) => log::error!("Invalid ConnectAckPacket: {:?}", buf),
        }
    }

    fn publish_ack(&mut self, buf: &[u8]) {
        log::info!("publish_ack()");
        match PublishAckPacket::from_net(&buf) {
            Ok(packet) => {
                let packet_id = packet.packet_id();
                if let Some(p) = self.publishing_qos1_packets.get(&packet_id) {
                    log::info!("Topic `{}` publish confirmed!", p.topic());
                    self.publishing_qos1_packets.remove(&packet.packet_id());
                } else {
                    log::warn!("Failed to find PublishAckPacket: {}", packet_id);
                }
            }
            Err(err) => log::error!("Invalid PublishAckPacket: {:?}", buf),
        }
    }

    fn subscribe_ack(&mut self, buf: &[u8]) {
        log::info!("subscribe_ack()");
        // Parse packet_id and remove from vector.
        match SubscribeAckPacket::from_net(&buf) {
            Ok(packet) => {
                let packet_id = packet.packet_id();
                if let Some(p) = self.subscribing_packets.get(&packet_id) {
                    if packet.failed() {
                        log::warn!("Failed to subscribe: {}", p.topic());
                    }
                    log::info!("Topic `{}` subscription confirmed!", p.topic());
                    self.subscribing_packets.remove(&packet.packet_id());
                // TODO(Shaohua): Check qos value.
                } else {
                    log::warn!("Failed to find SubscribeAckPacket: {}", packet_id);
                }
            }
            Err(err) => log::error!("Invalid SubscribeAckPacket: {:?}", buf),
        }
    }

    fn unsubscribe_ack(&mut self, buf: &[u8]) {
        log::info!("unsubscribe_ack()");
        match UnsubscribeAckPacket::from_net(&buf) {
            Ok(packet) => {
                let packet_id = packet.packet_id();
                if let Some(p) = self.unsubscribing_packets.get(&packet_id) {
                    log::info!("Topics `{:?}` unsubscription confirmed!", p.topics());
                    self.unsubscribing_packets.remove(&packet.packet_id());
                } else {
                    log::warn!("Failed to find UnsubscribeAckPacket: {}", packet_id);
                }
            }
            Err(err) => log::error!("Invalid UnsubscribeAckPacket: {:?}", buf),
        }
    }

    fn next_packet_id(&mut self) -> PacketId {
        if self.packet_id == u16::MAX {
            self.packet_id = 1;
        } else {
            self.packet_id += 1;
        }
        self.packet_id
    }
}
