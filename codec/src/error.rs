// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::io;

use super::byte_array::ByteArrayError;
use super::topic::TopicError;
use super::utils::StringError;
use super::var_int::VarIntError;

#[allow(clippy::module_name_repetitions)]
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

    /// Resrved bit in connect packet is not zero.
    InvalidConnectFlags,

    /// QoS not 0, 1, 2
    InvalidQoS,

    /// Invalid flag value in fixed header.
    ///
    /// Does not contain InvalidQoS.
    InvalidPacketFlags,

    /// PacketId should be present but not set.
    /// Or PacketId is none where it is required.
    InvalidPacketId,

    /// Failed to parse variable byte integer.
    InvalidVarInt,

    InvalidBoolData,

    /// Length of buffer - offset < remaining length.
    // TODO(Shaohua): Replace with InvalidVarInt
    InvalidRemainingLength,

    /// Invalid UTF-8 string.
    InvalidString(StringError),

    /// Violate topic filter rules.
    /// Topic name might contain wildcard characters.
    InvalidTopic(TopicError),

    /// Unknown property type bit.
    InvalidPropertyType,

    /// Failed to parse property value.
    InvalidPropertyValue,

    /// Used in v5 protocol.
    InvalidReasonCode,

    /// Byte array index ouf of range.
    OutOfRangeError,

    /// Length of data exceeds its limitation
    TooManyData,

    /// No topic is speicified in Subscribe packet.
    EmptyTopicFilter,

    /// General errors
    OtherErrors,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum EncodeError {
    InvalidData,

    /// ClientId is empty or its length exceeds 23.
    /// Or contains invalid characters.
    InvalidClientId,

    IoError(io::Error),

    InvalidPacketType,

    InvalidPacketLevel,

    /// PacketId should be present but not set.
    /// Or PacketId is none where it is required.
    InvalidPacketId,

    /// Length of data exceeds its limitation
    TooManyData,

    InvalidString(StringError),

    /// Violate topic filter rules.
    /// No topic is speicified in Subscribe packet.
    /// Topic name might contain wildcard characters.
    InvalidTopic(TopicError),

    InvalidVarInt,

    /// Used in v5 protocol.
    InvalidReasonCode,
}

impl From<io::Error> for EncodeError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<StringError> for EncodeError {
    fn from(err: StringError) -> Self {
        Self::InvalidString(err)
    }
}

impl From<StringError> for DecodeError {
    fn from(err: StringError) -> Self {
        Self::InvalidString(err)
    }
}

impl From<TopicError> for EncodeError {
    fn from(err: TopicError) -> Self {
        Self::InvalidTopic(err)
    }
}

impl From<VarIntError> for EncodeError {
    fn from(_err: VarIntError) -> Self {
        Self::InvalidVarInt
    }
}

impl From<TopicError> for DecodeError {
    fn from(err: TopicError) -> Self {
        Self::InvalidTopic(err)
    }
}

impl From<ByteArrayError> for DecodeError {
    fn from(err: ByteArrayError) -> Self {
        match err {
            ByteArrayError::OutOfRangeError => Self::OutOfRangeError,
            ByteArrayError::InvalidString(err) => Self::InvalidString(err),
        }
    }
}

impl From<VarIntError> for DecodeError {
    fn from(_err: VarIntError) -> Self {
        // TODO(Shaohua): Add description
        Self::InvalidVarInt
    }
}
