// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, Packet, PacketId,
    PacketType, SubTopic, VarIntError,
};

/// The Client request to unsubscribe topics from the Server.
///
/// When the Server receives this packet, no more Publish packet will be sent to the Client.
/// Unfinished `QoS` 1 and `QoS` 2 packets will be delivered as usual.
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
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UnsubscribePacket {
    /// Used in UnsubscribeAck packet.
    packet_id: PacketId,

    /// Topic filters to be unsubscribed.
    ///
    /// Note that these strings must exactly identical to the topic filters used in
    /// Subscribe packets.
    topics: Vec<SubTopic>,
}

impl UnsubscribePacket {
    /// Create a new unsubscribe packet.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn new(topic: &str, packet_id: PacketId) -> Result<Self, EncodeError> {
        let topic = SubTopic::new(topic)?;
        Ok(Self {
            packet_id,
            topics: vec![topic],
        })
    }

    /// Create a new unsubscribe packet with multiple `topics`.
    ///
    /// # Errors
    ///
    /// Returns error if some topic is invalid.
    pub fn with_topics(topics: &[&str], packet_id: PacketId) -> Result<Self, EncodeError> {
        let mut topics_result = Vec::with_capacity(topics.len());
        for topic in topics {
            let topic = SubTopic::new(topic)?;
            topics_result.push(topic);
        }
        Ok(Self {
            packet_id,
            topics: topics_result,
        })
    }

    /// Update packet id.
    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    /// Get current packet id.
    #[must_use]
    pub const fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    /// Add `topic` to topic list.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn add_topic(&mut self, topic: &str) -> Result<&mut Self, EncodeError> {
        let topic = SubTopic::new(topic)?;
        self.topics.push(topic);
        Ok(self)
    }

    /// Updae topic list.
    ///
    /// # Errors
    ///
    /// Returns error if some topic is invalid.
    pub fn set_topics(&mut self, topics: &[&str]) -> Result<&mut Self, EncodeError> {
        self.topics.clear();
        for topic in topics {
            let topic = SubTopic::new(topic)?;
            self.topics.push(topic);
        }
        Ok(self)
    }

    /// Get a reference to topic list.
    #[must_use]
    pub fn topics(&self) -> &[SubTopic] {
        &self.topics
    }

    /// Get a mutable reference to topic list.
    pub fn mut_topics(&mut self) -> &mut Vec<SubTopic> {
        &mut self.topics
    }

    fn get_fixed_header(&self) -> Result<FixedHeader, VarIntError> {
        let mut remaining_length: usize = PacketId::bytes();
        for topic in &self.topics {
            remaining_length += topic.bytes();
        }

        FixedHeader::new(PacketType::Unsubscribe, remaining_length)
    }
}

impl DecodePacket for UnsubscribePacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
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

        let mut remaining_length = PacketId::bytes();
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

        Ok(Self { packet_id, topics })
    }
}

impl EncodePacket for UnsubscribePacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = v.len();

        let fixed_header = self.get_fixed_header()?;
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

    fn bytes(&self) -> Result<usize, VarIntError> {
        let fixed_header = self.get_fixed_header()?;
        Ok(fixed_header.bytes() + fixed_header.remaining_length())
    }
}
