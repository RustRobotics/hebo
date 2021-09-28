// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// U16Data represents a two bytes integer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct U16Data(u16);

impl U16Data {
    pub fn new(value: u16) -> Self {
        U16Data(value)
    }
}

impl std::ops::Deref for U16Data {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for U16Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        Ok(4)
    }
}
