// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// BoolData represents one byte value with two states.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BoolData(bool);

impl BoolData {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn value(&self) -> bool {
        self.0
    }

    pub fn bytes(&self) -> usize {
        1
    }

    pub const fn const_bytes() -> usize {
        1
    }
}

impl DecodePacket for BoolData {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let byte = ba.read_byte()?;
        match byte {
            0x00 => Ok(Self(false)),
            0x01 => Ok(Self(true)),
            _ => Err(DecodeError::InvalidBoolData),
        }
    }
}

impl EncodePacket for BoolData {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let byte = if self.0 { 0x01 } else { 0x00 };
        buf.push(byte);
        Ok(self.bytes())
    }
}
