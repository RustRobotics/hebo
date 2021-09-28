// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// BoolData represents one byte value with two states.
pub type BoolData = bool;

impl DecodePacket for BoolData {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let byte = ba.read_byte()?;
        match byte {
            0x00 => Ok(false),
            0x01 => Ok(true),
            _ => Err(DecodeError::InvalidBoolData),
        }
    }
}

impl EncodePacket for BoolData {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let byte = if *self { 0x01 } else { 0x00 };
        buf.push(byte);
        Ok(1)
    }
}
