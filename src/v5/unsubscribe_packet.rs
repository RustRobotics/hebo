// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId, SubTopic};

/// The Client request to unsubscribe topics from the Server.
/// When the Server receives this packet, no more Publish packet will be sent to the Client.
/// Unfinished QoS 1 and QoS 2 packets will be delivered as usual.
///
/// Basic packet struct:
/// ```txt
/// +-------------------------+
/// | Fixed header            |
/// |                         |
/// +-------------------------+
/// | Packet id               |
/// |                         |
/// +-------------------------+
/// | Properties ...          |
/// +-------------------------+
/// | Topic 0 length          |
/// |                         |
/// +-------------------------+
/// | Topic 0 ...             |
/// +-------------------------+
/// | Topic 1 length          |
/// |                         |
/// +-------------------------+
/// | Topic 1 ...             |
/// +-------------------------+
/// | Topic N length          |
/// |                         |
/// +-------------------------+
/// | Topic N ...             |
/// +-------------------------+
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UnsubscribePacket {
    /// Used in UnsubscribeAck packet.
    packet_id: PacketId,

    properties: Properties,

    /// Topic filters to be unsubscribed.
    /// Note that these strings must exactly identical to the topic filters used in
    /// Subscribe packets.
    topics: Vec<SubTopic>,
}

impl UnsubscribePacket {
    pub fn new(topic: &str, packet_id: PacketId) -> Result<Self, EncodeError> {
        let topic = SubTopic::new(topic)?;
        Ok(UnsubscribePacket {
            packet_id,
            properties: Properties::new(),
            topics: vec![topic],
        })
    }

    pub fn with_topics(topics: &[&str], packet_id: PacketId) -> Result<Self, EncodeError> {
        let mut topics_result = Vec::with_capacity(topics.len());
        for topic in topics {
            let topic = SubTopic::new(topic)?;
            topics_result.push(topic);
        }
        Ok(UnsubscribePacket {
            packet_id,
            properties: Properties::new(),
            topics: topics_result,
        })
    }

    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn add_topic(&mut self, topic: &str) -> Result<&mut Self, EncodeError> {
        let topic = SubTopic::new(topic)?;
        self.topics.push(topic);
        Ok(self)
    }

    pub fn set_topics(&mut self, topics: &[&str]) -> Result<&mut Self, EncodeError> {
        self.topics.clear();
        for topic in topics {
            let topic = SubTopic::new(topic)?;
            self.topics.push(topic);
        }
        Ok(self)
    }

    pub fn topics(&self) -> &[SubTopic] {
        &self.topics
    }

    pub fn mut_topics(&mut self) -> &mut Vec<SubTopic> {
        &mut self.topics
    }
}

impl DecodePacket for UnsubscribePacket {
    fn decode(ba: &mut ByteArray) -> Result<UnsubscribePacket, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Unsubscribe {
            return Err(DecodeError::InvalidPacketType);
        }

        let packet_id = PacketId::decode(ba)?;
        if packet_id.value() == 0 {
            // SUBSCRIBE, UNSUBSCRIBE, and PUBLISH (in cases where QoS > 0) Control Packets
            // MUST contain a non-zero 16-bit Packet Identifier. [MQTT-2.3.1-1]
            return Err(DecodeError::InvalidPacketId);
        }

        let mut remaining_length = packet_id.bytes();
        let mut topics = Vec::new();
        while remaining_length < fixed_header.remaining_length() {
            let topic = SubTopic::decode(ba)?;
            remaining_length += topic.bytes();
            topics.push(topic);
        }

        // The Payload of an UNSUBSCRIBE packet MUST contain at least one Topic Filter.
        // An UNSUBSCRIBE packet with no payload is a protocol violation [MQTT-4.10.3-2].
        if topics.is_empty() {
            return Err(DecodeError::EmptyTopicFilter);
        }

        Ok(UnsubscribePacket { packet_id, topics })
    }
}

impl EncodePacket for UnsubscribePacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = v.len();
        let mut remaining_length: usize = self.packet_id.bytes();
        for topic in &self.topics {
            remaining_length += topic.bytes();
        }

        let fixed_header = FixedHeader::new(PacketType::Unsubscribe, remaining_length)?;
        fixed_header.encode(v)?;

        self.packet_id.encode(v)?;

        for topic in &self.topics {
            topic.encode(v)?;
        }

        Ok(v.len() - old_len)
    }
}

impl Packet for UnsubscribePacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Unsubscribe
    }
}
