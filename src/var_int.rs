// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::{consts, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VarInt(usize);

impl VarInt {
    pub fn new(len: usize) -> Result<Self, EncodeError> {
        if len as usize > consts::MAX_PACKET_LEN {
            return Err(EncodeError::TooManyData);
        }
        Ok(Self(len))
    }

    pub fn len(&self) -> usize {
        self.0
    }

    pub fn bytes(&self) -> usize {
        if self.0 > 0x7f_ff_ff {
            3
        } else if self.0 > 0x7f_ff {
            3
        } else if self.0 > 0x7f {
            2
        } else {
            1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl DecodePacket for VarInt {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let mut byte: usize;
        let mut remaining_length: usize = 0;
        let mut multiplier = 1;

        // Read variant length
        loop {
            byte = ba.read_byte()? as usize;
            remaining_length += (byte & 127) * multiplier;
            multiplier *= 128;

            if multiplier > 128 * 128 * 128 * 128 {
                return Err(DecodeError::InvalidVarInt);
            }

            if (byte & 128) == 0 {
                break;
            }
        }

        // Sometimes we only receive header part of packet and decide
        // whether to prevent from sending more bytes.
        if ba.remaining_bytes() < remaining_length as usize {
            Err(DecodeError::InvalidVarInt)
        } else {
            Ok(VarInt(remaining_length))
        }
    }
}

impl EncodePacket for VarInt {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        if self.0 == 0 {
            buf.push(0);
            return Ok(1);
        }

        let mut n = self.0;
        let mut count = 0;
        // TODO(Shaohua): Simplify
        while n > 0 {
            let mut m = n % 128;
            count += 1;
            n /= 128;
            if n > 0 {
                m |= 128;
            }
            buf.push(m as u8);
        }
        Ok(count)
    }
}
