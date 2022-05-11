// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType, ReasonCode};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

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
    reason_code: ReasonCode,

    properties: Properties,
}

/// If the Server sends a ConnectAck packet with non-zero return code, it MUST
/// close the network connection.
pub const CONNECT_REASONS: &[ReasonCode] = &[
    ReasonCode::Success,
    ReasonCode::UnspecifiedError,
    ReasonCode::MalformedPacket,
    ReasonCode::ProtocolError,
    ReasonCode::ImplementationSpecificError,
    ReasonCode::UnsupportedProtocolVersion,
    ReasonCode::ClientIdentifierNotValid,
    ReasonCode::BadUserNameOrPassword,
    ReasonCode::NotAuthorized,
    ReasonCode::ServerUnavailable,
    ReasonCode::ServerBusy,
    ReasonCode::Banned,
    ReasonCode::BadAuthenticationMethod,
    ReasonCode::TopicNameInvalid,
    ReasonCode::PacketTooLarge,
    ReasonCode::QuotaExceeded,
    ReasonCode::PayloadFormatInvalid,
    ReasonCode::QoSNotSupported,
    ReasonCode::UseAnotherServer,
    ReasonCode::ServerMoved,
    ReasonCode::ConnectionRateExceeded,
];

/// Available properties for ConnectAck packets.
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
    PropertyType::SubscriptionIdentifierAvailable,
    PropertyType::SharedSubscriptionAvailable,
    PropertyType::ServerKeepAlive,
    PropertyType::ResponseInformation,
    PropertyType::ServerReference,
    PropertyType::AuthenticationMethod,
    PropertyType::AuthenticationData,
];

impl ConnectAckPacket {
    /// Create a new ConnectAck packet.
    pub fn new(mut session_present: bool, reason_code: ReasonCode) -> ConnectAckPacket {
        // If a Server sends a CONNACK packet containing a non-zero Reason Code
        // it MUST set Session Present to 0 [MQTT-3.2.2-6].
        if reason_code != ReasonCode::Success {
            session_present = false;
        }
        ConnectAckPacket {
            session_present,
            reason_code,
            properties: Properties::new(),
        }
    }

    /// Update reason_code.
    ///
    /// Returns Error if `reason_code` is not in `CONNECT_REASONS` list.
    pub fn set_reason_code(&mut self, reason_code: ReasonCode) -> Result<&mut Self, EncodeError> {
        if !CONNECT_REASONS.contains(&reason_code) {
            return Err(EncodeError::InvalidReasonCode);
        }
        if reason_code != ReasonCode::Success {
            self.session_present = false;
        }
        self.reason_code = reason_code;
        Ok(self)
    }

    /// Get current reason code.
    pub fn reason_code(&self) -> ReasonCode {
        self.reason_code
    }

    /// Update session present flag.
    pub fn set_session_present(&mut self, present: bool) -> &mut Self {
        self.session_present = present;
        self
    }

    /// Get current session present value.
    pub fn session_present(&self) -> bool {
        self.session_present
    }

    /// Get a mutable reference to property list.
    pub fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Get a reference to property list.
    pub fn properties(&self) -> &Properties {
        &self.properties
    }
}

impl DecodePacket for ConnectAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        assert_eq!(fixed_header.packet_type(), PacketType::ConnectAck);

        let ack_flags = ba.read_byte()?;
        let session_present = ack_flags & 0b0000_0001 == 0b0000_0001;
        let reason_code = ReasonCode::decode(ba)?;
        if !CONNECT_REASONS.contains(&reason_code) {
            log::error!("Invalid reason code {:?}", reason_code);
            return Err(DecodeError::InvalidReasonCode);
        }
        let properties = Properties::decode(ba)?;

        if let Err(property_type) =
            check_property_type_list(properties.props(), CONNECT_ACK_PROPERTIES)
        {
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

        let remaining_length = 1 + ReasonCode::bytes() + self.properties.bytes();
        let fixed_header = FixedHeader::new(PacketType::ConnectAck, remaining_length)?;
        fixed_header.encode(buf)?;

        let ack_flags = if self.session_present { 0b0000_0001 } else { 0 };
        buf.push(ack_flags);
        self.reason_code.encode(buf)?;
        self.properties.encode(buf)?;

        Ok(buf.len() - old_len)
    }
}

impl Packet for ConnectAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::ConnectAck
    }
}
