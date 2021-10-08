// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

/// The Client or Server sending the PUBACK packet MUST use one of the PUBACK Reason Codes[MQTT-3.4.2-1].
///
/// The Reason Code and Property Length can be omitted if the Reason Code is 0x00 (Success)
/// and there are no Properties. In this case the PUBACK has a Remaining Length of 2.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishAckReasonCode {
    /// The message is accepted. Publication of the QoS 1 message proceeds.
    Success = 0x00,

    /// The message is accepted but there are no subscribers. This is sent only by the Server.
    /// If the Server knows that there are no matching subscribers, it MAY use this Reason Code
    /// instead of 0x00 (Success).
    NoMatchingSubscribers = 0x10,

    /// The receiver does not accept the publish but either does not want to reveal the reason,
    /// or it does not match one of the other values.
    UnspecifiedError = 0x80,

    /// The PUBLISH is valid but the receiver is not willing to accept it.
    ImplementationSpecificError = 0x83,

    /// The PUBLISH is not authorized.
    NotAuthorized = 0x87,

    /// The Topic Name is not malformed, but is not accepted by this Client or Server.
    TopicNameInvalid = 0x90,

    /// The Packet Identifier is already in use. This might indicate a mismatch
    /// in the Session State between the Client and Server.
    PacketIdentifierInUse = 0x91,

    /// An implementation or administrative imposed limit has been exceeded.
    QuotaExceeded = 0x97,

    /// The payload format does not match the specified Payload Format Indicator.
    PayloadFormatInvalid = 0x99,
}

impl Default for PublishAckReasonCode {
    fn default() -> Self {
        Self::Success
    }
}

impl TryFrom<u8> for PublishAckReasonCode {
    type Error = DecodeError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(Self::Success),
            0x10 => Ok(Self::NoMatchingSubscribers),
            0x80 => Ok(Self::UnspecifiedError),
            0x83 => Ok(Self::ImplementationSpecificError),
            0x87 => Ok(Self::NotAuthorized),
            0x90 => Ok(Self::TopicNameInvalid),
            0x91 => Ok(Self::PacketIdentifierInUse),
            0x97 => Ok(Self::QuotaExceeded),
            0x99 => Ok(Self::PayloadFormatInvalid),
            _ => Err(DecodeError::OtherErrors),
        }
    }
}

impl PublishAckReasonCode {
    pub fn bytes(&self) -> usize {
        1
    }

    pub fn const_bytes() -> usize {
        1
    }
}

/// Acknowledge packet for Publish message in QoS 1.
///
/// Basic packet structure:
/// ```txt
///  7                  0
/// +--------------------+
/// | Fixed header       |
/// |                    |
/// +--------------------+
/// | Packet id          |
/// |                    |
/// +--------------------+
/// | Reason Code        |
/// +--------------------+
/// | Properties ...     |
/// +--------------------+
/// ```
///
/// This type of packet does not contain payload.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PublishAckPacket {
    packet_id: PacketId,

    /// Byte 3 in the Variable Header is the PUBACK Reason Code. If the Remaining Length is 2,
    /// then there is no Reason Code and the value of 0x00 (Success) is used.
    reason: PublishAckReasonCode,

    /// The length of the Properties in the PUBACK packet Variable Header encoded
    /// as a Variable Byte Integer.  If the Remaining Length is less than 4
    /// there is no Property Length and the value of 0 is used.
    properties: Properties,
}

impl PublishAckPacket {
    pub fn new(packet_id: PacketId) -> Self {
        Self {
            packet_id,
            ..Self::default()
        }
    }

    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn set_reason(&mut self, reason: PublishAckReasonCode) -> &mut Self {
        self.reason = reason;
        self
    }

    pub fn reason(&self) -> PublishAckReasonCode {
        self.reason
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn mut_properties(&mut self) -> &mut Properties {
        &mut self.properties
    }
}

impl EncodePacket for PublishAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let mut packet_bytes = self.packet_id.bytes();
        if self.reason != PublishAckReasonCode::Success || !self.properties.is_empty() {
            packet_bytes += self.reason.bytes();
        }
        if !self.properties.is_empty() {
            packet_bytes += self.properties.bytes();
        }
        let fixed_header = FixedHeader::new(PacketType::PublishAck, packet_bytes)?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        if self.reason != PublishAckReasonCode::Success || !self.properties.is_empty() {
            buf.push(self.reason as u8);
        }
        if !self.properties.is_empty() {
            self.properties.encode(buf)?;
        }

        Ok(buf.len() - old_len)
    }
}

impl Packet for PublishAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PublishAck
    }
}

impl DecodePacket for PublishAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PublishAck {
            return Err(DecodeError::InvalidPacketType);
        }
        if fixed_header.remaining_length() < PacketId::const_bytes() {
            return Err(DecodeError::InvalidRemainingLength);
        }
        let packet_id = PacketId::decode(ba)?;
        let remaining_length = fixed_header.remaining_length() - packet_id.bytes();
        let reason = if remaining_length >= PublishAckReasonCode::const_bytes() {
            let reason_code_byte = ba.read_byte()?;
            let reason_code = PublishAckReasonCode::try_from(reason_code_byte)?;
            PublishAckReasonCode::default()
        } else {
            PublishAckReasonCode::default()
        };
        let properties = if remaining_length > PublishAckReasonCode::const_bytes() {
            Properties::decode(ba)?
        } else {
            Properties::new()
        };
        Ok(PublishAckPacket {
            packet_id,
            reason,
            properties,
        })
    }
}
