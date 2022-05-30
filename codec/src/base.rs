// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use serde::Deserialize;
use std::convert::TryFrom;

use super::{ByteArray, DecodeError, EncodeError};

pub const PROTOCOL_NAME: &str = "MQTT";
pub const PROTOCOL_NAME_V3: &str = "MQIsdp";

/// Convert native data types to network byte stream.
pub trait EncodePacket {
    /// Encode packets into byte array.
    ///
    /// # Errors
    ///
    /// Returns error if packet state is invalid of buffer capacity is insufficient.
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError>;
}

pub trait DecodePacket: Sized {
    /// Decode byte array into a mqtt packet.
    ///
    /// # Errors
    ///
    /// Returns error if byte array size or packet state is invalid.
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError>;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QoS {
    /// At most once delivery.
    AtMostOnce = 0,

    /// At least once delivery.
    AtLeastOnce = 1,

    /// Exactly once delivery.
    ExactOnce = 2,
}

impl QoS {
    /// Byte length used in packet.
    #[must_use]
    pub const fn bytes() -> usize {
        1
    }
}

impl Default for QoS {
    fn default() -> Self {
        Self::AtMostOnce
    }
}

impl TryFrom<u8> for QoS {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::AtMostOnce),
            1 => Ok(Self::AtLeastOnce),
            2 => Ok(Self::ExactOnce),
            _ => Err(DecodeError::InvalidQoS),
        }
    }
}

impl EncodePacket for QoS {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        v.push(*self as u8);
        Ok(Self::bytes())
    }
}

impl DecodePacket for QoS {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let byte = ba.read_byte()?;
        let qos = Self::try_from(byte)?;
        Ok(qos)
    }
}
