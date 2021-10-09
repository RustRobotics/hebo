// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

/// Each Reason Code corresponds to a Topic Filter in the UNSUBSCRIBE packet being acknowledged.
///
/// The Server sending an UNSUBACK packet MUST use one of the Unsubscribe Reason Code
/// values for each Topic Filter received [MQTT-3.11.3-2].
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsubscribeReasonCode {
    /// The subscription is deleted.
    Success = 0x00,

    /// No matching Topic Filter is being used by the Client.
    NoSubscriptionExisted = 0x11,

    /// The unsubscribe could not be completed and the Server either does not
    /// wish to reveal the reason or none of the other Reason Codes apply.
    UnspecifiedError = 0x80,

    /// The UNSUBSCRIBE is valid but the Server does not accept it.
    ImplementationSpecificError = 0x83,

    /// The Client is not authorized to unsubscribe.
    NotAuthorized = 0x87,

    /// The Topic Filter is correctly formed but is not allowed for this Client.
    TopicFilterInvalid = 0x8f,

    /// The specified Packet Identifier is already in use.
    PacketIdentifierInUse = 0x91,
}

impl Default for UnsubscribeReasonCode {
    fn default() -> Self {
        Self::Success
    }
}

impl UnsubscribeReasonCode {
    pub fn bytes(&self) -> usize {
        1
    }

    pub fn const_bytes() -> usize {
        1
    }
}

impl TryFrom<u8> for UnsubscribeReasonCode {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(Self::Success),
            0x11 => Ok(Self::NoSubscriptionExisted),
            0x80 => Ok(Self::UnspecifiedError),
            0x83 => Ok(Self::ImplementationSpecificError),
            0x87 => Ok(Self::NotAuthorized),
            0x8f => Ok(Self::TopicFilterInvalid),
            0x91 => Ok(Self::PacketIdentifierInUse),
            _ => Err(DecodeError::OtherErrors),
        }
    }
}

impl DecodePacket for UnsubscribeReasonCode {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let byte = ba.read_byte()?;
        let flag = Self::try_from(byte)?;
        Ok(flag)
    }
}

impl EncodePacket for UnsubscribeReasonCode {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.push(*self as u8);
        Ok(self.bytes())
    }
}

/// UnsubscribeAck packet is sent by the Server to the Client to confirm receipt of an
/// Unsubscribe packet.
///
/// Basic struct of packet:
/// ```txt
///  7                       0
/// +-------------------------+
/// | Fixed header            |
/// |                         |
/// +-------------------------+
/// | Packet id               |
/// |                         |
/// +-------------------------+
/// | Properties ...          |
/// +-------------------------+
/// | Reason Code ...         |
/// +-------------------------+
/// ```
///
/// Note that this packet does not contain payload message.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UnsubscribeAckPacket {
    /// `packet_id` field is read from Unsubscribe packet.
    packet_id: PacketId,

    properties: Properties,

    /// The order of Reason Codes in the UNSUBACK packet MUST match the order of
    /// Topic Filters in the UNSUBSCRIBE packet [MQTT-3.11.3-1].
    reasons: Vec<UnsubscribeReasonCode>,
}

impl UnsubscribeAckPacket {
    pub fn new(packet_id: PacketId, reason: UnsubscribeReasonCode) -> Self {
        Self {
            packet_id,
            properties: Properties::new(),
            reasons: vec![reason],
        }
    }

    pub fn with_vec(packet_id: PacketId, reasons: Vec<UnsubscribeReasonCode>) -> Self {
        Self {
            packet_id,
            properties: Properties::new(),
            reasons,
        }
    }

    pub fn set_packet_id(&mut self, packet_id: PacketId) {
        self.packet_id = packet_id;
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

    pub fn reasons_mut(&mut self) -> &mut Vec<UnsubscribeReasonCode> {
        &mut self.reasons
    }

    pub fn reasons(&self) -> &[UnsubscribeReasonCode] {
        &self.reasons
    }
}

pub const UNSUBSCRIBE_ACK_PROPERTIES: &[PropertyType] = &[
    // The Server MUST NOT send this Property if it would increase the size of
    // the UNSUBACK packet beyond the Maximum Packet Size specified by the Client [MQTT-3.11.2-1].
    PropertyType::ReasonString,
    // The Server MUST NOT send this property if it would increase the size of the UNSUBACK
    // packet beyond the Maximum Packet Size specified by the Client [MQTT-3.11.2-2].
    PropertyType::UserProperty,
];

impl DecodePacket for UnsubscribeAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<UnsubscribeAckPacket, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::UnsubscribeAck {
            return Err(DecodeError::InvalidPacketType);
        }

        let packet_id = PacketId::decode(ba)?;
        let properties = if fixed_header.remaining_length() > packet_id.bytes() {
            let properties = Properties::decode(ba)?;
            if let Err(property_type) =
                check_property_type_list(properties.props(), UNSUBSCRIBE_ACK_PROPERTIES)
            {
                log::error!(
                    "v5/UnsubscribeAckPacket: property type {:?} cannot be used in properties!",
                    property_type
                );
                return Err(DecodeError::InvalidPropertyType);
            }
            properties
        } else {
            Properties::new()
        };

        let mut reasons = Vec::new();
        let mut remaining_length = packet_id.bytes() + properties.bytes();

        while remaining_length < fixed_header.remaining_length() {
            let reason = UnsubscribeReasonCode::decode(ba)?;
            reasons.push(reason);
            remaining_length += reason.bytes();
        }

        Ok(UnsubscribeAckPacket {
            packet_id,
            properties,
            reasons,
        })
    }
}

impl EncodePacket for UnsubscribeAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();
        let remaining_length = self.packet_id.bytes()
            + self.properties.bytes()
            + self.reasons.len() * UnsubscribeReasonCode::const_bytes();
        let fixed_header = FixedHeader::new(PacketType::UnsubscribeAck, remaining_length)?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        self.properties.encode(buf)?;

        for reason in &self.reasons {
            reason.encode(buf)?;
        }

        Ok(buf.len() - old_len)
    }
}

impl Packet for UnsubscribeAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::UnsubscribeAck
    }
}
