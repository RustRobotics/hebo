// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// `BoolData` represents one byte value with two states.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BoolData(bool);

impl BoolData {
    /// Create a new bool data.
    #[must_use]
    pub const fn new(value: bool) -> Self {
        Self(value)
    }

    /// Get inner boolean value.
    #[must_use]
    pub const fn value(&self) -> bool {
        self.0
    }

    /// Get byte length in packet.
    #[must_use]
    pub const fn bytes() -> usize {
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
        Ok(Self::bytes())
    }
}
