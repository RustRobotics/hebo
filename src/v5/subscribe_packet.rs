// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId, QoS, SubscribeTopic,
};

/// Subscribe packet is sent from the Client to the Server to subscribe one or more topics.
/// This packet also specifies the maximum QoS with which the Server can send Application
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
/// Each topic name is followed by associated QoS flag.
///
/// If a Server receives a Subscribe packet containing a Topic Filter that is identical
/// to an existing Subscription's Topic Filter then it must completely replace existing
/// Subscription with a new Subscription. The Topic Filter in the new Subscription will
/// be identical to the previous Subscription, also QoS may be different. Any existing
/// retained message will be re-sent to the new Subscrption.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SubscribePacket {
    /// `packet_id` is used by the Server to reply SubscribeAckPacket to the client.
    packet_id: PacketId,

    properties: Properties,

    /// A list of topic the Client subscribes to.
    topics: Vec<SubscribeTopic>,
}

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
    pub fn new(topic: &str, qos: QoS, packet_id: PacketId) -> Result<SubscribePacket, EncodeError> {
        let topic = SubscribeTopic::new(topic, qos)?;
        Ok(SubscribePacket {
            packet_id,
            properties: Properties::new(),
            topics: vec![topic],
        })
    }

    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn set_topics(&mut self, topics: &[SubscribeTopic]) -> &mut Self {
        self.topics.clear();
        self.topics.extend_from_slice(topics);
        self
    }

    pub fn topics(&self) -> &[SubscribeTopic] {
        &self.topics
    }

    pub fn mut_topics(&mut self) -> &mut Vec<SubscribeTopic> {
        &mut self.topics
    }
}

impl DecodePacket for SubscribePacket {
    fn decode(ba: &mut ByteArray) -> Result<SubscribePacket, DecodeError> {
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

        let mut remaining_length = packet_id.bytes() + properties.bytes();
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

        Ok(SubscribePacket {
            packet_id,
            properties,
            topics,
        })
    }
}

impl EncodePacket for SubscribePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let mut remaining_length = self.packet_id.bytes();
        for topic in &self.topics {
            remaining_length += topic.bytes();
        }

        let fixed_header = FixedHeader::new(PacketType::Subscribe, remaining_length)?;
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
}
