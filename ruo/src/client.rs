// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use codec::base::*;
use codec::connect_ack_packet::{ConnectAckPacket, ConnectReturnCode};
use codec::connect_packet::ConnectPacket;
use codec::disconnect_packet::DisconnectPacket;
use codec::ping_request_packet::PingRequestPacket;
use codec::publish_ack_packet::PublishAckPacket;
use codec::publish_packet::PublishPacket;
use codec::subscribe_ack_packet::SubscribeAckPacket;
use codec::subscribe_packet::SubscribePacket;
use codec::unsubscribe_ack_packet::UnsubscribeAckPacket;
use codec::unsubscribe_packet::UnsubscribePacket;
use std::collections::HashMap;

use crate::connect_options::*;
use crate::stream::Stream;

#[derive(Debug, Hash, PartialEq)]
enum StreamStatus {
    Connecting,
    Connected,
    ConnectFailed,
    Disconnecting,
    Disconnected,
}

pub struct Client {
    connect_options: ConnectOptions,
    stream: Stream,
    status: StreamStatus,
    topics: HashMap<String, PacketId>,
    packet_id: PacketId,
    subscribing_packets: HashMap<PacketId, SubscribePacket>,
    unsubscribing_packets: HashMap<PacketId, UnsubscribePacket>,
    publishing_qos1_packets: HashMap<PacketId, PublishPacket>,
    publishing_qos2_packets: HashMap<PacketId, PublishPacket>,
    connect_cb: Option<fn(&mut Self)>,
}

impl Client {
    pub fn new(connect_options: ConnectOptions, connect_cb: Option<fn(&mut Self)>) -> Client {
        let stream =
            Stream::new(connect_options.address(), connect_options.connect_type()).unwrap();
        let client = Client {
            connect_options,
            stream,
            status: StreamStatus::Connecting,
            topics: HashMap::new(),
            packet_id: 1,
            subscribing_packets: HashMap::new(),
            unsubscribing_packets: HashMap::new(),
            publishing_qos1_packets: HashMap::new(),
            publishing_qos2_packets: HashMap::new(),
            connect_cb,
        };

        client
    }

    pub fn start(&mut self) {
        log::info!("client.start()");

        let mut buf: Vec<u8> = vec![0; 1024];
        log::info!("reader loop");

        let conn_packet = ConnectPacket::new(self.connect_options.client_id());
        println!("connect packet client id: {}", conn_packet.client_id());
        self.send(conn_packet);
        log::info!("send conn packet");

        loop {
            if let Ok(n_recv) = self.stream.read_buf(&mut buf) {
                if n_recv > 0 {
                    self.recv_router(&mut buf);
                    buf.clear();
                }
            }
        }
    }

    fn recv_router(&mut self, buf: &mut Vec<u8>) {
        let mut offset = 0;
        match FixedHeader::from_net(&buf, &mut offset) {
            Ok(fixed_header) => {
                //log::info!("fixed header: {:?}", fixed_header);
                match fixed_header.packet_type {
                    PacketType::ConnectAck => self.connect_ack(&buf),
                    PacketType::Publish => self.on_message(&buf),
                    PacketType::PublishAck => self.publish_ack(&buf),
                    PacketType::SubscribeAck => self.subscribe_ack(&buf),
                    PacketType::UnsubscribeAck => self.unsubscribe_ack(&buf),
                    PacketType::PingResponse => self.on_ping_resp(),
                    t => log::info!("Unhandled msg: {:?}", t),
                }
            }
            Err(err) => log::warn!("err: {:?}", err),
        }
    }

    fn send<P: ToNetPacket>(&mut self, packet: P) {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        self.stream.write_all(&buf).unwrap();
    }

    pub fn publish(&mut self, topic: &str, qos: QoS, data: &[u8]) {
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
        self.send(packet);
    }

    pub fn subscribe(&mut self, topic: &str, qos: QoS) {
        log::info!("subscribe to: {}", topic);
        let packet_id = self.next_packet_id();
        self.topics.insert(topic.to_string(), packet_id);
        let packet = SubscribePacket::new(topic, qos, packet_id);
        self.subscribing_packets.insert(packet_id, packet.clone());
        self.send(packet);
    }

    pub fn unsubscribe(&mut self, topic: &str) {
        log::info!("unsubscribe to: {:?}", topic);
        let packet_id = self.next_packet_id();
        let packet = UnsubscribePacket::new(topic, packet_id);
        self.unsubscribing_packets.insert(packet_id, packet.clone());
        self.send(packet);
    }

    pub fn disconnect(&mut self) {
        if self.status == StreamStatus::Connected {
            self.status = StreamStatus::Disconnecting;
            let packet = DisconnectPacket::new();
            self.send(packet);
        }
        self.on_disconnect();
    }

    fn on_connect(&mut self) {
        log::info!("On connect()");
        if let Some(cb) = &self.connect_cb {
            cb(self);
        }
    }

    fn ping(&mut self) {
        log::info!("ping()");
        if self.status == StreamStatus::Connected {
            log::info!("Send ping packet");
            let packet = PingRequestPacket::new();
            self.send(packet);
        }
    }

    fn on_disconnect(&mut self) {
        self.status = StreamStatus::Disconnected;
    }

    fn on_message(&self, buf: &[u8]) {
        log::info!("on_message()");
        let mut offset = 0;
        match PublishPacket::from_net(buf, &mut offset) {
            Ok(packet) => {
                log::info!("packet: {:?}", packet);
                log::info!("message: {:?}", std::str::from_utf8(packet.message()));
            }
            Err(err) => log::warn!("Failed to parse publish msg: {:?}", err),
        }
    }

    fn on_ping_resp(&self) {
        log::info!("on ping resp");
        // TODO(Shaohua): Reset reconnect timer.
    }

    fn connect_ack(&mut self, buf: &[u8]) {
        log::info!("connect_ack()");
        let mut offset = 0;
        match ConnectAckPacket::from_net(&buf, &mut offset) {
            Ok(packet) => match packet.return_code() {
                ConnectReturnCode::Accepted => {
                    self.status = StreamStatus::Connected;
                    self.on_connect();
                }
                _ => {
                    log::warn!("Failed to connect to server, {:?}", packet.return_code());
                    self.status = StreamStatus::ConnectFailed;
                }
            },
            Err(err) => log::error!("Invalid ConnectAckPacket: {:?}, {:?}", buf, err),
        }
    }

    fn publish_ack(&mut self, buf: &[u8]) {
        log::info!("publish_ack()");
        let mut offset = 0;
        match PublishAckPacket::from_net(&buf, &mut offset) {
            Ok(packet) => {
                let packet_id = packet.packet_id();
                if let Some(p) = self.publishing_qos1_packets.get(&packet_id) {
                    log::info!("Topic `{}` publish confirmed!", p.topic());
                    self.publishing_qos1_packets.remove(&packet.packet_id());
                } else {
                    log::warn!("Failed to find PublishAckPacket: {}", packet_id);
                }
            }
            Err(err) => log::error!("Invalid PublishAckPacket: {:?}, {:?}", buf, err),
        }
    }

    fn subscribe_ack(&mut self, buf: &[u8]) {
        log::info!("subscribe_ack()");
        // Parse packet_id and remove from vector.
        let mut offset = 0;
        match SubscribeAckPacket::from_net(&buf, &mut offset) {
            Ok(packet) => {
                let packet_id = packet.packet_id();
                if let Some(p) = self.subscribing_packets.get(&packet_id) {
                    log::info!("Subscription {:?} confirmed!", p.topics());
                    self.subscribing_packets.remove(&packet.packet_id());
                } else {
                    log::warn!("Failed to find SubscribeAckPacket: {}", packet_id);
                }
            }
            Err(err) => log::error!("Invalid SubscribeAckPacket: {:?}, {:?}", buf, err),
        }
    }

    fn unsubscribe_ack(&mut self, buf: &[u8]) {
        log::info!("unsubscribe_ack()");
        let mut offset = 0;
        match UnsubscribeAckPacket::from_net(&buf, &mut offset) {
            Ok(packet) => {
                let packet_id = packet.packet_id();
                if let Some(p) = self.unsubscribing_packets.get(&packet_id) {
                    log::info!("Topics `{:?}` unsubscribe confirmed!", p.topics());
                    self.unsubscribing_packets.remove(&packet.packet_id());
                } else {
                    log::warn!("Failed to find UnsubscribeAckPacket: {}", packet_id);
                }
            }
            Err(err) => log::error!("Invalid UnsubscribeAckPacket: {:?}, {:?}", buf, err),
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

    pub fn connect_option(&self) -> &ConnectOptions {
        return &self.connect_options;
    }
}
