// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{Properties, PropertyType, ReasonCode};
use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, Packet, PacketId,
    PacketType, VarIntError,
};

/// Response to a Publish packet with `QoS` 2. It is the third packet of the `QoS` 2
/// protocol exchange.
///
/// Packet structre is:
/// ```txt
///  7                     0
/// +--------------------+
/// | Fixed header       |
/// |                    |
/// +--------------------+
/// | Packet id          |
/// |                    |
/// +--------------------+
/// | Reason Code        |
/// +--------------------+
/// | Property Length    |
/// +--------------------+
/// | Properties ...     |
/// +--------------------+
/// ```
///
/// This packet does not contain payload part.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PublishReleasePacket {
    packet_id: PacketId,
    reason_code: ReasonCode,
    properties: Properties,
}

impl PublishReleasePacket {
    /// Create a new publish release packet.
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

    fn get_fixed_header(&self) -> Result<FixedHeader, VarIntError> {
        let mut packet_bytes = PacketId::bytes();
        if self.reason_code != ReasonCode::Success || !self.properties.is_empty() {
            packet_bytes += ReasonCode::bytes();
        }
        if !self.properties.is_empty() {
            packet_bytes += self.properties.bytes();
        }
        FixedHeader::new(PacketType::PublishRelease, packet_bytes)
    }
}

/// Byte 3 in the Variable Header is the PUBREL Reason Code. If the Remaining Length is 2,
/// the value of 0x00 (Success) is used.
pub const PUBLISH_RELEASE_REASONS: &[ReasonCode] =
    &[ReasonCode::Success, ReasonCode::PacketIdentifierNotFound];

pub const PUBLISH_RELEASE_PROPERTIES: &[PropertyType] = &[
    // The sender MUST NOT send this Property if it would increase the size of the PUBREL packet
    // beyond the Maximum Packet Size specified by the receiver [MQTT-3.6.2-2]
    PropertyType::ReasonString,
    // The sender MUST NOT send this property if it would increase the size of the PUBREL packet
    // beyond the Maximum Packet Size specified by the receiver [MQTT-3.6.2-3]
    PropertyType::UserProperty,
];

impl EncodePacket for PublishReleasePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = self.get_fixed_header()?;
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

impl DecodePacket for PublishReleasePacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PublishRelease {
            return Err(DecodeError::InvalidPacketType);
        }
        if fixed_header.remaining_length() < PacketId::bytes() {
            return Err(DecodeError::InvalidRemainingLength);
        }
        let packet_id = PacketId::decode(ba)?;
        let remaining_length = fixed_header.remaining_length() - PacketId::bytes();
        let reason_code = if remaining_length >= ReasonCode::bytes() {
            ReasonCode::decode(ba)?
        } else {
            ReasonCode::default()
        };
        if !PUBLISH_RELEASE_REASONS.contains(&reason_code) {
            log::error!("Invalid reason code: {:?}", reason_code);
            return Err(DecodeError::InvalidReasonCode);
        }

        let properties = if remaining_length > ReasonCode::bytes() {
            let properties = Properties::decode(ba)?;
            if let Err(property_type) =
                check_property_type_list(properties.props(), PUBLISH_RELEASE_PROPERTIES)
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

impl Packet for PublishReleasePacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PublishRelease
    }

    fn bytes(&self) -> Result<usize, VarIntError> {
        let fixed_header = self.get_fixed_header()?;
        Ok(fixed_header.bytes() + fixed_header.remaining_length())
    }
}
