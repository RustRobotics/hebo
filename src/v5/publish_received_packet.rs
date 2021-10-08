// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

/// Byte 3 in the Variable Header is the PUBREC Reason Code. If the Remaining Length is 2,
/// then the Publish Reason Code has the value 0x00 (Success).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishReceivedReasonCode {
    /// The message is accepted. Publication of the QoS 2 message proceeds.
    Success = 0x00,

    /// The message is accepted but there are no subscribers.
    ///
    /// This is sent only by the Server. If the Server knows that there are
    /// no matching subscribers, it MAY use this Reason Code instead of 0x00 (Success).
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

    /// The payload format does not match the one specified in the Payload Format Indicator.
    PayloadFormatInvalid = 0x99,
}

impl Default for PublishReceivedReasonCode {
    fn default() -> Self {
        Self::Success
    }
}

impl TryFrom<u8> for PublishReceivedReasonCode {
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

impl PublishReceivedReasonCode {
    pub fn bytes(&self) -> usize {
        1
    }

    pub fn const_bytes() -> usize {
        1
    }
}

/// Response to a Publish packet with QoS 2. It is the second packet of the QoS 2 protocol
/// exchange.
///
/// Packet structre is:
/// ```txt
///  7                     0
/// +-----------------------+
/// | Fixed header          |
/// |                       |
/// +-----------------------+
/// | Packet id             |
/// |                       |
/// +-----------------------+
/// | Reason Code           |
/// +-----------------------+
/// | Properties ...        |
/// +-----------------------+
/// ```
///
/// This packet does not contain payload part.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PublishReceivedPacket {
    packet_id: PacketId,
    reason_code: PublishReceivedReasonCode,
    properties: Properties,
}

pub const PUBLISH_RECEIVED_PROPERTIES: &[PropertyType] = &[
    // The sender MUST NOT send this property if it would increase the size of the PUBREC packet
    // beyond the Maximum Packet Size specified by the receiver [MQTT-3.5.2-2].
    PropertyType::ReasonString,
    // The sender MUST NOT send this property if it would increase the size of the PUBREC packet
    // beyond the Maximum Packet Size specified by the receiver [MQTT-3.5.2-3].
    PropertyType::UserProperty,
];

impl PublishReceivedPacket {
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

    pub fn set_reason_code(&mut self, reason_code: PublishReceivedReasonCode) -> &mut Self {
        self.reason_code = reason_code;
        self
    }

    pub fn reason_code(&self) -> PublishReceivedReasonCode {
        self.reason_code
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn mut_properties(&mut self) -> &mut Properties {
        &mut self.properties
    }
}

impl EncodePacket for PublishReceivedPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let mut packet_bytes = self.packet_id.bytes();
        if self.reason_code != PublishReceivedReasonCode::Success || !self.properties.is_empty() {
            packet_bytes += self.reason_code.bytes();
        }
        if !self.properties.is_empty() {
            packet_bytes += self.properties.bytes();
        }
        let fixed_header = FixedHeader::new(PacketType::PublishReceived, packet_bytes)?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        if self.reason_code != PublishReceivedReasonCode::Success || !self.properties.is_empty() {
            buf.push(self.reason_code as u8);
        }
        if !self.properties.is_empty() {
            self.properties.encode(buf)?;
        }
        Ok(buf.len() - old_len)
    }
}

impl Packet for PublishReceivedPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PublishReceived
    }
}

impl DecodePacket for PublishReceivedPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PublishReceived {
            return Err(DecodeError::InvalidPacketType);
        }
        if fixed_header.remaining_length() < PacketId::const_bytes() {
            return Err(DecodeError::InvalidRemainingLength);
        }
        let packet_id = PacketId::decode(ba)?;
        let remaining_length = fixed_header.remaining_length() - packet_id.bytes();
        let reason_code = if remaining_length >= PublishReceivedReasonCode::const_bytes() {
            let reason_code_byte = ba.read_byte()?;
            PublishReceivedReasonCode::try_from(reason_code_byte)?
        } else {
            PublishReceivedReasonCode::default()
        };
        let properties = if remaining_length > PublishReceivedReasonCode::const_bytes() {
            let properties = Properties::decode(ba)?;
            if let Err(property_type) =
                check_property_type_list(&properties, PUBLISH_RECEIVED_PROPERTIES)
            {
                log::error!(
                    "v5/PublishReceivedPacket: property type {:?} cannot be used in properties!",
                    property_type
                );
                return Err(DecodeError::InvalidPropertyType);
            }
            properties
        } else {
            Properties::new()
        };
        Ok(PublishReceivedPacket {
            packet_id,
            reason_code,
            properties,
        })
    }
}
