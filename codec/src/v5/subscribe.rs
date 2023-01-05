// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::{
    property::check_multiple_subscription_identifiers, property::check_property_type_list,
    Properties, PropertyType,
};
use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, Packet, PacketId,
    PacketType, QoS, SubTopic, VarIntError,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RetainHandling {
    /// 0 = Send retained messages at the time of the subscribe.
    #[default]
    Send = 0,

    /// 1 = Send retained messages at subscribe only if the subscription does not currently exist.
    SendFirst = 1,

    /// 2 = Do not send retained messages at the time of the subscribe.
    NoSend = 2,
}

impl TryFrom<u8> for RetainHandling {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Send),
            1 => Ok(Self::SendFirst),
            2 => Ok(Self::NoSend),
            _ => Err(DecodeError::OtherErrors),
        }
    }
}

/// Topic/QoS pair.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SubscribeTopic {
    /// Subscribed `topic` contains wildcard characters to match interested topics with patterns.
    topic: SubTopic,

    /// Bits 0 and 1 of the Subscription Options represent Maximum QoS field.
    ///
    /// This gives the maximum QoS level at which the Server can send Application Messages
    /// to the Client. It is a Protocol Error if the Maximum QoS field has the value 3.
    qos: QoS,

    /// Bit 2 of the Subscription Options represents the No Local option.
    ///
    /// If the value is 1, Application Messages MUST NOT be forwarded to a connection
    /// with a ClientID equal to the ClientID of the publishing connection [MQTT-3.8.3-3].
    ///
    /// It is a Protocol Error to set the No Local bit to 1 on a Shared Subscription [MQTT-3.8.3-4].
    no_local: bool,

    /// Bit 3 of the Subscription Options represents the Retain As Published option.
    ///
    /// If 1, Application Messages forwarded using this subscription keep the RETAIN flag
    /// they were published with. If 0, Application Messages forwarded using this subscription
    /// have the RETAIN flag set to 0. Retained messages sent when the subscription
    /// is established have the RETAIN flag set to 1.
    retain_as_published: bool,

    /// Bits 4 and 5 of the Subscription Options represent the Retain Handling option.
    ///
    /// This option specifies whether retained messages are sent when the subscription
    /// is established. This does not affect the sending of retained messages
    /// at any point after the subscribe. If there are no retained messages
    /// matching the Topic Filter, all of these values act the same. The values are:
    ///
    /// - 0 = Send retained messages at the time of the subscribe
    /// - 1 = Send retained messages at subscribe only if the subscription does not currently exist
    /// - 2 = Do not send retained messages at the time of the subscribe
    ///
    /// It is a Protocol Error to send a Retain Handling value of 3.
    retain_handling: RetainHandling,
}

impl SubscribeTopic {
    /// Create a new subscribe topic.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn new(topic: &str, qos: QoS) -> Result<Self, EncodeError> {
        let topic = SubTopic::new(topic)?;
        Ok(Self {
            topic,
            qos,
            ..Self::default()
        })
    }

    /// Update topic pattern.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn set_topic(&mut self, topic: &str) -> Result<&mut Self, EncodeError> {
        self.topic = SubTopic::new(topic)?;
        Ok(self)
    }

    /// Get current topic pattern.
    #[must_use]
    pub fn topic(&self) -> &str {
        self.topic.as_ref()
    }

    /// Update `qos` value.
    pub fn set_qos(&mut self, qos: QoS) -> &mut Self {
        self.qos = qos;
        self
    }

    /// Get current `QoS`.
    #[must_use]
    pub const fn qos(&self) -> QoS {
        self.qos
    }

    /// Set `no_local` flag.
    pub fn set_no_local(&mut self, no_local: bool) -> &mut Self {
        self.no_local = no_local;
        self
    }

    /// Get `no_local` flag.
    #[must_use]
    pub const fn no_local(&self) -> bool {
        self.no_local
    }

    /// Update `retain_as_published` flag.
    pub fn set_retain_as_published(&mut self, retain_as_published: bool) -> &mut Self {
        self.retain_as_published = retain_as_published;
        self
    }

    /// Get `retain_as_published` flag.
    #[must_use]
    pub const fn retain_as_published(&self) -> bool {
        self.retain_as_published
    }

    /// Update `retain_handling` flag.
    pub fn set_retain_handling(&mut self, retain_handling: RetainHandling) -> &mut Self {
        self.retain_handling = retain_handling;
        self
    }

    /// Get `retain_handling` flag.
    #[must_use]
    pub const fn retain_handling(&self) -> RetainHandling {
        self.retain_handling
    }

    pub fn bytes(&self) -> usize {
        1 + self.topic.bytes()
    }
}

impl EncodePacket for SubscribeTopic {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        self.topic.encode(buf)?;
        let mut flag: u8 = 0b0000_0011 & (self.qos as u8);
        if self.no_local {
            flag |= 0b0000_0100;
        }
        if self.retain_as_published {
            flag |= 0b0000_1000;
        }
        flag |= 0b0011_0000 & (self.retain_handling as u8);
        buf.push(flag);

        Ok(self.bytes())
    }
}

impl DecodePacket for SubscribeTopic {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let topic = SubTopic::decode(ba)?;

        let flag = ba.read_byte()?;
        // Bits 0 and 1 of the Subscription Options represent Maximum QoS field.
        // This gives the maximum QoS level at which the Server can send
        // Application Messages to the Client. It is a Protocol Error if
        // the Maximum QoS field has the value 3.
        let qos = QoS::try_from(flag & 0b0000_0011)?;

        let no_local = (flag & 0b0000_0100) == 0b0000_0100;
        let retain_as_published = (flag & 0b0000_1000) == 0b0000_1000;
        let retain_handling = RetainHandling::try_from(flag & 0b0011_0000)?;

        // Bits 6 and 7 of the Subscription Options byte are reserved for future use.
        // The Server MUST treat a SUBSCRIBE packet as malformed if any of Reserved bits
        // in the Payload are non-zero [MQTT-3.8.3-5].
        if flag & 0b1100_0000 != 0b0000_0000 {
            return Err(DecodeError::OtherErrors);
        }

        Ok(Self {
            topic,
            qos,
            no_local,
            retain_as_published,
            retain_handling,
        })
    }
}

/// Subscribe packet is sent from the Client to the Server to subscribe one or more topics.
/// This packet also specifies the maximum `QoS` with which the Server can send Application
/// message to the Client.
///
/// Basic struct of this packet:
///
/// ```txt
/// +----------------------------+
/// | Fixed header               |
/// |                            |
/// +----------------------------+
/// | Packet Id                  |
/// |                            |
/// +----------------------------+
/// | Properties ...             |
/// +----------------------------+
/// | Topic 0 length             |
/// |                            |
/// +----------------------------+
/// | Topic 0 ...                |
/// +----------------------------+
/// | Topic 0 QoS                |
/// +----------------------------+
/// | Topic 1 length             |
/// |                            |
/// +----------------------------+
/// | Topic 1 ...                |
/// +----------------------------+
/// | Tpoic 1 QoS                |
/// +----------------------------+
/// | ...                        |
/// +----------------------------+
/// ```
///
/// Each topic name is followed by associated `QoS` flag.
///
/// If a Server receives a Subscribe packet containing a Topic Filter that is identical
/// to an existing Subscription's Topic Filter then it must completely replace existing
/// Subscription with a new Subscription. The Topic Filter in the new Subscription will
/// be identical to the previous Subscription, also `QoS` may be different. Any existing
/// retained message will be re-sent to the new Subscrption.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SubscribePacket {
    /// `packet_id` is used by the Server to reply SubscribeAckPacket to the client.
    packet_id: PacketId,

    properties: Properties,

    /// A list of topic the Client subscribes to.
    topics: Vec<SubscribeTopic>,
}

/// Properties available in subscribe packet.
pub const SUBSCRIBE_PROPERTIES: &[PropertyType] = &[
    // The Subscription Identifier can have the value of 1 to 268,435,455.
    // It is a Protocol Error if the Subscription Identifier has a value of 0.
    // It is a Protocol Error to include the Subscription Identifier more than once.
    //
    // The Subscription Identifier is associated with any subscription created or
    // modified as the result of this SUBSCRIBE packet. If there is a Subscription Identifier,
    // it is stored with the subscription. If this property is not specified,
    // then the absence of a Subscription Identifier is stored with the subscription.
    PropertyType::SubscriptionIdentifier,
    PropertyType::UserProperty,
];

impl SubscribePacket {
    /// Create a new subscribe packet.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` pattern is invalid.
    pub fn new(topic: &str, qos: QoS, packet_id: PacketId) -> Result<Self, EncodeError> {
        let topic = SubscribeTopic::new(topic, qos)?;
        Ok(Self {
            packet_id,
            properties: Properties::new(),
            topics: vec![topic],
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

    /// Get a mutable reference to property list.
    pub fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Get a reference to property list.
    #[must_use]
    pub const fn properties(&self) -> &Properties {
        &self.properties
    }

    /// Update topic patterns.
    pub fn set_topics(&mut self, topics: &[SubscribeTopic]) -> &mut Self {
        self.topics.clear();
        self.topics.extend_from_slice(topics);
        self
    }

    /// Get a reference to topic patterns.
    #[must_use]
    pub fn topics(&self) -> &[SubscribeTopic] {
        &self.topics
    }

    /// Get a mutable reference to topic patterns.
    pub fn mut_topics(&mut self) -> &mut Vec<SubscribeTopic> {
        &mut self.topics
    }

    fn get_fixed_header(&self) -> Result<FixedHeader, VarIntError> {
        let mut remaining_length = PacketId::bytes();
        for topic in &self.topics {
            remaining_length += topic.bytes();
        }

        FixedHeader::new(PacketType::Subscribe, remaining_length)
    }
}

impl DecodePacket for SubscribePacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Subscribe {
            return Err(DecodeError::InvalidPacketType);
        }

        let packet_id = PacketId::decode(ba)?;
        if packet_id.value() == 0 {
            // SUBSCRIBE, UNSUBSCRIBE, and PUBLISH (in cases where QoS > 0) Control Packets
            // MUST contain a non-zero 16-bit Packet Identifier. [MQTT-2.3.1-1]
            return Err(DecodeError::InvalidPacketId);
        }

        let properties = Properties::decode(ba)?;
        if let Err(property_type) =
            check_property_type_list(properties.props(), SUBSCRIBE_PROPERTIES)
        {
            log::error!(
                "v5/SubscribePacket: property type {:?} cannot be used in properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }
        if let Err(property_type) = check_multiple_subscription_identifiers(properties.props()) {
            log::error!(
                "v5/SubscribePacket: property type {:?} cannot be used in properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        let mut remaining_length = PacketId::bytes() + properties.bytes();
        let mut topics = Vec::new();

        // Parse topic/qos list.
        while remaining_length < fixed_header.remaining_length() {
            let topic = SubscribeTopic::decode(ba)?;
            remaining_length += topic.bytes();
            topics.push(topic);
        }

        // The payload of a SUBSCRIBE packet MUST contain at least one Topic Filter / QoS pair.
        // A SUBSCRIBE packet with no payload is a protocol violation [MQTT-3.8.3-3].
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

impl EncodePacket for SubscribePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = self.get_fixed_header()?;
        fixed_header.encode(buf)?;

        // Variable header
        self.packet_id.encode(buf)?;

        // Payload
        for topic in &self.topics {
            topic.encode(buf)?;
        }

        Ok(buf.len() - old_len)
    }
}

impl Packet for SubscribePacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Subscribe
    }

    fn bytes(&self) -> Result<usize, VarIntError> {
        let fixed_header = self.get_fixed_header()?;
        Ok(fixed_header.bytes() + fixed_header.remaining_length())
    }
}
