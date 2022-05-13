// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType, ReasonCode};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

/// Reply to Subscribe packet.
///
/// Basic structure of packet is:
/// ```txt
/// +---------------------------+
/// | Fixed header              |
/// |                           |
/// +---------------------------+
/// | Packet id                 |
/// |                           |
/// +---------------------------+
/// | Properties ...            |
/// +---------------------------+
/// | Ack 0                     |
/// +---------------------------+
/// | Ack 1                     |
/// +---------------------------+
/// | Ack N ...                 |
/// +---------------------------+
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SubscribeAckPacket {
    /// `packet_id` field is identical in Subscribe packet.
    packet_id: PacketId,

    properties: Properties,

    /// The Payload contains a list of Reason Codes.
    ///
    /// Each Reason Code corresponds to a Topic Filter in the SUBSCRIBE packet
    /// being acknowledged. The order of Reason Codes in the SUBACK packet MUST match
    /// the order of Topic Filters in the SUBSCRIBE packet [MQTT-3.9.3-1].
    reasons: Vec<ReasonCode>,
}

/// Reply to each subscribed topic.
pub const SUBSCRIBE_REASONS: &[ReasonCode] = &[
    ReasonCode::Success,
    ReasonCode::GrantedQoS1,
    ReasonCode::GrantedQoS2,
    ReasonCode::UnspecifiedError,
    ReasonCode::ImplementationSpecificError,
    ReasonCode::NotAuthorized,
    ReasonCode::TopicFilterInvalid,
    ReasonCode::PacketIdentifierInUse,
    ReasonCode::QuotaExceeded,
    ReasonCode::SharedSubscriptionNotSupported,
    ReasonCode::SubscriptionIdentifiersNotSupported,
    ReasonCode::WildcardSubscriptionsNotSupported,
];

/// Properties available in subscribe ack packet.
pub const SUBSCRIBE_ACK_PROPERTIES: &[PropertyType] = &[
    // The Server MUST NOT send this Property if it would increase the size of
    // the SUBACK packet beyond the Maximum Packet Size specified by the Client [MQTT-3.9.2-1].
    PropertyType::ReasonString,
    // The Server MUST NOT send this Property if it would increase the size of
    // the SUBACK packet beyond the Maximum Packet Size specified by the Client [MQTT-3.9.2-1]
    PropertyType::UserProperty,
];

impl SubscribeAckPacket {
    /// Create a new subscribe ack packet.
    #[must_use]
    pub fn new(packet_id: PacketId, reason: ReasonCode) -> Self {
        Self {
            packet_id,
            properties: Properties::new(),
            reasons: vec![reason],
        }
    }

    /// Create a new subscribe ack packet with reason code list.
    #[must_use]
    pub fn with_vec(packet_id: PacketId, reasons: Vec<ReasonCode>) -> Self {
        Self {
            packet_id,
            properties: Properties::new(),
            reasons,
        }
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

    /// Get a mutable reference to reason code list.
    pub fn reasons_mut(&mut self) -> &mut Vec<ReasonCode> {
        &mut self.reasons
    }

    /// Get a reference to reason code list.
    #[must_use]
    pub fn reasons(&self) -> &[ReasonCode] {
        &self.reasons
    }
}

impl DecodePacket for SubscribeAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::SubscribeAck {
            return Err(DecodeError::InvalidPacketType);
        }

        let packet_id = PacketId::decode(ba)?;
        let properties = Properties::decode(ba)?;
        if let Err(property_type) =
            check_property_type_list(properties.props(), SUBSCRIBE_ACK_PROPERTIES)
        {
            log::error!(
                "v5/SubscribeAckPacket: property type {:?} cannot be used in properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        let mut reasons = Vec::new();
        let mut remaining_length = PacketId::bytes() + properties.bytes();

        while remaining_length < fixed_header.remaining_length() {
            let reason = ReasonCode::decode(ba)?;
            if !SUBSCRIBE_REASONS.contains(&reason) {
                return Err(DecodeError::InvalidReasonCode);
            }
            reasons.push(reason);
            remaining_length += ReasonCode::bytes();
        }

        Ok(Self {
            packet_id,
            properties,
            reasons,
        })
    }
}

impl EncodePacket for SubscribeAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();
        let remaining_length =
            PacketId::bytes() + self.properties.bytes() + self.reasons.len() * ReasonCode::bytes();
        let fixed_header = FixedHeader::new(PacketType::SubscribeAck, remaining_length)?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        self.properties.encode(buf)?;

        for reason in &self.reasons {
            reason.encode(buf)?;
        }

        Ok(buf.len() - old_len)
    }
}

impl Packet for SubscribeAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::SubscribeAck
    }
}
