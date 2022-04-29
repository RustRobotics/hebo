// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::v3::{
    ConnectAckPacket, ConnectPacket, ConnectReturnCode, DisconnectPacket, FixedHeader, PacketType,
    PingRequestPacket, PublishAckPacket, PublishPacket, SubscribeAckPacket, SubscribePacket,
    UnsubscribeAckPacket, UnsubscribePacket,
};
use codec::{ByteArray, DecodePacket, EncodePacket, PacketId, QoS};
use std::collections::HashMap;

use super::{ClientStatus, PublishMessage, Stream};
use crate::connect_options::*;
use crate::error::{Error, ErrorKind};

/// MQTT Client for V3.1.
pub struct ClientInnerV3 {
    connect_options: ConnectOptions,
    status: ClientStatus,

    stream: Option<Stream>,
    topics: HashMap<String, PacketId>,
    packet_id: PacketId,
    subscribing_packets: HashMap<PacketId, SubscribePacket>,
    unsubscribing_packets: HashMap<PacketId, UnsubscribePacket>,
    publishing_qos1_packets: HashMap<PacketId, PublishPacket>,
    publishing_qos2_packets: HashMap<PacketId, PublishPacket>,

    buffer: Vec<u8>,
}

impl ClientInnerV3 {
    /// Create a new client object.
    ///
    /// No socket is connect to server yet.
    pub fn new(connect_options: ConnectOptions) -> Self {
        Self {
            connect_options,
            status: ClientStatus::Disconnected,

            stream: None,
            topics: HashMap::new(),
            packet_id: PacketId::new(1),
            subscribing_packets: HashMap::new(),
            unsubscribing_packets: HashMap::new(),
            publishing_qos1_packets: HashMap::new(),
            publishing_qos2_packets: HashMap::new(),

            // TODO(Shaohua): Support large packets.
            buffer: Vec::with_capacity(1024),
        }
    }

    /// Get connection options.
    pub fn connect_options(&self) -> &ConnectOptions {
        &self.connect_options
    }

    /// Get current connection status.
    pub fn status(&self) -> ClientStatus {
        self.status
    }

    /// Connct to server.
    ///
    /// Returns Ok() if success.
    pub fn connect(&mut self) -> Result<bool, Error> {
        // TODO(Shaohua): Do not return bool, return errors instead.
        assert_eq!(self.status, ClientStatus::Disconnected);
        let stream = Stream::new(self.connect_options.connect_type())?;
        self.stream = Some(stream);
        let conn_packet = ConnectPacket::new(&self.connect_options.client_id())?;
        self.status = ClientStatus::Connecting;
        self.send_packet(conn_packet)?;
        loop {
            let n_recv = self.read_stream()?;
            if n_recv > 0 {
                break;
            }
        }

        let mut ba = ByteArray::new(&mut self.buffer);
        let fixed_header = FixedHeader::decode(&mut ba)?;
        match fixed_header.packet_type() {
            PacketType::ConnectAck => {
                ba.reset_offset();
                let packet = ConnectAckPacket::decode(&mut ba)?;
                match packet.return_code() {
                    ConnectReturnCode::Accepted => {
                        self.status = ClientStatus::Connected;
                        return Ok(true);
                    }
                    _ => {
                        log::warn!("Failed to connect to server, {:?}", packet.return_code());
                        return Ok(false);
                    }
                }
            }

            t => {
                return Err(Error::from_string(
                    ErrorKind::PacketError,
                    format!("Expected connect packet, got: {:?}", t),
                ));
            }
        }
    }

    /// Publish message to server.
    pub fn publish(&mut self, topic: &str, qos: QoS, data: &[u8]) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        let mut packet = PublishPacket::new(topic, qos, data)?;
        let packet_id = self.next_packet_id();
        packet.set_packet_id(packet_id);
        match packet.qos() {
            QoS::AtLeastOnce => {
                // TODO(Shaohua): Tuning memory usage.
                self.publishing_qos1_packets
                    .insert(packet_id, packet.clone());
            }
            QoS::ExactOnce => {
                self.publishing_qos2_packets
                    .insert(packet_id, packet.clone());
            }
            _ => (),
        }
        self.send_packet(packet)
    }

    /// Subscribe topic pattern.
    pub fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        let packet_id = self.next_packet_id();
        // TODO(Shaohua): Support multiple topics.
        //self.topics.insert(packet.topic().to_string(), packet_id);
        let packet = SubscribePacket::new(topic, qos, packet_id)?;
        self.subscribing_packets.insert(packet_id, packet.clone());
        self.send_packet(packet)
    }

    /// Unsubscribe topic pattern.
    pub fn unsubscribe(&mut self, topic: &str) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        let packet_id = self.next_packet_id();
        let packet = UnsubscribePacket::new(topic, packet_id)?;
        self.unsubscribing_packets.insert(packet_id, packet.clone());
        self.send_packet(packet)
    }

    /// Send disconnect packet to broker.
    pub fn disconnect(&mut self) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        let packet = DisconnectPacket::new();
        self.status = ClientStatus::Disconnecting;
        // Network connection will be closed soon.
        self.send_packet(packet)?;
        self.status = ClientStatus::Disconnected;
        Ok(())
    }

    /// Send ping packet to server.
    pub fn ping(&mut self) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        let packet = PingRequestPacket::new();
        self.send_packet(packet)
    }

    pub fn wait_for_packet(&mut self) -> Result<Option<PublishMessage>, Error> {
        todo!()
    }

    fn on_message(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("on_message()");
        let mut ba = ByteArray::new(buf);
        let packet = PublishPacket::decode(&mut ba)?;
        log::info!("on_message() packet: {:?}", packet);
        Ok(())
    }

    fn on_ping_resp(&self) -> Result<(), Error> {
        log::info!("on ping resp");
        // TODO(Shaohua): Reset reconnect timer.
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
            self.packet_id = PacketId::new(1);
        } else {
            self.packet_id += 1;
        }
        self.packet_id
    }

    fn read_stream(&mut self) -> Result<usize, Error> {
        self.buffer.resize(self.buffer.capacity(), 0);
        if let Some(stream) = &mut self.stream {
            match stream.read_buf(&mut self.buffer) {
                Ok(n_recv) => {
                    self.buffer.resize(n_recv, 0);
                    return Ok(n_recv);
                }
                Err(error) => {
                    return Err(Error::from_string(
                        ErrorKind::SocketError,
                        format!("Failed to read bytes from socket, err: {:?}", error),
                    ));
                }
            }
        } else {
            return Err(Error::new(
                ErrorKind::SocketError,
                "Socket is uninitialized",
            ));
        }
    }

    fn handle_session_packet(&mut self, buf: &mut Vec<u8>) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let fixed_header = FixedHeader::decode(&mut ba)?;
        log::info!("fixed header: {:?}", fixed_header);
        match fixed_header.packet_type() {
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

    fn send_packet<P: EncodePacket>(&mut self, packet: P) -> Result<(), Error> {
        // TODO(Shaohua): Replace Vec<u8> with ByteArray.
        let mut buf = Vec::new();
        packet.encode(&mut buf)?;
        if let Some(stream) = &mut self.stream {
            stream.write_all(&buf).map_err(|err| err.into())
        } else {
            Err(Error::new(
                ErrorKind::SocketError,
                "Socket is uninitialized",
            ))
        }
    }
}
