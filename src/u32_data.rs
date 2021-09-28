// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// U32Data represents a four bytes integer.
pub type U32Data = u32;

impl DecodePacket for U32Data {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let value = ba.read_u32()?;
        Ok(value)
    }
}

impl EncodePacket for U32Data {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.write_u32::<BigEndian>(*self)?;
        Ok(4)
    }
}
