// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// Text fields within the MQTT Control Packets described later are encoded as UTF-8 strings.
/// UTF-8 [RFC3629] is an efficient encoding of Unicode [Unicode] characters that
/// optimizes the encoding of ASCII characters in support of text-based communications.
///
/// String Data is represented by a Two Byte Integer length which indicates
/// the number of data bytes, followed by characters.
///
/// Thus, the length of String Data is limited to the range of 0 to 65,535 Bytes.
///
/// ```txt
/// +-------------------+
/// | String Length     |
/// |                   |
/// +-------------------+
/// | String            |
/// |                   |
/// +-------------------+
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StringData(String);

impl StringData {
    pub fn new(data: &str) -> Result<Self, EncodeError> {
        if data.len() > StringData::max() {
            return Err(EncodeError::TooManyData);
        }
        Ok(Self(data.to_string()))
    }

    pub fn max() -> usize {
        u16::MAX as usize
    }
}

impl AsRef<str> for StringData {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<String> for StringData {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl DecodePacket for StringData {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let len = ba.read_u16()?;
        let s = ba.read_string(len as usize)?;
        Ok(Self(s))
    }
}

impl EncodePacket for StringData {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.write_u16::<BigEndian>(self.0.len() as u16)?;
        buf.write_all(self.0.as_bytes())?;
        Ok(2 + self.0.len())
    }
}
