// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::property::check_property_type_list;
use super::{Properties, PropertyType, ReasonCode};
use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, Packet,
    PacketType, VarIntError,
};

/// The Disconnect packet is the final packet sent to the Server from a Client.
///
/// When the Server receives this packet, it will close the network connection
/// and will not send any more packets. And the Server will discard any Will message
/// associated with current connection.
///
/// ```txt
///  7                       0
/// +-------------------------+
/// | Fixed header            |
/// |                         |
/// +-------------------------+
/// | Reason Code             |
/// +-------------------------+
/// | Properties ...          |
/// +-------------------------+
/// ```
///
/// This packet does not contain variable header or payload.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DisconnectPacket {
    reason_code: ReasonCode,
    properties: Properties,
}

impl DisconnectPacket {
    /// Create a disconnect packet with default value.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update reason code.
    pub const fn set_reason_code(&mut self, reason_code: ReasonCode) -> &mut Self {
        self.reason_code = reason_code;
        self
    }

    /// Get current reason code.
    #[must_use]
    pub const fn reason_code(&self) -> ReasonCode {
        self.reason_code
    }

    /// Get a mutable reference to property list.
    pub const fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Get a reference to property list.
    #[must_use]
    pub const fn properties(&self) -> &Properties {
        &self.properties
    }

    fn get_fixed_header(&self) -> Result<FixedHeader, VarIntError> {
        let remaining_length = ReasonCode::bytes() + self.properties.bytes();
        FixedHeader::new(PacketType::Disconnect, remaining_length)
    }
}

/// Byte 1 in the Variable Header is the Disconnect Reason Code.
///
/// If the Remaining Length is less than 1 the value of 0x00 (Normal disconnection) is used.
pub const DISCONNECT_REASONS: &[ReasonCode] = &[
    ReasonCode::Success,
    ReasonCode::DisconnectWithWillMessage,
    ReasonCode::UnspecifiedError,
    ReasonCode::MalformedPacket,
    ReasonCode::ProtocolError,
    ReasonCode::ImplementationSpecificError,
    ReasonCode::NotAuthorized,
    ReasonCode::ServerBusy,
    ReasonCode::ServerShuttingDown,
    ReasonCode::KeepAliveTimeout,
    ReasonCode::SessionTakenOver,
    ReasonCode::TopicFilterInvalid,
    ReasonCode::TopicNameInvalid,
    ReasonCode::ReceiveMaximumExceeded,
    ReasonCode::TopicAliasInvalid,
];

/// Properties available in disconnect packet.
pub const DISCONNECT_PROPERTIES: &[PropertyType] = &[
    // The Session Expiry Interval MUST NOT be sent on a DISCONNECT by the Server [MQTT-3.14.2-2].
    PropertyType::SessionExpiryInterval,
    // The sender MUST NOT send this Property if it would increase the size of the DISCONNECT packet
    // beyond the Maximum Packet Size specified by the receiver [MQTT-3.14.2-3].
    PropertyType::ReasonString,
    // The sender MUST NOT send this property if it would increase the size of the DISCONNECT
    // packet beyond the Maximum Packet Size specified by the receiver [MQTT-3.14.2-4].
    PropertyType::UserProperty,
    PropertyType::ServerReference,
];

impl EncodePacket for DisconnectPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = self.get_fixed_header()?;
        fixed_header.encode(buf)?;
        self.reason_code.encode(buf)?;
        self.properties.encode(buf)?;

        Ok(buf.len() - old_len)
    }
}

impl DecodePacket for DisconnectPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Disconnect {
            return Err(DecodeError::InvalidPacketType);
        }
        if fixed_header.remaining_length() == 0 {
            return Ok(Self::default());
        }

        let reason_code = ReasonCode::decode(ba)?;
        if !DISCONNECT_REASONS.contains(&reason_code) {
            log::error!("Invalid reason code {reason_code:?}");
            return Err(DecodeError::InvalidReasonCode);
        }

        let properties = Properties::decode(ba)?;
        if let Err(property_type) =
            check_property_type_list(properties.props(), DISCONNECT_PROPERTIES)
        {
            log::error!(
                "v5/DisconnectPacket: property type {property_type:?} cannot be used in properties!"
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        Ok(Self {
            reason_code,
            properties,
        })
    }
}

impl Packet for DisconnectPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Disconnect
    }

    fn bytes(&self) -> Result<usize, VarIntError> {
        let fixed_header = self.get_fixed_header()?;
        Ok(fixed_header.bytes() + fixed_header.remaining_length())
    }
}
