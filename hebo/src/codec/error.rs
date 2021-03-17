// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

#[derive(Debug)]
enum Error {
    TcpConnectError,
    PacketEmpty,
}

pub enum DecodeError {
    /// ClientId is empty or its length exceeds 23.
    /// Or contains invalid characters.
    InvalidClientId,

    InvalidPacketType,

    /// Protocol level is not in `3.1`, `3.1.1` or `5.0`.
    InvalidProtocolLevel,

    /// Protocol name must be "MQTT".
    InvalidProtocolName,

    // QoS not 0, 1, 2
    InvalidQoS,

    /// Length of buffer - offset < remaining length.
    InvalidRemainingLength,

    /// Invalid UTF-8 string.
    InvalidString,

    /// Violate topic filter rules.
    InvalidTopicFilter,

    /// No topic is speicified in Subscribe packet.
    EmptyTopic,

    /// Topic name might contain wildcard characters.
    InvalidTopicName,

    OutOfRangeError,

    /// Length of data exceeds its limitation
    TooManyData,
}

pub enum EncodeError {
    InvalidData,

    IoError,

    /// Length of data exceeds its limitation
    TooManyData,
}

pub enum ClientIdError {
    InvalidLength,
    InvalidChar,
}

impl From<std::io::Error> for EncodeError {
    fn from(_e: std::io::Error) -> EncodeError {
        EncodeError::IoError
    }
}

impl From<std::string::FromUtf8Error> for DecodeError {
    fn from(_e: std::string::FromUtf8Error) -> DecodeError {
        DecodeError::InvalidString
    }
}
