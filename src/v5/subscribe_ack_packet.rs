// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

/// Reply to each subscribed topic.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscribeReasonCode {
    /// The subscription is accepted and the maximum QoS sent will be QoS 0.
    ///
    /// This might be a lower QoS than was requested.
    GrantedQoS0 = 0x00,

    /// The subscription is accepted and the maximum QoS sent will be QoS 1.
    ///
    /// This might be a lower QoS than was requested.
    GrantedQoS1 = 0x01,

    /// The subscription is accepted and any received QoS will be sent to this subscription.
    GrantedQoS2 = 0x02,

    /// The subscription is not accepted and the Server either does not wish to reveal
    /// the reason or none of the other Reason Codes apply.
    UnspecifiedError = 0x80,

    /// The SUBSCRIBE is valid but the Server does not accept it.
    ImplementationSpecificError = 0x83,

    /// The Client is not authorized to make this subscription.
    NotAuthorized = 0x87,

    /// The Topic Filter is correctly formed but is not allowed for this Client.
    TopicFilterInvalid = 0x8f,

    /// The specified Packet Identifier is already in use.
    PacketIdentifierInUse = 0x91,

    /// An implementation or administrative imposed limit has been exceeded.
    QuotaExceeded = 0x97,

    /// The Server does not support Shared Subscriptions for this Client.
    SharedSubscriptionsNotSupported = 0x9e,

    /// The Server does not support Subscription Identifiers; the subscription is not accepted.
    SubscriptionIdentifiersNotSupported = 0xa1,

    /// The Server does not support Wildcard Subscriptions; the subscription is not accepted.
    WildcardSubscriptionsNotSupported = 0xa2,
}

impl Default for SubscribeReasonCode {
    fn default() -> Self {
        Self::GrantedQoS0
    }
}

impl SubscribeReasonCode {
    pub fn bytes(&self) -> usize {
        1
    }

    pub fn const_bytes() -> usize {
        1
    }
}

impl TryFrom<u8> for SubscribeReasonCode {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(Self::GrantedQoS0),
            0x01 => Ok(Self::GrantedQoS1),
            0x02 => Ok(Self::GrantedQoS2),
            0x80 => Ok(Self::UnspecifiedError),
            0x83 => Ok(Self::ImplementationSpecificError),
            0x87 => Ok(Self::NotAuthorized),
            0x8f => Ok(Self::TopicFilterInvalid),
            0x91 => Ok(Self::PacketIdentifierInUse),
            0x97 => Ok(Self::QuotaExceeded),
            0x9e => Ok(Self::SharedSubscriptionsNotSupported),
            0xa1 => Ok(Self::SubscriptionIdentifiersNotSupported),
            0xa2 => Ok(Self::WildcardSubscriptionsNotSupported),
            _ => Err(DecodeError::OtherErrors),
        }
    }
}

impl DecodePacket for SubscribeReasonCode {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let byte = ba.read_byte()?;
        let flag = Self::try_from(byte)?;
        Ok(flag)
    }
}

impl EncodePacket for SubscribeReasonCode {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.push(*self as u8);
        Ok(self.bytes())
    }
}

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
    reasons: Vec<SubscribeReasonCode>,
}

pub const SUBSCRIBE_ACK_PROPERTIES: &[PropertyType] = &[
    // The Server MUST NOT send this Property if it would increase the size of
    // the SUBACK packet beyond the Maximum Packet Size specified by the Client [MQTT-3.9.2-1].
    PropertyType::ReasonString,
    // The Server MUST NOT send this Property if it would increase the size of
    // the SUBACK packet beyond the Maximum Packet Size specified by the Client [MQTT-3.9.2-1]
    PropertyType::UserProperty,
];

impl SubscribeAckPacket {
    pub fn new(packet_id: PacketId, reason: SubscribeReasonCode) -> Self {
        Self {
            packet_id,
            properties: Properties::new(),
            reasons: vec![reason],
        }
    }

    pub fn with_vec(packet_id: PacketId, reasons: Vec<SubscribeReasonCode>) -> Self {
        Self {
            packet_id,
            properties: Properties::new(),
            reasons,
        }
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

    pub fn reasons_mut(&mut self) -> &mut Vec<SubscribeReasonCode> {
        &mut self.reasons
    }

    pub fn reasons(&self) -> &[SubscribeReasonCode] {
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
        let mut remaining_length = packet_id.bytes() + properties.bytes();

        while remaining_length < fixed_header.remaining_length() {
            let reason = SubscribeReasonCode::decode(ba)?;
            reasons.push(reason);
            remaining_length += reason.bytes();
        }

        Ok(SubscribeAckPacket {
            packet_id,
            properties,
            reasons,
        })
    }
}

impl EncodePacket for SubscribeAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();
        let remaining_length = self.packet_id.bytes()
            + self.properties.bytes()
            + self.reasons.len() * SubscribeReasonCode::const_bytes();
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
