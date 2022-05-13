// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::cmp;
use std::convert;
use std::fmt;
use std::ops;

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// `U16Data` represents a two bytes integer.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U16Data(u16);

impl U16Data {
    /// Create a new `U16Data`.
    #[must_use]
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Get inner u16 value.
    #[must_use]
    pub const fn value(&self) -> u16 {
        self.0
    }

    /// Get byte length in packet.
    #[must_use]
    #[inline]
    pub const fn bytes() -> usize {
        2
    }
}

impl fmt::Display for U16Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DecodePacket for U16Data {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let value = ba.read_u16()?;
        Ok(Self(value))
    }
}

impl EncodePacket for U16Data {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.write_u16::<BigEndian>(self.0)?;
        Ok(Self::bytes())
    }
}

impl ops::AddAssign<u16> for U16Data {
    fn add_assign(&mut self, value: u16) {
        self.0 += value;
    }
}

impl cmp::PartialEq<u16> for U16Data {
    fn eq(&self, value: &u16) -> bool {
        self.0 == *value
    }
}

impl convert::From<u16> for U16Data {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}
