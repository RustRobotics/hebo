// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use crate::base::PROTOCOL_NAME;
use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, PacketType,
    StringData,
};

/// Current version of MQTT protocol can be:
/// * 3.1
/// * 3.1.1
/// * 5.0
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtocolLevel {
    /// MQTT 3.1
    V3 = 3,

    /// MQTT 3.1.1
    #[default]
    V4 = 4,

    /// MQTT 5.0
    V5 = 5,
}

impl ProtocolLevel {
    /// Get byte length in packet.
    #[must_use]
    #[inline]
    pub const fn bytes() -> usize {
        1
    }
}

impl TryFrom<u8> for ProtocolLevel {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            3 => Ok(Self::V3),
            4 => Ok(Self::V4),
            5 => Ok(Self::V5),

            _ => Err(DecodeError::InvalidProtocolLevel),
        }
    }
}

impl EncodePacket for ProtocolLevel {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        v.push(*self as u8);
        Ok(Self::bytes())
    }
}

impl DecodePacket for ProtocolLevel {
    /// Only parse protocol level in byte array.
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Connect {
            return Err(DecodeError::InvalidPacketType);
        }

        let protocol_name = StringData::decode(ba)?;
        if protocol_name.as_ref() != PROTOCOL_NAME {
            return Err(DecodeError::InvalidProtocolName);
        }

        let protocol_level = Self::try_from(ba.read_byte()?)?;
        Ok(protocol_level)
    }
}
