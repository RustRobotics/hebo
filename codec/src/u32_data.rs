// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::fmt;

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// `U32Data` represents a four bytes integer.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U32Data(u32);

impl U32Data {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Get u32 value.
    #[must_use]
    pub const fn value(&self) -> u32 {
        self.0
    }

    /// Get byte length in packet.
    #[must_use]
    pub const fn bytes() -> usize {
        4
    }
}

impl fmt::Display for U32Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DecodePacket for U32Data {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let value = ba.read_u32()?;
        Ok(Self(value))
    }
}

impl EncodePacket for U32Data {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.write_u32::<BigEndian>(self.0)?;
        Ok(Self::bytes())
    }
}
