// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// Byte 1 in the Variable Header is the Disconnect Reason Code.
///
/// If the Remaining Length is less than 1 the value of 0x00 (Normal disconnection) is used.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisconnectReasonCode {
    /// Close the connection normally.
    ///
    /// Do not send the Will Message.
    ///
    /// Sent by client or server.
    NormalDisconnection = 0x00,

    /// The Client wishes to disconnect but requires that the Server also publishes its Will Message.
    ///
    /// Sent by client.
    DisconnectWithWillMessage = 0x04,

    /// The Connection is closed but the sender either does not wish to reveal the reason,
    /// or none of the other Reason Codes apply.
    ///
    /// Sent by client or server.
    UnspecifiedError = 0x80,

    /// The received packet does not conform to this specification.
    ///
    /// Sent by client or server.
    MalformedPacket = 0x81,

    /// An unexpected or out of order packet was received.
    ///
    /// Sent by client or server.
    ProtocolError = 0x82,

    /// The packet received is valid but cannot be processed by this implementation.
    ///
    /// Sent by client or server.
    ImplementationSpecificError = 0x83,

    /// The request is not authorized.
    ///
    /// Sent by server.
    NotAuthorized = 0x87,

    /// The Server is busy and cannot continue processing requests from this Client.
    ///
    /// Sent by server.
    ServerBusy = 0x89,

    /// The Server is shutting down.
    ///
    /// Sent by server.
    ServerShuttingDown = 0x8b,

    /// The Connection is closed because no packet has been received for 1.5 times the Keepalive time.
    ///
    /// Sent by server.
    KeepAliveTimeout = 0x8d,

    /// Another Connection using the same ClientID has connected causing this Connection to be closed.
    ///
    /// Sent by server.
    SessionTakenOver = 0x8e,

    /// The Topic Filter is correctly formed, but is not accepted by this Sever.
    ///
    /// Sent by server.
    TopicFilterInvalid = 0x8f,

    /// The Topic Name is correctly formed, but is not accepted by this Client or Server.
    ///
    /// Sent by client or server.
    TopicNameInvalid = 0x90,

    /// The Client or Server has received more than Receive Maximum publication
    /// for which it has not sent PUBACK or PUBCOMP.
    ///
    /// Sent by client or server.
    ReceiveMaximumExceeded = 0x93,

    /// The Client or Server has received a PUBLISH packet containing a Topic Alias
    /// which is greater than the Maximum Topic Alias it sent in the CONNECT or CONNACK packet.
    ///
    /// Sent by client or server.
    TopicAliasInvalid = 0x94,
}

impl DisconnectReasonCode {
    pub fn bytes(&self) -> usize {
        1
    }
    pub fn const_bytes() -> usize {
        1
    }
}

impl Default for DisconnectReasonCode {
    fn default() -> Self {
        Self::NormalDisconnection
    }
}

impl TryFrom<u8> for DisconnectReasonCode {
    type Error = DecodeError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(Self::NormalDisconnection),
            0x04 => Ok(Self::DisconnectWithWillMessage),
            0x80 => Ok(Self::UnspecifiedError),
            0x81 => Ok(Self::MalformedPacket),
            0x82 => Ok(Self::ProtocolError),
            0x83 => Ok(Self::ImplementationSpecificError),
            0x87 => Ok(Self::NotAuthorized),
            0x89 => Ok(Self::ServerBusy),
            0x8b => Ok(Self::ServerShuttingDown),
            0x8d => Ok(Self::KeepAliveTimeout),
            0x8e => Ok(Self::SessionTakenOver),
            0x8f => Ok(Self::TopicFilterInvalid),
            0x90 => Ok(Self::TopicNameInvalid),
            0x93 => Ok(Self::ReceiveMaximumExceeded),
            0x94 => Ok(Self::TopicAliasInvalid),
            _ => Err(DecodeError::OtherErrors),
        }
    }
}

/// The Disconnect packet is the final packet sent to the Server from a Client.
///
/// When the Server receives this packet, it will close the network connection
/// and will not send any more packets. And the Server will discard any Will message
/// associated with current connection.
///
/// This packet does not contain variable header or payload.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DisconnectPacket {
    reason_code: DisconnectReasonCode,
    properties: Properties,
}

impl DisconnectPacket {
    pub fn new() -> DisconnectPacket {
        Self::default()
    }

    pub fn set_reason_code(&mut self, code: DisconnectReasonCode) -> &mut Self {
        self.reason_code = code;
        self
    }

    pub fn reason_code(&self) -> DisconnectReasonCode {
        self.reason_code
    }
}

pub const DISCONNECT_PROPERTIES: &[PropertyType] = &[];

impl EncodePacket for DisconnectPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let remaining_length = self.reason_code.bytes() + self.properties.bytes();
        let fixed_header = FixedHeader::new(PacketType::Disconnect, remaining_length)?;
        fixed_header.encode(buf)?;
        buf.push(self.reason_code as u8);
        self.properties.encode(buf)?;

        Ok(buf.len() - old_len)
    }
}

impl Packet for DisconnectPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Disconnect
    }
}

impl DecodePacket for DisconnectPacket {
    fn decode(ba: &mut ByteArray) -> Result<DisconnectPacket, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Disconnect {
            return Err(DecodeError::InvalidPacketType);
        }
        if fixed_header.remaining_length() == 0 {
            return Ok(Self::default());
        }

        let reason_code_byte = ba.read_byte()?;
        let reason_code = DisconnectReasonCode::try_from(reason_code_byte)?;

        let properties = Properties::decode(ba)?;
        if let Err(property_type) =
            check_property_type_list(properties.props(), DISCONNECT_PROPERTIES)
        {
            log::error!(
                "v5/DisconnectPacket: property type {:?} cannot be used in properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        Ok(Self {
            reason_code,
            properties,
        })
    }
}
