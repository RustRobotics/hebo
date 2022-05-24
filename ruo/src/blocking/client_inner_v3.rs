// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::v3::{
    ConnectAckPacket, ConnectPacket, ConnectReturnCode, DisconnectPacket, PingRequestPacket,
    PublishAckPacket, PublishPacket, SubscribeAckPacket, SubscribePacket, UnsubscribeAckPacket,
    UnsubscribePacket,
};
use codec::{ByteArray, DecodePacket, EncodePacket, FixedHeader, PacketId, PacketType, QoS};
use std::collections::HashMap;

use super::Stream;
use crate::connect_options::ConnectOptions;
use crate::error::{Error, ErrorKind};
use crate::{ClientStatus, PublishMessage};

/// MQTT Client for V3.1.
pub struct ClientInnerV3 {
    connect_options: ConnectOptions,
    status: ClientStatus,

    stream: Option<Stream>,
    _topics: HashMap<String, PacketId>,
    packet_id: PacketId,
    subscribing_packets: HashMap<PacketId, SubscribePacket>,
    unsubscribing_packets: HashMap<PacketId, UnsubscribePacket>,
    publishing_qos1_packets: HashMap<PacketId, PublishPacket>,
    publishing_qos2_packets: HashMap<PacketId, PublishPacket>,
}

impl Drop for ClientInnerV3 {
    fn drop(&mut self) {
        if self.status == ClientStatus::Connected {
            // Send Disconnect packet to server.
            // The server will close connection without sending any response packet any more.
            let _ret = self.disconnect();
        }
    }
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
            _topics: HashMap::new(),
            packet_id: PacketId::new(1),
            subscribing_packets: HashMap::new(),
            unsubscribing_packets: HashMap::new(),
            publishing_qos1_packets: HashMap::new(),
            publishing_qos2_packets: HashMap::new(),
        }
    }

    /// Get connection options.
    #[must_use]
    pub const fn connect_options(&self) -> &ConnectOptions {
        &self.connect_options
    }

    /// Get current connection status.
    #[must_use]
    pub const fn status(&self) -> ClientStatus {
        self.status
    }

    /// Connct to server.
    ///
    /// Returns Ok() if success.
    pub fn connect(&mut self) -> Result<(), Error> {
        // TODO(Shaohua): Do not return bool, return errors instead.
        assert_eq!(self.status, ClientStatus::Disconnected);
        let stream = Stream::new(self.connect_options.connect_type())?;
        self.stream = Some(stream);
        let conn_packet = ConnectPacket::new(self.connect_options.client_id())?;
        self.status = ClientStatus::Connecting;
        self.send_packet(&conn_packet)?;

        // We read ConnectAck packet directly here,
        // because the first packet shall be Connect Packet.
        let mut buffer = Vec::with_capacity(128);
        loop {
            // TODO(Shaohua): Enable blocking mode.
            let n_recv = self.read_stream(&mut buffer)?;
            if n_recv > 0 {
                break;
            }
        }

        let mut ba = ByteArray::new(&buffer);
        let fixed_header = FixedHeader::decode(&mut ba)?;
        match fixed_header.packet_type() {
            PacketType::ConnectAck => {
                ba.reset_offset();
                let packet = ConnectAckPacket::decode(&mut ba)?;
                if packet.return_code() == ConnectReturnCode::Accepted {
                    self.status = ClientStatus::Connected;
                    Ok(())
                } else {
                    self.status = ClientStatus::Disconnected;
                    Err(Error::from_string(
                        ErrorKind::AuthFailed,
                        format!("return code: {:?}", packet.return_code()),
                    ))
                }
            }

            t => {
                self.status = ClientStatus::Disconnected;
                Err(Error::from_string(
                    ErrorKind::PacketError,
                    format!("Expected connect packet, got: {:?}", t),
                ))
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
            QoS::AtMostOnce => (),
        }
        self.send_packet(&packet)
    }

    /// Subscribe topic pattern.
    pub fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        let packet_id = self.next_packet_id();
        // TODO(Shaohua): Support multiple topics.
        //self.topics.insert(packet.topic().to_string(), packet_id);
        let packet = SubscribePacket::new(topic, qos, packet_id)?;
        self.subscribing_packets.insert(packet_id, packet.clone());
        self.send_packet(&packet)
    }

    /// Unsubscribe topic pattern.
    pub fn unsubscribe(&mut self, topic: &str) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        let packet_id = self.next_packet_id();
        let packet = UnsubscribePacket::new(topic, packet_id)?;
        self.unsubscribing_packets.insert(packet_id, packet.clone());
        self.send_packet(&packet)
    }

    /// Send disconnect packet to broker.
    pub fn disconnect(&mut self) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        let packet = DisconnectPacket::new();
        self.status = ClientStatus::Disconnecting;
        // Network connection will be closed soon.
        self.send_packet(&packet)?;
        self.status = ClientStatus::Disconnected;
        Ok(())
    }

    /// Send ping packet to server.
    pub fn ping(&mut self) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        let packet = PingRequestPacket::new();
        self.send_packet(&packet)
    }

    pub fn wait_for_packet(&mut self) -> Result<Option<PublishMessage>, Error> {
        // TODO(Shaohua): Support large packets.
        let mut buffer = Vec::with_capacity(1024);
        loop {
            let n_recv = self.read_stream(&mut buffer)?;
            if n_recv > 0 {
                break;
            }
        }

        let mut offset: usize = 0;

        loop {
            let slice = &buffer[offset..];
            let mut ba = ByteArray::new(slice);
            let fixed_header = FixedHeader::decode(&mut ba)?;
            ba.reset_offset();
            match fixed_header.packet_type() {
                PacketType::PublishAck => self.on_publish_ack(&mut ba)?,
                PacketType::SubscribeAck => self.on_subscribe_ack(&mut ba)?,
                PacketType::UnsubscribeAck => self.on_unsubscribe_ack(&mut ba)?,
                PacketType::PingResponse => self.on_ping_resp(&mut ba)?,
                PacketType::Publish { .. } => {
                    let msg = self.on_publish_message(&mut ba)?;
                    return Ok(Some(msg));
                }
                t => {
                    log::error!("Unhandled msg: {:?}", t);
                }
            }

            offset += ba.offset();
            if offset >= buffer.len() {
                break;
            }
        }

        Ok(None)
    }

    fn on_publish_message(&mut self, ba: &mut ByteArray) -> Result<PublishMessage, Error> {
        // TODO(Shaohua): Support QoS1 / QoS2.
        let packet = PublishPacket::decode(ba)?;
        Ok(PublishMessage {
            topic: packet.topic().to_owned(),
            qos: packet.qos(),
            payload: packet.message().into(),
        })
    }

    fn on_ping_resp(&self, ba: &mut ByteArray) -> Result<(), Error> {
        log::info!("on ping resp");
        let _ping_req = PingRequestPacket::decode(ba)?;
        // TODO(Shaohua): Reset reconnect timer.
        Ok(())
    }

    fn on_publish_ack(&mut self, ba: &mut ByteArray) -> Result<(), Error> {
        // TODO(Shaohua): Support QoS2
        let packet = PublishAckPacket::decode(ba)?;
        let packet_id = packet.packet_id();
        if let Some(p) = self.publishing_qos1_packets.get(&packet_id) {
            log::info!("Topic `{}` publish confirmed!", p.topic());
            self.publishing_qos1_packets.remove(&packet.packet_id());
        } else {
            log::warn!("Failed to find PublishAckPacket: {}", packet_id);
        }
        Ok(())
    }

    fn on_subscribe_ack(&mut self, ba: &mut ByteArray) -> Result<(), Error> {
        // Parse packet_id and remove from cache.
        let packet = SubscribeAckPacket::decode(ba)?;
        let packet_id = packet.packet_id();
        if self.subscribing_packets.remove(&packet_id).is_none() {
            log::warn!("Failed to find SubscribeAckPacket: {}", packet_id);
        }
        Ok(())
    }

    fn on_unsubscribe_ack(&mut self, ba: &mut ByteArray) -> Result<(), Error> {
        let packet = UnsubscribeAckPacket::decode(ba)?;
        let packet_id = packet.packet_id();
        if self.unsubscribing_packets.remove(&packet_id).is_none() {
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

    fn read_stream(&mut self, buffer: &mut Vec<u8>) -> Result<usize, Error> {
        // TODO(Shaohua): Do not resize buffer.
        buffer.resize(buffer.capacity(), 0);
        self.stream.as_mut().map_or_else(
            || {
                Err(Error::new(
                    ErrorKind::SocketError,
                    "Socket is uninitialized",
                ))
            },
            |stream| match stream.read_buf(buffer) {
                Ok(n_recv) => {
                    buffer.resize(n_recv, 0);
                    Ok(n_recv)
                }
                Err(error) => Err(Error::from_string(
                    ErrorKind::SocketError,
                    format!("Failed to read bytes from socket, err: {:?}", error),
                )),
            },
        )
    }

    fn send_packet<P: EncodePacket>(&mut self, packet: &P) -> Result<(), Error> {
        // TODO(Shaohua): Replace Vec<u8> with ByteArray.
        let mut buf = Vec::new();
        packet.encode(&mut buf)?;
        self.stream.as_mut().map_or_else(
            || {
                Err(Error::new(
                    ErrorKind::SocketError,
                    "Socket is uninitialized",
                ))
            },
            |stream| stream.write_all(&buf).map(drop),
        )
    }
}
