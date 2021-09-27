// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use crate::{DecodeError, EncodeError, EncodePacket};

/// Current version of MQTT protocol can be:
/// * 3.1
/// * 3.1.1
/// * 5.0
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtocolLevel {
    V31 = 3,
    V311 = 4,
    V5 = 5,
}

impl Default for ProtocolLevel {
    fn default() -> Self {
        ProtocolLevel::V311
    }
}

impl TryFrom<u8> for ProtocolLevel {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<ProtocolLevel, Self::Error> {
        match v {
            3 => Ok(ProtocolLevel::V31),
            4 => Ok(ProtocolLevel::V311),
            5 => Ok(ProtocolLevel::V5),

            _ => Err(DecodeError::InvalidProtocolLevel),
        }
    }
}

impl EncodePacket for ProtocolLevel {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        v.push(*self as u8);
        Ok(1)
    }
}
