// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
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
/// ```
///
/// Note that this packet does not contain payload message.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UnsubscribeAckPacket {
    /// `packet_id` field is read from Unsubscribe packet.
    packet_id: PacketId,

    properties: Properties,
}

impl UnsubscribeAckPacket {
    pub fn new(packet_id: PacketId) -> Self {
        Self {
            packet_id,
            properties: Properties::new(),
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
        Ok(UnsubscribeAckPacket {
            packet_id,
            properties,
        })
    }
}

impl EncodePacket for UnsubscribeAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();
        let remaining_length = self.packet_id.bytes() + self.properties.bytes();
        let fixed_header = FixedHeader::new(PacketType::UnsubscribeAck, remaining_length)?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        self.properties.encode(buf)?;
        Ok(buf.len() - old_len)
    }
}

impl Packet for UnsubscribeAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::UnsubscribeAck
    }
}
