// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType, ReasonCode};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, VarIntError};

/// An AUTH packet is sent from Client to Server or Server to Client
/// as part of an extended authentication exchange, such as challenge / response authentication.
///
/// It is a Protocol Error for the Client or Server to send an AUTH packet if the CONNECT packet
/// did not contain the same Authentication Method.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuthPacket {
    reason_code: ReasonCode,
    properties: Properties,
}

impl AuthPacket {
    /// Create a new auth packet with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update reason code.
    pub fn set_reason_code(&mut self, code: ReasonCode) -> &mut Self {
        self.reason_code = code;
        self
    }

    /// Get reason code.
    #[must_use]
    pub const fn reason_code(&self) -> ReasonCode {
        self.reason_code
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
}

/// Byte 0 in the Variable Header is the Authenticate Reason Code.
///
/// The values for the one byte unsigned Authenticate Reason Code field are shown below.
///
/// The sender of the AUTH Packet MUST use one of the Authenticate Reason Codes [MQTT-3.15.2-1].
pub const AUTH_REASONS: &[ReasonCode] = &[
    ReasonCode::Success,
    ReasonCode::ContinueAuthentication,
    ReasonCode::ReAuthenticate,
];

pub const AUTH_PROPERTIES: &[PropertyType] = &[
    PropertyType::AuthenticationMethod,
    PropertyType::AuthenticationData,
    // The sender MUST NOT send this property if it would increase the size of
    // the AUTH packet beyond the Maximum Packet Size specified by the receiver [MQTT-3.15.2-2].
    PropertyType::ReasonString,
    // The sender MUST NOT send this property if it would increase the size of the AUTH packet
    // beyond the Maximum Packet Size specified by the receiver [MQTT-3.15.2-3].
    PropertyType::UserProperty,
];

impl EncodePacket for AuthPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let remaining_length = ReasonCode::bytes() + self.properties.bytes();
        let fixed_header = FixedHeader::new(PacketType::PingRequest, remaining_length)?;
        fixed_header.encode(buf)?;
        self.reason_code.encode(buf)?;
        self.properties.encode(buf)?;

        Ok(buf.len() - old_len)
    }
}

impl DecodePacket for AuthPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Auth {
            return Err(DecodeError::InvalidPacketType);
        }
        if fixed_header.remaining_length() == 0 {
            return Ok(Self::default());
        }

        let reason_code = ReasonCode::decode(ba)?;
        if !AUTH_REASONS.contains(&reason_code) {
            log::error!("Invalid reason code: {:?}", reason_code);
            return Err(DecodeError::InvalidReasonCode);
        }

        let properties = Properties::decode(ba)?;
        if let Err(property_type) = check_property_type_list(properties.props(), AUTH_PROPERTIES) {
            log::error!(
                "v5/AuthPacket: property type {:?} cannot be used in properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        Ok(Self {
            reason_code,
            properties,
        })
    }
}

impl Packet for AuthPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Auth
    }

    fn bytes(&self) -> Result<usize, VarIntError> {
        let remaining_length = ReasonCode::bytes() + self.properties.bytes();
        let fixed_header = FixedHeader::new(PacketType::PingRequest, remaining_length)?;

        Ok(fixed_header.bytes() + remaining_length)
    }
}
