// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    CodecError(codec::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<codec::Error> for Error {
    fn from(err: codec::Error) -> Self {
        Error::CodecError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
