// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId, SubTopic};

/// The Client request to unsubscribe topics from the Server.
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
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UnsubscribePacket {
    /// Packet id is used in unsubscribe ack packet.
    packet_id: PacketId,

    properties: Properties,

    /// Topic filters to be unsubscribed.
    /// Note that these strings must exactly identical to the topic filters used in
    /// Subscribe packets.
    topics: Vec<SubTopic>,
}

impl UnsubscribePacket {
    /// Create a new unsubscribe packet which only contains one topic pattern.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn new(topic: &str, packet_id: PacketId) -> Result<Self, EncodeError> {
        let topic = SubTopic::new(topic)?;
        Ok(Self {
            packet_id,
            properties: Properties::new(),
            topics: vec![topic],
        })
    }

    /// Create a new unsubscribe packet which contains multiple topic patterns.
    ///
    /// # Errors
    ///
    /// Returns error if `topics` are invalid.
    pub fn with_topics(topics: &[&str], packet_id: PacketId) -> Result<Self, EncodeError> {
        let mut topics_result = Vec::with_capacity(topics.len());
        for topic in topics {
            let topic = SubTopic::new(topic)?;
            topics_result.push(topic);
        }
        Ok(Self {
            packet_id,
            properties: Properties::new(),
            topics: topics_result,
        })
    }

    /// Update packet id.
    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    /// Get curent packet id.
    #[must_use]
    pub const fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    /// Get a mutable reference to property list.
    pub fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Get a reference to property list.
    #[must_use]
    pub const fn properties(&self) -> &Properties {
        &self.properties
    }

    /// Add `topic` pattern to subscribed topic list.
    ///
    /// # Errors
    ///
    /// Returns error if topic pattern is invalid.
    pub fn add_topic(&mut self, topic: &str) -> Result<&mut Self, EncodeError> {
        let topic = SubTopic::new(topic)?;
        self.topics.push(topic);
        Ok(self)
    }

    /// Set `topics` as unsubscribed topic patterns.
    ///
    /// # Errors
    ///
    /// Returns error if topic pattern is invalid.
    pub fn set_topics(&mut self, topics: &[&str]) -> Result<&mut Self, EncodeError> {
        self.topics.clear();
        for topic in topics {
            let topic = SubTopic::new(topic)?;
            self.topics.push(topic);
        }
        Ok(self)
    }

    /// Get a reference to unsubscribed topic patterns.
    #[must_use]
    pub fn topics(&self) -> &[SubTopic] {
        &self.topics
    }

    /// Get a mutable references to unsubscribed topic patterns.
    pub fn mut_topics(&mut self) -> &mut Vec<SubTopic> {
        &mut self.topics
    }
}

/// Properties can be used in `UnsubscribePacket`.
pub const UNSUBSCRIBE_PROPERTIES: &[PropertyType] = &[PropertyType::UserProperty];

impl DecodePacket for UnsubscribePacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Unsubscribe {
            return Err(DecodeError::InvalidPacketType);
        }

        let packet_id = PacketId::decode(ba)?;
        if packet_id.value() == 0 {
            return Err(DecodeError::InvalidPacketId);
        }

        let properties = Properties::decode(ba)?;
        if let Err(property_type) =
            check_property_type_list(properties.props(), UNSUBSCRIBE_PROPERTIES)
        {
            log::error!(
                "v5/UnsubscribePacket: property type {:?} cannot be used in properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        let mut remaining_length = packet_id.bytes() + properties.bytes();
        let mut topics = Vec::new();
        while remaining_length < fixed_header.remaining_length() {
            let topic = SubTopic::decode(ba)?;
            remaining_length += topic.bytes();
            topics.push(topic);
        }

        // The Payload of an UNSUBSCRIBE packet MUST contain at least one Topic Filter [MQTT-3.10.3-2].
        if topics.is_empty() {
            return Err(DecodeError::EmptyTopicFilter);
        }

        Ok(Self {
            packet_id,
            properties,
            topics,
        })
    }
}

impl EncodePacket for UnsubscribePacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = v.len();
        let mut remaining_length: usize = self.packet_id.bytes() + self.properties.bytes();
        for topic in &self.topics {
            remaining_length += topic.bytes();
        }

        let fixed_header = FixedHeader::new(PacketType::Unsubscribe, remaining_length)?;
        fixed_header.encode(v)?;

        self.packet_id.encode(v)?;
        self.properties.encode(v)?;

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
