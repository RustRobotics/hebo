// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{
    ByteArray, ConnectAckPacket, ConnectPacket, ConnectReturnCode, DecodePacket, DisconnectPacket,
    EncodePacket, FixedHeader, PacketId, PacketType, PingRequestPacket, PublishAckPacket,
    PublishPacket, QoS, SubscribeAckPacket, SubscribePacket, UnsubscribeAckPacket,
    UnsubscribePacket,
};
use std::collections::HashMap;
use std::fmt;

use crate::connect_options::*;
use crate::error::Error;
use crate::sync_stream::Stream;

#[derive(Debug, Hash, PartialEq)]
enum StreamStatus {
    Connecting,
    Connected,
    ConnectFailed,
    Disconnecting,
    Disconnected,
}

type ConnectCallback = fn(&mut Client);
type MessageCallback = fn(&mut Client, &PublishPacket);

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
    on_connect_cb: Option<ConnectCallback>,
    on_message_cb: Option<MessageCallback>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("connect_options", &self.connect_options)
            .field("stream", &"TcpStream")
            .field("status", &self.status)
            .field("packet_id", &self.packet_id)
            .finish()
    }
}

impl Client {
    pub fn new(
        connect_options: ConnectOptions,
        on_connect_cb: Option<ConnectCallback>,
        on_message_cb: Option<MessageCallback>,
    ) -> Result<Client, Error> {
        let stream = Stream::new(connect_options.connect_type())?;
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
            on_connect_cb,
            on_message_cb,
        };

        Ok(client)
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let conn_packet = ConnectPacket::new(self.connect_options.client_id());
        println!("connect packet client id: {}", conn_packet.client_id());
        self.send(conn_packet)?;
        let mut buf = Vec::with_capacity(1024);

        loop {
            buf.resize(buf.capacity(), 0);
            if let Ok(n_recv) = self.stream.read_buf(&mut buf) {
                if n_recv > 0 {
                    if let Err(err) = self.handle_session_packet(&mut buf) {
                        log::error!("err: {:?}", err);
                    }
                    buf.clear();
                } else if n_recv == 0 {
                    log::warn!("n_recv is 0");
                    break;
                }
            }
        }

        Ok(())
    }

    fn handle_session_packet(&mut self, buf: &mut Vec<u8>) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let fixed_header = FixedHeader::decode(&mut ba)?;
        log::info!("fixed header: {:?}", fixed_header);
        match fixed_header.packet_type {
            PacketType::ConnectAck => self.on_connect_ack(&buf),
            PacketType::Publish { .. } => self.on_message(&buf),
            PacketType::PublishAck => self.on_publish_ack(&buf),
            PacketType::SubscribeAck => self.on_subscribe_ack(&buf),
            PacketType::UnsubscribeAck => self.on_unsubscribe_ack(&buf),
            PacketType::PingResponse => self.on_ping_resp(),
            t => {
                log::info!("Unhandled msg: {:?}", t);
                Ok(())
            }
        }
    }

    fn send<P: EncodePacket>(&mut self, packet: P) -> Result<(), Error> {
        let mut buf = Vec::new();
        packet.encode(&mut buf)?;
        self.stream.write_all(&buf).map_err(|err| err.into())
    }

    pub fn publish(&mut self, topic: &str, qos: QoS, data: &[u8]) -> Result<(), Error> {
        let mut packet = PublishPacket::new(topic, qos, data)?;
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
        self.send(packet)
    }

    pub fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Error> {
        log::info!("subscribe to: {}", topic);
        let packet_id = self.next_packet_id();
        self.topics.insert(topic.to_string(), packet_id);
        let packet = SubscribePacket::new(topic, qos, packet_id)?;
        self.subscribing_packets.insert(packet_id, packet.clone());
        self.send(packet)
    }

    pub fn unsubscribe(&mut self, topic: &str) -> Result<(), Error> {
        let packet_id = self.next_packet_id();
        let packet = UnsubscribePacket::new(topic, packet_id);
        self.unsubscribing_packets.insert(packet_id, packet.clone());
        self.send(packet)
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        if self.status == StreamStatus::Connected {
            self.status = StreamStatus::Disconnecting;
            let packet = DisconnectPacket::new();
            self.send(packet)?;
        }
        self.on_disconnect();
        Ok(())
    }

    fn on_connect(&mut self) {
        if let Some(cb) = &self.on_connect_cb {
            cb(self);
        }
    }

    fn ping(&mut self) -> Result<(), Error> {
        log::info!("ping()");
        if self.status == StreamStatus::Connected {
            log::info!("Send ping packet");
            let packet = PingRequestPacket::new();
            self.send(packet)
        } else {
            // TODO(Shaohua): Return Error
            Ok(())
        }
    }

    fn on_disconnect(&mut self) {
        self.status = StreamStatus::Disconnected;
    }

    fn on_message(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("on_message()");
        let mut ba = ByteArray::new(buf);
        let packet = PublishPacket::decode(&mut ba)?;
        log::info!("packet: {:?}", packet);
        if let Some(cb) = &self.on_message_cb {
            cb(self, &packet);
        }
        Ok(())
    }

    fn on_ping_resp(&self) -> Result<(), Error> {
        log::info!("on ping resp");
        // TODO(Shaohua): Reset reconnect timer.
        Ok(())
    }

    fn on_connect_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("connect_ack()");
        let mut ba = ByteArray::new(buf);
        let packet = ConnectAckPacket::decode(&mut ba)?;
        match packet.return_code() {
            ConnectReturnCode::Accepted => {
                self.status = StreamStatus::Connected;
                self.on_connect();
            }
            _ => {
                log::warn!("Failed to connect to server, {:?}", packet.return_code());
                self.status = StreamStatus::ConnectFailed;
            }
        }
        Ok(())
    }

    fn on_publish_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("publish_ack()");
        let mut ba = ByteArray::new(buf);
        let packet = PublishAckPacket::decode(&mut ba)?;
        let packet_id = packet.packet_id();
        if let Some(p) = self.publishing_qos1_packets.get(&packet_id) {
            log::info!("Topic `{}` publish confirmed!", p.topic());
            self.publishing_qos1_packets.remove(&packet.packet_id());
        } else {
            log::warn!("Failed to find PublishAckPacket: {}", packet_id);
        }
        Ok(())
    }

    fn on_subscribe_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("subscribe_ack()");
        // Parse packet_id and remove from vector.
        let mut ba = ByteArray::new(buf);
        let packet = SubscribeAckPacket::decode(&mut ba)?;
        let packet_id = packet.packet_id();
        if let Some(p) = self.subscribing_packets.get(&packet_id) {
            log::info!("Subscription {:?} confirmed!", p.topics());
            self.subscribing_packets.remove(&packet.packet_id());
        } else {
            log::warn!("Failed to find SubscribeAckPacket: {}", packet_id);
        }
        Ok(())
    }

    fn on_unsubscribe_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("unsubscribe_ack()");
        let mut ba = ByteArray::new(buf);
        let packet = UnsubscribeAckPacket::decode(&mut ba)?;
        let packet_id = packet.packet_id();
        // TODO(Shaohua): Tuning
        if let Some(_p) = self.unsubscribing_packets.get(&packet_id) {
            self.unsubscribing_packets.remove(&packet.packet_id());
        } else {
            log::warn!("Failed to find UnsubscribeAckPacket: {}", packet_id);
        }
        Ok(())
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
