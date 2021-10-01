// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::convert::TryFrom;

use super::{ByteArray, DecodeError, EncodeError};

pub const PROTOCOL_NAME: &'static str = "MQTT";

/// Convert native data types to network byte stream.
pub trait EncodePacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError>;
}

pub trait DecodePacket: Sized {
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
    pub fn bytes(&self) -> usize {
        1
    }

    pub const fn const_bytes() -> usize {
        1
    }
}

impl Default for QoS {
    fn default() -> Self {
        QoS::AtMostOnce
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

impl Into<u8> for QoS {
    fn into(self) -> u8 {
        match self {
            Self::AtMostOnce => 0,
            Self::AtLeastOnce => 1,
            Self::ExactOnce => 2,
        }
    }
}

impl EncodePacket for QoS {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let byte: u8 = (*self).into();
        v.push(byte);
        Ok(self.bytes())
    }
}

impl DecodePacket for QoS {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let byte = ba.read_byte()?;
        let qos = Self::try_from(byte)?;
        Ok(qos)
    }
}
