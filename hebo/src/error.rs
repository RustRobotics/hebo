// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    EncodeError(codec::EncodeError),
    DecodeError(codec::DecodeError),
    SendError,
    SocketError,
    CertError,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<codec::EncodeError> for Error {
    fn from(err: codec::EncodeError) -> Self {
        Error::EncodeError(err)
    }
}

impl From<codec::DecodeError> for Error {
    fn from(err: codec::DecodeError) -> Self {
        Error::DecodeError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
