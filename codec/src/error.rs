// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug)]
pub enum Error {
    TcpConnectError,
    InvalidFixedHeader,
    PacketEmpty,
    InvalidQoS,

    /// Protocol level is not in `3.1`, `3.1.1` or `5.0`.
    InvalidProtocolLevel,

    /// ClientId is empty or its length exceeds 23.
    /// Or contains invalid characters.
    InvalidClientId,

    /// Length of data exceeds its limitation
    TooManyData,

    /// Invalid UTF-8 string.
    InvalidString,

    /// Length of buffer - offset < remaining length.
    InvalidRemainingLength,
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_e: std::string::FromUtf8Error) -> Error {
        Error::InvalidString
    }
}
