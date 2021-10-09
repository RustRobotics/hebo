// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// Byte 0 in the Variable Header is the Authenticate Reason Code.
///
/// The values for the one byte unsigned Authenticate Reason Code field are shown below.
///
/// The sender of the AUTH Packet MUST use one of the Authenticate Reason Codes [MQTT-3.15.2-1].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthReasonCode {
    /// Authentication is successful
    ///
    /// Sent by server.
    Success = 0x00,

    /// Continue the authentication with another step.
    ///
    /// Sent by client or server.
    ContinueAuthentication = 0x18,

    /// Initiate a re-authentication
    ///
    /// Sent by client.
    ReAuthenticate = 0x19,
}

impl AuthReasonCode {
    pub fn bytes(&self) -> usize {
        1
    }
    pub fn const_bytes() -> usize {
        1
    }
}

impl Default for AuthReasonCode {
    fn default() -> Self {
        Self::Success
    }
}

impl TryFrom<u8> for AuthReasonCode {
    type Error = DecodeError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(Self::Success),
            0x18 => Ok(Self::ContinueAuthentication),
            0x19 => Ok(Self::ReAuthenticate),
            _ => Err(DecodeError::OtherErrors),
        }
    }
}

impl DecodePacket for AuthReasonCode {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let byte = ba.read_byte()?;
        let flag = Self::try_from(byte)?;
        Ok(flag)
    }
}

impl EncodePacket for AuthReasonCode {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.push(*self as u8);
        Ok(self.bytes())
    }
}

/// An AUTH packet is sent from Client to Server or Server to Client
/// as part of an extended authentication exchange, such as challenge / response authentication.
///
/// It is a Protocol Error for the Client or Server to send an AUTH packet if the CONNECT packet
/// did not contain the same Authentication Method.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuthPacket {
    reason_code: AuthReasonCode,
    properties: Properties,
}

impl AuthPacket {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_reason_code(&mut self, code: AuthReasonCode) -> &mut Self {
        self.reason_code = code;
        self
    }

    pub fn reason_code(&self) -> AuthReasonCode {
        self.reason_code
    }

    pub fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }
}

impl EncodePacket for AuthPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let remaining_length = self.reason_code.bytes() + self.properties.bytes();
        let fixed_header = FixedHeader::new(PacketType::PingRequest, remaining_length)?;
        fixed_header.encode(buf)?;
        self.reason_code.encode(buf)?;
        self.properties.encode(buf)?;

        Ok(buf.len() - old_len)
    }
}

impl Packet for AuthPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Auth
    }
}

impl DecodePacket for AuthPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Auth {
            return Err(DecodeError::InvalidPacketType);
        }
        if fixed_header.remaining_length() == 0 {
            return Ok(AuthPacket::default());
        }

        Ok(AuthPacket::new())
    }
}
