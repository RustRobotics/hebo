// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use super::topic::TopicError;
use super::utils::StringError;

//enum Error {
//    TcpConnectError,
//    PacketEmpty,
//}

#[derive(Debug)]
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
    InvalidString(StringError),

    /// Violate topic filter rules.
    /// Topic name might contain wildcard characters.
    InvalidTopic(TopicError),

    OutOfRangeError,

    /// Length of data exceeds its limitation
    TooManyData,

    /// No topic is speicified in Subscribe packet.
    EmptyTopics,
}

#[derive(Debug)]
pub enum EncodeError {
    InvalidData,

    IoError,

    /// Length of data exceeds its limitation
    TooManyData,

    InvalidString(StringError),

    /// Violate topic filter rules.
    /// No topic is speicified in Subscribe packet.
    /// Topic name might contain wildcard characters.
    InvalidTopic(TopicError),
}

#[derive(Debug)]
pub enum ClientIdError {
    InvalidLength,
    InvalidChar,
}

impl From<std::io::Error> for EncodeError {
    fn from(_e: std::io::Error) -> EncodeError {
        EncodeError::IoError
    }
}

impl From<StringError> for EncodeError {
    fn from(e: StringError) -> EncodeError {
        EncodeError::InvalidString(e)
    }
}

impl From<StringError> for DecodeError {
    fn from(e: StringError) -> DecodeError {
        DecodeError::InvalidString(e)
    }
}

impl From<TopicError> for EncodeError {
    fn from(e: TopicError) -> EncodeError {
        EncodeError::InvalidTopic(e)
    }
}

impl From<TopicError> for DecodeError {
    fn from(e: TopicError) -> DecodeError {
        DecodeError::InvalidTopic(e)
    }
}
