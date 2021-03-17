// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use super::DecodeError;

pub struct ByteArray<'a> {
    offset: usize,
    data: &'a [u8],
}

impl<'a> ByteArray<'a> {
    pub fn new(data: &'a [u8], offset: usize) -> Self {
        assert!(offset < data.len());
        ByteArray { offset, data }
    }

    // TODO(Shaohua): Add ByteArrayError
    pub fn one_byte(&mut self) -> Result<u8, DecodeError> {
        self.offset += 1;
        if self.offset >= self.data.len() {
            Err(DecodeError::OutOfRangeError)
        } else {
            Ok(self.data[self.offset - 1])
        }
    }
}
