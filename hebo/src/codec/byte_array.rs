// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use super::DecodeError;

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
        let remain = self.data.len() - self.offset;
        remain
    }

    // TODO(Shaohua): Add ByteArrayError
    pub fn read_byte(&mut self) -> Result<u8, DecodeError> {
        self.offset += 1;
        if self.offset > self.data.len() {
            Err(DecodeError::OutOfRangeError)
        } else {
            Ok(self.data[self.offset - 1])
        }
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&[u8], DecodeError> {
        self.offset += len;
        if self.offset > self.data.len() {
            Err(DecodeError::OutOfRangeError)
        } else {
            Ok(&self.data[self.offset - len..self.offset])
        }
    }
}
