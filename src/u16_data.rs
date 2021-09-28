// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// U16Data represents a two bytes integer.
pub type U16Data = u16;

impl DecodePacket for U16Data {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let value = ba.read_u16()?;
        Ok(value)
    }
}

impl EncodePacket for U16Data {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.write_u16::<BigEndian>(*self)?;
        Ok(4)
    }
}
