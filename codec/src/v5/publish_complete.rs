// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType, ReasonCode};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

/// Response to a Publish packet with `QoS` 2.
///
/// It is the fourth and final packet of the `QoS` 2 protocol exchange.
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
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PublishCompletePacket {
    packet_id: PacketId,
    reason_code: ReasonCode,
    properties: Properties,
}

impl PublishCompletePacket {
    /// Create a new publish complete packet with specify `packet_id`.
    #[must_use]
    pub fn new(packet_id: PacketId) -> Self {
        Self {
            packet_id,
            ..Self::default()
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

    /// Update reason code.
    pub fn set_reason_code(&mut self, reason_code: ReasonCode) -> &mut Self {
        self.reason_code = reason_code;
        self
    }

    /// Get current reason code.
    #[must_use]
    pub const fn reason_code(&self) -> ReasonCode {
        self.reason_code
    }

    /// Get a reference to property list.
    #[must_use]
    pub const fn properties(&self) -> &Properties {
        &self.properties
    }

    /// Get a mutable reference to property list.
    pub fn mut_properties(&mut self) -> &mut Properties {
        &mut self.properties
    }
}

pub const PUBLISH_COMPLETE_REASONS: &[ReasonCode] =
    &[ReasonCode::Success, ReasonCode::PacketIdentifierNotFound];

pub const PUBLISH_COMPLETE_PROPERTIES: &[PropertyType] =
    &[PropertyType::ReasonString, PropertyType::UserProperty];

impl EncodePacket for PublishCompletePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let mut packet_bytes = self.packet_id.bytes();
        if self.reason_code != ReasonCode::Success || !self.properties.is_empty() {
            packet_bytes += ReasonCode::bytes();
        }
        if !self.properties.is_empty() {
            packet_bytes += self.properties.bytes();
        }
        let fixed_header = FixedHeader::new(PacketType::PublishComplete, packet_bytes)?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        if self.reason_code != ReasonCode::Success || !self.properties.is_empty() {
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
        let reason_code = if remaining_length >= ReasonCode::bytes() {
            ReasonCode::decode(ba)?
        } else {
            ReasonCode::default()
        };
        if !PUBLISH_COMPLETE_REASONS.contains(&reason_code) {
            log::error!("Invalid reason code: {:?}", reason_code);
            return Err(DecodeError::InvalidReasonCode);
        }

        let properties = if remaining_length > ReasonCode::bytes() {
            let properties = Properties::decode(ba)?;
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
            Properties::new()
        };
        Ok(Self {
            packet_id,
            reason_code,
            properties,
        })
    }
}
