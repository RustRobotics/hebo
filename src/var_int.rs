// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::{consts, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// The Variable Byte Integer is encoded using an encoding scheme which uses a single byte
/// for values up to 127.
///
/// Larger values are handled as follows. The least significant seven bits of each byte
/// encode the data, and the most significant bit is used to indicate whether there are bytes
/// following in the representation. Thus, each byte encodes 128 values and a "continuation bit".
/// The maximum number of bytes in the Variable Byte Integer field is four.  The encoded value
/// MUST use the minimum number of bytes necessary to represent the value [MQTT-1.5.5-1].
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_int_encode() {
        let mut buf = Vec::with_capacity(4);

        let remaining_len = VarInt(126);
        let _ = remaining_len.encode(&mut buf);
        assert_eq!(&buf, &[0x7e]);
        buf.clear();

        let remaining_len = VarInt(146);
        let _ = remaining_len.encode(&mut buf);
        assert_eq!(&buf, &[0x92, 0x01]);
        buf.clear();

        let remaining_len = VarInt(16_385);
        let _ret = remaining_len.encode(&mut buf);
        assert_eq!(&buf, &[0x81, 0x80, 0x01]);
        buf.clear();

        let remaining_len = VarInt(2_097_152);
        let _ret = remaining_len.encode(&mut buf);
        assert_eq!(&buf, &[0x80, 0x80, 0x80, 0x01]);
        buf.clear();
    }

    #[test]
    fn test_var_int_decode() {
        let buf = [0x7e];
        let mut ba = ByteArray::new(&buf);
        let ret = VarInt::decode(&mut ba);
        assert_eq!(ret.unwrap().0, 126);

        let buf = [0x92, 0x01];
        let mut ba = ByteArray::new(&buf);
        let ret = VarInt::decode(&mut ba);
        assert_eq!(ret.unwrap().0, 146);

        let buf = [0x81, 0x80, 0x01];
        let mut ba = ByteArray::new(&buf);
        let ret = VarInt::decode(&mut ba);
        assert_eq!(ret.unwrap().0, 16_385);

        let buf = [0x81, 0x80, 0x80, 0x01];
        let mut ba = ByteArray::new(&buf);
        let ret = VarInt::decode(&mut ba);
        assert_eq!(ret.unwrap().0, 2_097_153);

        let buf = [0xff, 0xff, 0xff, 0x7f];
        let mut ba = ByteArray::new(&buf);
        let ret = VarInt::decode(&mut ba);
        assert_eq!(ret.unwrap().0, 268_435_455);
    }
}
