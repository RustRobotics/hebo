// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::{FixedHeader, Packet, PacketType};
use crate::base::PROTOCOL_NAME;
use crate::connect_flags::ConnectFlags;
use crate::utils::{validate_client_id, validate_keep_alive};
use crate::{
    BinaryData, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, ProtocolLevel,
    PubTopic, QoS, StringData, U16Data, VarIntError,
};

/// `ConnectPacket` consists of three parts:
/// * `FixedHeader`
/// * `VariableHeader`
/// * `Payload`
/// Note that fixed header part is same in all packets so that we just ignore it.
///
/// Basic struct of `ConnectPacket` is as below:
/// ```txt
///  7                          0
/// +----------------------------+
/// | Fixed header               |
/// |                            |
/// +----------------------------+
/// | Protocol level             |
/// +----------------------------+
/// | Connect flags              |
/// +----------------------------+
/// | Keep alive                 |
/// |                            |
/// +----------------------------+
/// | Client id length           |
/// |                            |
/// +----------------------------+
/// | Client id string ...       |
/// +----------------------------+
/// | Will topic length          |
/// |                            |
/// +----------------------------+
/// | Will topic string ...      |
/// +----------------------------+
/// | Will message length        |
/// |                            |
/// +----------------------------+
/// | Will message bytes ...     |
/// +----------------------------+
/// | Username length            |
/// |                            |
/// +----------------------------+
/// | Username string ...        |
/// +----------------------------+
/// | Password length            |
/// |                            |
/// +----------------------------+
/// | Password bytes ...         |
/// +----------------------------+
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConnectPacket {
    /// Protocol name can only be `MQTT` in specification.
    protocol_name: StringData,

    protocol_level: ProtocolLevel,

    connect_flags: ConnectFlags,

    /// Time interval between two packets in seconds.
    /// Client must send PingRequest Packet before exceeding this interval.
    /// If this value is not zero and time exceeds after last packet, the Server
    /// will disconnect the network.
    ///
    /// If this value is zero, the Server is not required to disconnect the network.
    keep_alive: U16Data,

    /// Payload is `client_id`.
    /// `client_id` is generated in client side. Normally it can be `device_id` or just
    /// randomly generated string.
    /// `client_id` is used to identify client connections in server. Session is based on this field.
    /// It must be valid UTF-8 string, length shall be between 1 and 23 bytes.
    /// It can only contain the characters: "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    /// If `client_id` is invalid, the Server will reply ConnectAck Packet with return code
    /// 0x02(Identifier rejected).
    client_id: StringData,

    /// If the `will` flag is true in `connect_flags`, then `will_topic` field must be set.
    /// It will be used as the topic of Will Message.
    will_topic: Option<PubTopic>,

    /// If the `will` flag is true in `connect_flags`, then `will_message` field must be set.
    /// It will be used as the payload of Will Message.
    /// It consists of 0 to 64k bytes of binary data.
    will_message: BinaryData,

    /// If the `username` flag is true in `connect_flags`, then `username` field must be set.
    /// It is a valid UTF-8 string.
    username: StringData,

    /// If the `password` flag is true in `connect_flags`, then `password` field must be set.
    /// It consists of 0 to 64k bytes of binary data.
    password: BinaryData,
}

impl ConnectPacket {
    /// Create a new connect packet with `client_id`.
    ///
    /// # Errors
    ///
    /// Returns error if `client_id` is invalid.
    pub fn new(client_id: &str) -> Result<Self, EncodeError> {
        let protocol_name = StringData::from(PROTOCOL_NAME)?;
        validate_client_id(client_id).map_err(|_err| EncodeError::InvalidClientId)?;
        let client_id = StringData::from(client_id)?;
        Ok(Self {
            protocol_name,
            keep_alive: U16Data::new(60),
            client_id,
            ..Self::default()
        })
    }

    /// Update protocol level.
    pub fn set_protcol_level(&mut self, level: ProtocolLevel) -> &Self {
        self.protocol_level = level;
        self
    }

    /// Get current protocol level.
    #[must_use]
    #[inline]
    pub const fn protocol_level(&self) -> ProtocolLevel {
        self.protocol_level
    }

    /// Update connect flags
    pub fn set_connect_flags(&mut self, flags: ConnectFlags) -> &Self {
        self.connect_flags = flags;
        self
    }

    /// Get current connect flags.
    #[must_use]
    #[inline]
    pub const fn connect_flags(&self) -> &ConnectFlags {
        &self.connect_flags
    }

    /// Update keep alive value in milliseconds.
    pub fn set_keep_alive(&mut self, keep_alive: u16) -> &mut Self {
        self.keep_alive = U16Data::new(keep_alive);
        self
    }

    /// Get current keep alive value.
    #[must_use]
    #[inline]
    pub const fn keep_alive(&self) -> u16 {
        // TODO(Shaohua): Returns a duration
        self.keep_alive.value()
    }

    /// Update client id.
    ///
    /// # Errors
    ///
    /// Returns error if `client_id` is invalid.
    pub fn set_client_id(&mut self, client_id: &str) -> Result<&mut Self, EncodeError> {
        validate_client_id(client_id).map_err(|_err| EncodeError::InvalidClientId)?;
        self.client_id = StringData::from(client_id)?;
        Ok(self)
    }

    /// Get current client id.
    #[must_use]
    pub fn client_id(&self) -> &str {
        self.client_id.as_ref()
    }

    /// Update username value.
    ///
    /// # Errors
    ///
    /// Returns error if `username` contains invalid chars or too long.
    pub fn set_username(&mut self, username: &str) -> Result<&mut Self, EncodeError> {
        self.username = StringData::from(username)?;
        Ok(self)
    }

    /// Get current username value.
    #[must_use]
    pub fn username(&self) -> &str {
        self.username.as_ref()
    }

    /// Update password value.
    ///
    /// # Errors
    ///
    /// Returns error if `password` is too long.
    pub fn set_password(&mut self, password: &[u8]) -> Result<&mut Self, EncodeError> {
        self.password = BinaryData::from_slice(password)?;
        Ok(self)
    }

    /// Get current password value.
    #[must_use]
    pub fn password(&self) -> &[u8] {
        self.password.as_ref()
    }

    /// Update will-topic.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn set_will_topic(&mut self, topic: &str) -> Result<&mut Self, EncodeError> {
        if topic.is_empty() {
            self.will_topic = None;
        } else {
            self.will_topic = Some(PubTopic::new(topic)?);
        }
        Ok(self)
    }

    /// Get current will-topic value.
    #[must_use]
    pub fn will_topic(&self) -> Option<&str> {
        self.will_topic.as_ref().map(AsRef::as_ref)
    }

    /// Update will-message.
    ///
    /// # Errors
    ///
    /// Returns error if `message` is too long.
    pub fn set_will_message(&mut self, message: &[u8]) -> Result<&mut Self, EncodeError> {
        self.will_message = BinaryData::from_slice(message)?;
        Ok(self)
    }

    /// Get current will-message value.
    #[must_use]
    pub fn will_message(&self) -> &[u8] {
        self.will_message.as_ref()
    }

    // TODO(Shaohua): Add more getters/setters.

    fn get_fixed_header(&self) -> Result<FixedHeader, VarIntError> {
        let mut remaining_length = self.protocol_name.bytes()
            + ProtocolLevel::bytes()
            + ConnectFlags::bytes()
            + U16Data::bytes()  // keep_alive
            + self.client_id.bytes();

        // Check username/password/topic/message.
        if self.connect_flags.will() {
            assert!(self.will_topic.is_some());
            if let Some(will_topic) = &self.will_topic {
                remaining_length += will_topic.bytes();
            }
            remaining_length += self.will_message.bytes();
        }
        if self.connect_flags.has_username() {
            remaining_length += self.username.bytes();
        }
        if self.connect_flags.has_password() {
            remaining_length += self.password.bytes();
        }
        FixedHeader::new(PacketType::Connect, remaining_length)
    }
}

impl EncodePacket for ConnectPacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = v.len();

        // Write fixed header
        let fixed_header = self.get_fixed_header()?;
        fixed_header.encode(v)?;

        // Write variable header
        self.protocol_name.encode(v)?;
        self.protocol_level.encode(v)?;
        self.connect_flags.encode(v)?;
        self.keep_alive.encode(v)?;

        // Write payload
        self.client_id.encode(v)?;
        if self.connect_flags.will() {
            assert!(self.will_topic.is_some());
            if let Some(will_topic) = &self.will_topic {
                will_topic.encode(v)?;
            }

            self.will_message.encode(v)?;
        }
        if self.connect_flags.has_username() {
            self.username.encode(v)?;
        }
        if self.connect_flags.has_password() {
            self.password.encode(v)?;
        }

        Ok(v.len() - old_len)
    }
}

impl DecodePacket for ConnectPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Connect {
            return Err(DecodeError::InvalidPacketType);
        }

        let protocol_name = StringData::decode(ba)?;
        if protocol_name.as_ref() != PROTOCOL_NAME {
            return Err(DecodeError::InvalidProtocolName);
        }

        let protocol_level = ProtocolLevel::try_from(ba.read_byte()?)?;

        let connect_flags = ConnectFlags::decode(ba)?;
        // If the Will Flag is set to 0 the Will QoS and Will Retain fields in the
        // Connect Flags MUST be set to zero and the Will Topic and Will Message fields
        // MUST NOT be present in the payload [MQTT-3.1.2-11].
        //
        // If the Will Flag is set to 0, then the Will QoS MUST be set to 0 (0x00) [MQTT-3.1.2-13].
        //
        // If the Will Flag is set to 1, the value of Will QoS can be 0 (0x00), 1 (0x01), or 2 (0x02).
        // It MUST NOT be 3 (0x03) [MQTT-3.1.2-14].
        if !connect_flags.will()
            && (connect_flags.will_qos() != QoS::AtMostOnce || connect_flags.will_retain())
        {
            return Err(DecodeError::InvalidConnectFlags);
        }

        // If the User Name Flag is set to 0, the Password Flag MUST be set to 0 [MQTT-3.1.2-22].
        if !connect_flags.has_username() && connect_flags.has_password() {
            return Err(DecodeError::InvalidConnectFlags);
        }

        let keep_alive = U16Data::decode(ba)?;
        validate_keep_alive(keep_alive.value())?;

        // A Server MAY allow a Client to supply a ClientId that has a length of zero bytes,
        // however if it does so the Server MUST treat this as a special case and assign
        // a unique ClientId to that Client. It MUST then process the CONNECT packet
        // as if the Client had provided that unique ClientId [MQTT-3.1.3-6].
        let client_id = StringData::decode(ba).map_err(|_err| DecodeError::InvalidClientId)?;

        // If the Client supplies a zero-byte ClientId, the Client MUST also set CleanSession
        // to 1 [MQTT-3.1.3-7].
        //
        // If the Client supplies a zero-byte ClientId with CleanSession set to 0, the Server
        // MUST respond to the CONNECT Packet with a CONNACK return code 0x02 (Identifier rejected)
        // and then close the Network Connection [MQTT-3.1.3-8].
        if client_id.is_empty() && !connect_flags.clean_session() {
            return Err(DecodeError::InvalidClientId);
        }

        let will_topic = if connect_flags.will() {
            Some(PubTopic::decode(ba)?)
        } else {
            None
        };
        let will_message = if connect_flags.will() {
            BinaryData::decode(ba)?
        } else {
            BinaryData::new()
        };

        let username = if connect_flags.has_username() {
            StringData::decode(ba)?
        } else {
            StringData::new()
        };

        let password = if connect_flags.has_password() {
            BinaryData::decode(ba)?
        } else {
            BinaryData::new()
        };

        Ok(Self {
            protocol_name,
            protocol_level,
            connect_flags,
            keep_alive,
            client_id,
            will_topic,
            will_message,
            username,
            password,
        })
    }
}

impl Packet for ConnectPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Connect
    }

    fn bytes(&self) -> Result<usize, VarIntError> {
        let fixed_header = self.get_fixed_header()?;
        Ok(fixed_header.bytes() + fixed_header.remaining_length())
    }
}

#[cfg(test)]
mod tests {
    use super::{ByteArray, ConnectPacket, DecodePacket};

    #[test]
    fn test_decode() {
        let buf: Vec<u8> = vec![
            16, 20, 0, 4, 77, 81, 84, 84, 4, 2, 0, 60, 0, 8, 119, 118, 80, 84, 88, 99, 67, 119,
        ];
        let mut ba = ByteArray::new(&buf);
        let packet = ConnectPacket::decode(&mut ba);
        assert!(packet.is_ok());
        let packet = packet.unwrap();
        assert_eq!(packet.client_id(), "wvPTXcCw");
    }
}
