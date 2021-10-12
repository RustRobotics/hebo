// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType, ReasonCode};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

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
    reasons: Vec<ReasonCode>,
}

impl UnsubscribeAckPacket {
    pub fn new(packet_id: PacketId, reason: ReasonCode) -> Self {
        Self {
            packet_id,
            properties: Properties::new(),
            reasons: vec![reason],
        }
    }

    pub fn with_vec(packet_id: PacketId, reasons: Vec<ReasonCode>) -> Self {
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

    pub fn reasons_mut(&mut self) -> &mut Vec<ReasonCode> {
        &mut self.reasons
    }

    pub fn reasons(&self) -> &[ReasonCode] {
        &self.reasons
    }
}

/// Each Reason Code corresponds to a Topic Filter in the UNSUBSCRIBE packet being acknowledged.
///
/// The Server sending an UNSUBACK packet MUST use one of the Unsubscribe Reason Code
/// values for each Topic Filter received [MQTT-3.11.3-2].
pub const UNSUBSCRIBE_REASON_CODE_LIST: &[ReasonCode] = &[
    ReasonCode::Success,
    ReasonCode::NoSubscriptionExisted,
    ReasonCode::UnspecifiedError,
    ReasonCode::ImplementationSpecificError,
    ReasonCode::NotAuthorized,
    ReasonCode::TopicFilterInvalid,
    ReasonCode::PacketIdentifierInUse,
];

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
            let reason = ReasonCode::decode(ba)?;
            if !UNSUBSCRIBE_REASON_CODE_LIST.contains(&reason) {
                log::error!("Invalid reason code: {:?}", reason);
                return Err(DecodeError::InvalidReasonCode);
            }
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
            + self.reasons.len() * ReasonCode::const_bytes();
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
