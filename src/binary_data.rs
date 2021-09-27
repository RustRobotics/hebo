// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

use crate::{utils, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// Binary Data is represented by a Two Byte Integer length which indicates
/// the number of data bytes, followed by that number of bytes.
///
/// Thus, the length of Binary Data is limited to the range of 0 to 65,535 Bytes.
/// ```text
/// +-------------------+
/// | Binary Length     |
/// |                   |
/// +-------------------+
/// | Bytes             |
/// |                   |
/// +-------------------+
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BinaryData(Vec<u8>);

impl BinaryData {
    pub fn new(data: &[u8]) -> Result<Self, EncodeError> {
        utils::validate_two_bytes_data(data)?;
        Ok(Self(data.to_vec()))
    }
}

impl AsRef<[u8]> for BinaryData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<Vec<u8>> for BinaryData {
    fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl DecodePacket for BinaryData {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let len = ba.read_u16()?;
        let data = ba.read_bytes(len as usize)?;
        Ok(Self(data.to_vec()))
    }
}

impl EncodePacket for BinaryData {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.write_u16::<BigEndian>(self.0.len() as u16)?;
        buf.write_all(&self.0)?;
        Ok(2 + self.0.len())
    }
}
