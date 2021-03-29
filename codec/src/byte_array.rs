// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, ByteOrder};

use super::utils;

#[derive(Debug)]
pub enum ByteArrayError {
    OutOfRangeError,
    InvalidString(utils::StringError),
}

impl From<utils::StringError> for ByteArrayError {
    fn from(e: utils::StringError) -> ByteArrayError {
        ByteArrayError::InvalidString(e)
    }
}

pub struct ByteArray<'a> {
    offset: usize,
    data: &'a [u8],
}

impl<'a> ByteArray<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ByteArray { offset: 0, data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn remaining_bytes(&self) -> usize {
        assert!(self.offset <= self.data.len());
        self.data.len() - self.offset
    }

    // TODO(Shaohua): Add ByteArrayError
    pub fn read_byte(&mut self) -> Result<u8, ByteArrayError> {
        self.offset += 1;
        if self.offset > self.data.len() {
            Err(ByteArrayError::OutOfRangeError)
        } else {
            Ok(self.data[self.offset - 1])
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, ByteArrayError> {
        Ok(BigEndian::read_u16(self.read_bytes(2)?))
    }

    pub fn read_string(&mut self, len: usize) -> Result<String, ByteArrayError> {
        let bytes = self.read_bytes(len)?;
        utils::to_utf8_string(bytes).map_err(ByteArrayError::from)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&[u8], ByteArrayError> {
        self.offset += len;
        if self.offset > self.data.len() {
            Err(ByteArrayError::OutOfRangeError)
        } else {
            Ok(&self.data[self.offset - len..self.offset])
        }
    }
}
