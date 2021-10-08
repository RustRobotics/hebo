// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, PropertyType, ShortProperties};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishCompleteReasonCode {
    /// Message released.
    Success = 0x00,

    /// The Packet Identifier is not known.
    ///
    /// This is not an error during recovery, but at other times indicates a mismatch
    /// between the Session State on the Client and Server.
    PacketIdentifierNotFound = 0x92,
}

impl Default for PublishCompleteReasonCode {
    fn default() -> Self {
        Self::Success
    }
}

impl TryFrom<u8> for PublishCompleteReasonCode {
    type Error = DecodeError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(Self::Success),
            0x92 => Ok(Self::PacketIdentifierNotFound),
            _ => Err(DecodeError::OtherErrors),
        }
    }
}

impl PublishCompleteReasonCode {
    pub fn bytes(&self) -> usize {
        1
    }

    pub fn const_bytes() -> usize {
        1
    }
}

/// Response to a Publish packet with QoS 2. It is the fourth and final packet of
/// the QoS 2 protocol exchange.
///
/// Packet structre is:
/// ```txt
///  7                     0
/// +-----------------------+
/// | Fixed header          |
/// |                       |
/// +-----------------------+
/// | Packet id             |
/// |                       |
/// +-----------------------+
/// | Reason Code           |
/// +-----------------------+
/// | Property Length       |
/// +-----------------------+
/// | Properties ...        |
/// +-----------------------+
/// ```
///
/// This packet does not contain payload part.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PublishCompletePacket {
    packet_id: PacketId,
    reason_code: PublishCompleteReasonCode,
    properties: ShortProperties,
}

impl PublishCompletePacket {
    pub fn new(packet_id: PacketId) -> Self {
        Self {
            packet_id,
            ..Self::default()
        }
    }

    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn set_reason_code(&mut self, reason_code: PublishCompleteReasonCode) -> &mut Self {
        self.reason_code = reason_code;
        self
    }

    pub fn reason_code(&self) -> PublishCompleteReasonCode {
        self.reason_code
    }

    pub fn properties(&self) -> &ShortProperties {
        &self.properties
    }

    pub fn mut_properties(&mut self) -> &mut ShortProperties {
        &mut self.properties
    }
}

pub const PUBLISH_COMPLETE_PROPERTIES: &[PropertyType] =
    &[PropertyType::ReasonString, PropertyType::UserProperty];

impl EncodePacket for PublishCompletePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let mut packet_bytes = self.packet_id.bytes();
        if self.reason_code != PublishCompleteReasonCode::Success || !self.properties.is_empty() {
            packet_bytes += self.reason_code.bytes();
        }
        if !self.properties.is_empty() {
            packet_bytes += self.properties.bytes();
        }
        let fixed_header = FixedHeader::new(PacketType::PublishComplete, packet_bytes)?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        if self.reason_code != PublishCompleteReasonCode::Success || !self.properties.is_empty() {
            buf.push(self.reason_code as u8);
        }
        if !self.properties.is_empty() {
            self.properties.encode(buf)?;
        }
        Ok(buf.len() - old_len)
    }
}

impl Packet for PublishCompletePacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PublishComplete
    }
}

impl DecodePacket for PublishCompletePacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PublishComplete {
            return Err(DecodeError::InvalidPacketType);
        }
        if fixed_header.remaining_length() < PacketId::const_bytes() {
            return Err(DecodeError::InvalidRemainingLength);
        }
        let packet_id = PacketId::decode(ba)?;
        let remaining_length = fixed_header.remaining_length() - packet_id.bytes();
        let reason_code = if remaining_length >= PublishCompleteReasonCode::const_bytes() {
            let reason_code_byte = ba.read_byte()?;
            PublishCompleteReasonCode::try_from(reason_code_byte)?
        } else {
            PublishCompleteReasonCode::default()
        };
        let properties = if remaining_length > PublishCompleteReasonCode::const_bytes() {
            let properties = ShortProperties::decode(ba)?;
            if let Err(property_type) =
                check_property_type_list(properties.props(), PUBLISH_COMPLETE_PROPERTIES)
            {
                log::error!(
                    "v5/PublishReleasePacket: property type {:?} cannot be used in properties!",
                    property_type
                );
                return Err(DecodeError::InvalidPropertyType);
            }
            properties
        } else {
            ShortProperties::new()
        };
        Ok(PublishCompletePacket {
            packet_id,
            reason_code,
            properties,
        })
    }
}
