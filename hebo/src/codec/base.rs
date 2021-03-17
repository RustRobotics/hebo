// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::convert::TryFrom;
use std::io;

use super::{DecodeError, EncodeError};

/// Packet identifier
pub type PacketId = u16;

/// Convert native data types to network byte stream.
pub trait EncodePacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError>;
}

pub trait DecodePacket: Sized {
    fn decode(buf: &[u8], offset: &mut usize) -> Result<Self, DecodeError>;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QoS {
    /// At most once delivery.
    AtMostOnce = 0,

    /// At least once delivery.
    AtLeastOnce = 1,

    /// Exactly once delivery.
    ExactOnce = 2,
}

impl Default for QoS {
    fn default() -> Self {
        QoS::AtMostOnce
    }
}

impl TryFrom<u8> for QoS {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<QoS, Self::Error> {
        match v {
            0 => Ok(QoS::AtMostOnce),
            1 => Ok(QoS::AtLeastOnce),
            2 => Ok(QoS::ExactOnce),
            _ => Err(DecodeError::InvalidQoS),
        }
    }
}
