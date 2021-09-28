// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// BoolData represents one byte value with two states.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoolData(bool);

impl BoolData {
    pub fn new(on: bool) -> Self {
        BoolData(on)
    }
}

impl std::ops::Deref for BoolData {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BoolData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        Ok(1)
    }
}
