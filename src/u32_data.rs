// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// U32Data represents a four byte integers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct U32Data(u32);

impl U32Data {
    pub fn new(value: u32) -> Self {
        U32Data(value)
    }
}

impl std::ops::Deref for U32Data {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for U32Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        Ok(4)
    }
}
