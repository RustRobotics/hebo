// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::{
    property::check_property_type_list, FixedHeader, Packet, PacketType, Properties, PropertyType,
};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// If the Server sends a ConnectAck packet with non-zero return code, it MUST
/// close the network connection.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectReasonCode {
    /// Connection accepted.
    Accepted = 0x00,

    /// The Server does not wish to reveal the reason for the failure, or
    /// none of the other Reason Codes apply.
    UnspecifiedError = 0x80,

    /// Data within the CONNECT packet could not be correctly parsed.
    MalformedPacket = 0x81,

    /// Data in the CONNECT packet does not conform to this specification.
    ProtocolError = 0x82,

    /// The CONNECT is valid but is not accepted by this Server.
    ImplementationSpecificError = 0x83,

    /// The Server does not support the version of the MQTT protocol requested by the Client.
    UnsupportedProtocolVersion = 0x84,

    /// The Client Identifier is a valid string but is not allowed by the Server.
    ClientIdentifierNotValid = 0x85,

    /// The Server does not accept the User Name or Password specified by the Client
    BadUserNameOrPassword = 0x86,

    /// The Client is not authorized to connect.
    NotAuthorized = 0x87,

    /// The MQTT Server is not available.
    ServerUnavailable = 0x88,

    /// The Server is busy. Try again later.
    ServerBusy = 0x89,

    /// This Client has been banned by administrative action. Contact the server administrator.
    Banned = 0x8a,

    /// The authentication method is not supported or does not match the authentication method
    /// currently in use.
    BadAuthenticationMethod = 0x8c,

    /// The Will Topic Name is not malformed, but is not accepted by this Server.
    TopicNameInvalid = 0x90,

    /// The CONNECT packet exceeded the maximum permissible size.
    PacketTooLarge = 0x95,

    /// An implementation or administrative imposed limit has been exceeded.
    QuotaExceeded = 0x97,

    /// The Will Payload does not match the specified Payload Format Indicator.
    PayloadFormatInvalid = 0x99,

    /// The Server does not support the QoS set in Will QoS.
    QoSNotSupported = 0x9b,

    /// The Client should temporarily use another server.
    UseAnotherServer = 0x9c,

    /// The Client should permanently use another server.
    ServerMoved = 0x9d,

    /// The connection rate limit has been exceeded.
    ConnectionRateExceeded = 0x9f,
}

impl Default for ConnectReasonCode {
    fn default() -> ConnectReasonCode {
        ConnectReasonCode::Accepted
    }
}

impl TryFrom<u8> for ConnectReasonCode {
    type Error = DecodeError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(ConnectReasonCode::Accepted),
            0x80 => Ok(ConnectReasonCode::UnspecifiedError),
            0x81 => Ok(ConnectReasonCode::MalformedPacket),
            0x82 => Ok(ConnectReasonCode::ProtocolError),
            0x83 => Ok(ConnectReasonCode::ImplementationSpecificError),
            0x84 => Ok(ConnectReasonCode::UnsupportedProtocolVersion),
            0x85 => Ok(ConnectReasonCode::ClientIdentifierNotValid),
            0x86 => Ok(ConnectReasonCode::BadUserNameOrPassword),
            0x87 => Ok(ConnectReasonCode::NotAuthorized),
            0x88 => Ok(ConnectReasonCode::ServerUnavailable),
            0x89 => Ok(ConnectReasonCode::ServerBusy),
            0x8a => Ok(ConnectReasonCode::Banned),
            0x8c => Ok(ConnectReasonCode::BadAuthenticationMethod),
            0x90 => Ok(ConnectReasonCode::TopicNameInvalid),
            0x95 => Ok(ConnectReasonCode::PacketTooLarge),
            0x97 => Ok(ConnectReasonCode::QuotaExceeded),
            0x99 => Ok(ConnectReasonCode::PayloadFormatInvalid),
            0x9b => Ok(ConnectReasonCode::QoSNotSupported),
            0x9c => Ok(ConnectReasonCode::UseAnotherServer),
            0x9d => Ok(ConnectReasonCode::ServerMoved),
            0x9f => Ok(ConnectReasonCode::ConnectionRateExceeded),
            _ => Err(DecodeError::OtherErrors),
        }
    }
}

/// The CONNACK packet is the packet sent by the Server in response to a CONNECT packet
/// received from a Client.
///
/// The Server MUST send a CONNACK with a 0x00 (Success) Reason Code before
/// sending any Packet other than AUTH [MQTT-3.2.0-1].
///
/// The Server MUST NOT send more than one CONNACK in a Network Connection [MQTT-3.2.0-2].
///
/// Basic packet structure:
/// ```txt
///  7                       0
/// +-------------------------+
/// | Fixed header            |
/// |                         |
/// +-------------------------+
/// | Ack flags               |
/// +-------------------------+
/// | Reason code             |
/// +-------------------------+
/// | Properties              |
/// |                         |
/// +-------------------------+
/// ```
/// he Variable Header of the CONNACK Packet contains the following fields in the order:
/// - Connect Acknowledge Flags
/// - Connect Reason Code
/// - Properties.
///
/// If the Client does not receive a CONNACK packet from the Server within a reasonable
/// amount of time, the Client SHOULD close the Network Connection. A "reasonable"
/// amount of time depends on the type of application and the communications infrastructure.
///
/// This packet does not contain payload.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConnectAckPacket {
    /// Acknowledge flags is the first byte in variable header.
    /// Session Present flag is set in bit 0 of Ack flags, bits 7-1 are reserved.
    ///
    /// If CleanSession flag in ConnectPacket is true, then this flag must be false
    /// and return code is set to zero.
    ///
    /// If CleanSession flag in ConnectPacket is false, and the Server have stored
    /// SessionState with the same ClientId, then this field is set to true, indicating
    /// that there is already a session state value present on the Server side.
    ///
    /// If return code is not zero, then this flag MUST be false.
    session_present: bool,

    /// Byte 2 in the connection return code.
    reason_code: ConnectReasonCode,

    properties: Properties,
}

pub const CONNECT_ACK_PROPERTIES: &[PropertyType] = &[
    PropertyType::SessionExpiryInterval,
    PropertyType::ReceiveMaximum,
    PropertyType::MaximumQoS,
    PropertyType::RetainAvailable,
    PropertyType::MaximumPacketSize,
    PropertyType::AssignedClientIdentifier,
    PropertyType::TopicAliasMaximum,
    PropertyType::ReasonString,
    PropertyType::UserProperty,
    PropertyType::WildcardSubscriptionAvailable,
];

impl ConnectAckPacket {
    pub fn new(mut session_present: bool, reason_code: ConnectReasonCode) -> ConnectAckPacket {
        // If a Server sends a CONNACK packet containing a non-zero Reason Code
        // it MUST set Session Present to 0 [MQTT-3.2.2-6].
        if reason_code != ConnectReasonCode::Accepted {
            session_present = false;
        }
        ConnectAckPacket {
            session_present,
            reason_code,
            properties: Vec::new(),
        }
    }

    pub fn set_reason_code(&mut self, code: ConnectReasonCode) -> &mut Self {
        if code != ConnectReasonCode::Accepted {
            self.session_present = false;
        }
        self.reason_code = code;
        self
    }

    pub fn reason_code(&self) -> ConnectReasonCode {
        self.reason_code
    }

    pub fn set_session_present(&mut self, present: bool) -> &mut Self {
        self.session_present = present;
        self
    }

    pub fn session_present(&self) -> bool {
        self.session_present
    }
}

impl DecodePacket for ConnectAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        assert_eq!(fixed_header.packet_type(), PacketType::ConnectAck);

        let ack_flags = ba.read_byte()?;
        let session_present = ack_flags & 0b0000_0001 == 0b0000_0001;
        let reason_code_byte = ba.read_byte()?;
        let reason_code = ConnectReasonCode::try_from(reason_code_byte)?;
        let properties = Properties::decode(ba)?;

        if let Err(property_type) = check_property_type_list(&properties, CONNECT_ACK_PROPERTIES) {
            log::error!(
                "v5/ConnectAckPacket: property type {:?} cannot be used in properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        Ok(ConnectAckPacket {
            session_present,
            reason_code,
            properties,
        })
    }
}

impl EncodePacket for ConnectAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();
        let fixed_header = FixedHeader::new(PacketType::ConnectAck, 2)?;
        fixed_header.encode(buf)?;

        let ack_flags = if self.session_present { 0b0000_0001 } else { 0 };
        buf.push(ack_flags);
        buf.push(self.reason_code as u8);

        Ok(buf.len() - old_len)
    }
}

impl Packet for ConnectAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::ConnectAck
    }
}

#[cfg(test)]
mod tests {
    use super::{ByteArray, ConnectAckPacket, DecodePacket};

    #[test]
    fn test_decode() {
        let buf: Vec<u8> = vec![0x20, 0x02, 0x00, 0x00];
        let mut ba = ByteArray::new(&buf);
        let packet = ConnectAckPacket::decode(&mut ba);
        assert!(packet.is_ok());
        let packet = packet.unwrap();
        assert_eq!(packet.session_present, false);
    }
}
