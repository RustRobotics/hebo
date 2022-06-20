// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::property::check_property_type_list;
use super::{Properties, PropertyType};
use crate::base::PROTOCOL_NAME;
use crate::connect_flags::ConnectFlags;
use crate::utils::validate_client_id;
use crate::{
    validate_keep_alive, BinaryData, ByteArray, DecodeError, DecodePacket, EncodeError,
    EncodePacket, FixedHeader, KeepAlive, Packet, PacketType, ProtocolLevel, PubTopic, QoS,
    StringData, VarIntError,
};

/// `ConnectPacket` consists of three parts:
/// * `FixedHeader`
/// * `VariableHeader`
/// * `Payload`
///
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
/// | Properties Length          |
/// +----------------------------+
/// | Properties                 |
/// |                            |
/// +----------------------------+
/// | Client id length           |
/// |                            |
/// +----------------------------+
/// | Client id string ...       |
/// +----------------------------+
/// | Will Properties Length     |
/// +----------------------------+
/// | Will Properties            |
/// |                            |
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
/// After a Network Connection is established by a Client to a Server, the first packet
/// sent from the Client to the Server MUST be a CONNECT packet [MQTT-3.1.0-1].
///
/// A Client can only send the CONNECT packet once over a Network Connection. The Server MUST
/// process a second CONNECT packet sent from a Client as a Protocol Error and close the Network
/// Connection [MQTT-3.1.0-2].
///
/// The Payload of the CONNECT packet contains one or more length-prefixed fields,
/// whose presence is determined by the flags in the Variable Header. These fields,
/// if present, MUST appear in the order Client Identifier, Will Properties, Will Topic,
/// Will Payload, User Name, Password [MQTT-3.1.3-1].
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConnectPacket {
    /// Protocol name can only be `MQTT` in specification.
    protocol_name: StringData,

    protocol_level: ProtocolLevel,

    connect_flags: ConnectFlags,

    /// The Keep Alive is a Two Byte Integer which is a time interval measured in seconds.
    ///
    /// It is the maximum time interval that is permitted to elapse between the point
    /// at which the Client finishes transmitting one MQTT Control Packet and the point
    /// it starts sending the next. It is the responsibility of the Client to ensure
    /// that the interval between MQTT Control Packets being sent does not exceed the Keep Alive value.
    /// If Keep Alive is non-zero and in the absence of sending any other MQTT Control Packets,
    /// the Client MUST send a PINGREQ packet [MQTT-3.1.2-20].
    ///
    /// If the Server returns a Server Keep Alive on the CONNACK packet, the Client MUST
    /// use that value instead of the value it sent as the Keep Alive [MQTT-3.1.2-21].
    ///
    /// The Client can send PINGREQ at any time, irrespective of the Keep Alive value,
    /// and check for a corresponding PINGRESP to determine that the network and
    /// the Server are available.
    ///
    /// If the Keep Alive value is non-zero and the Server does not receive an MQTT Control Packet
    /// from the Client within one and a half times the Keep Alive time period,
    /// it MUST close the Network Connection to the Client as if the network had failed [MQTT-3.1.2-22].
    ///
    /// If a Client does not receive a PINGRESP packet within a reasonable amount of time
    /// after it has sent a PINGREQ, it SHOULD close the Network Connection to the Server.
    ///
    /// A Keep Alive value of 0 has the effect of turning off the Keep Alive mechanism.
    /// If Keep Alive is 0 the Client is not obliged to send MQTT Control Packets
    /// on any particular schedule.
    keep_alive: KeepAlive,

    properties: Properties,

    // <-- variable body begins -->
    /// Payload is `client_id`.
    /// `client_id` is generated in client side. Normally it can be `device_id` or just
    /// randomly generated string.
    /// `client_id` is used to identify client connections in server. Session is based on this field.
    /// It must be valid UTF-8 string, length shall be between 1 and 23 bytes.
    /// It can only contain the characters: "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    /// If `client_id` is invalid, the Server will reply ConnectAck Packet with return code
    /// 0x02(Identifier rejected).
    ///
    /// The Client Identifier (ClientID) identifies the Client to the Server. Each Client
    /// connecting to the Server has a unique ClientID. The ClientID MUST be used by Clients
    /// and by Servers to identify state that they hold relating to this MQTT Session
    /// between the Client and the Server [MQTT-3.1.3-2].
    ///
    /// The ClientID MUST be present and is the first field in the CONNECT packet Payload [MQTT-3.1.3-3].
    ///
    /// The ClientID MUST be a UTF-8 Encoded String [MQTT-3.1.3-4].
    ///
    /// The Server MUST allow ClientID’s which are between 1 and 23 UTF-8 encoded bytes
    /// in length, and that contain only the characters
    /// "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" [MQTT-3.1.3-5].
    ///
    /// A Server MAY allow a Client to supply a ClientID that has a length of zero bytes,
    /// however if it does so the Server MUST treat this as a special case and
    /// assign a unique ClientID to that Client [MQTT-3.1.3-6].
    ///
    /// It MUST then process the CONNECT packet as if the Client had provided
    /// that unique ClientID, and MUST return the Assigned Client Identifier
    /// in the CONNACK packet [MQTT-3.1.3-7]
    ///
    /// If the Server rejects the ClientID it MAY respond to the CONNECT packet
    /// with a CONNACK using Reason Code 0x85 (Client Identifier not valid),
    /// and then it MUST close the Network Connection [MQTT-3.1.3-8].
    client_id: StringData,

    /// If the Will Flag is set to 1, the Will Properties is the next field in the Payload.
    ///
    /// The Will Properties field defines the Application Message properties to be sent
    /// with the Will Message when it is published, and properties which define
    /// when to publish the Will Message. The Will Properties consists of
    /// a Property Length and the Properties.
    will_properties: Properties,

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

/// Properties available in connect packet.
pub const CONNECT_PROPERTIES: &[PropertyType] = &[
    PropertyType::SessionExpiryInterval,
    PropertyType::ReceiveMaximum,
    PropertyType::MaximumPacketSize,
    PropertyType::TopicAliasMaximum,
    PropertyType::RequestProblemInformation,
    PropertyType::UserProperty,
    PropertyType::AuthenticationMethod,
    PropertyType::AuthenticationData,
];

/// Properties available in connect-will.
pub const CONNECT_WILL_PROPERTIES: &[PropertyType] = &[
    PropertyType::WillDelayInterval,
    PropertyType::PayloadFormatIndicator,
    PropertyType::MessageExpiryInterval,
    PropertyType::ContentType,
    PropertyType::ResponseTopic,
    PropertyType::CorrelationData,
    PropertyType::UserProperty,
];

impl ConnectPacket {
    /// Create a new connect packet.
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
            keep_alive: KeepAlive::new(60),
            client_id,
            ..Self::default()
        })
    }

    /// Update protocol level.
    pub fn set_protcol_level(&mut self, level: ProtocolLevel) -> &mut Self {
        self.protocol_level = level;
        self
    }

    /// Get current mqtt protocol level.
    #[must_use]
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

    /// Update keep-alive value.
    pub fn set_keep_alive(&mut self, keep_alive: u16) -> &mut Self {
        self.keep_alive = KeepAlive::new(keep_alive);
        self
    }

    /// Get current keep-alive value.
    #[must_use]
    pub const fn keep_alive(&self) -> u16 {
        self.keep_alive.value()
    }

    /// Update will-retain flag.
    pub fn set_will_retain(&mut self, will_retain: bool) -> &mut Self {
        self.connect_flags.set_will_retain(will_retain);
        self
    }

    /// Get current will-retain flag.
    #[must_use]
    pub const fn will_retain(&self) -> bool {
        self.connect_flags.will_retain()
    }

    /// Update will-qos value.
    pub fn set_will_qos(&mut self, qos: QoS) -> &mut Self {
        self.connect_flags.set_will_qos(qos);
        self
    }

    /// Get current will-qos value.
    #[must_use]
    pub const fn will_qos(&self) -> QoS {
        self.connect_flags.will_qos()
    }

    /// Update will flag.
    pub fn set_will(&mut self, will: bool) -> &mut Self {
        self.connect_flags.set_will(will);
        self
    }

    /// Get current will flag.
    #[must_use]
    pub const fn will(&self) -> bool {
        self.connect_flags.will()
    }

    /// Update clean-session flag.
    pub fn set_clean_session(&mut self, clean_session: bool) -> &mut Self {
        self.connect_flags.set_clean_session(clean_session);
        self
    }

    /// Get clean-session flag.
    #[must_use]
    pub const fn clean_session(&self) -> bool {
        self.connect_flags.clean_session()
    }

    /// Get a mutable reference to property list.
    pub fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Get a reference to property list.
    #[must_use]
    pub const fn properties(&self) -> &Properties {
        &self.properties
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
    /// Returns error if `username` is out of range.
    pub fn set_username(&mut self, username: Option<&str>) -> Result<&mut Self, DecodeError> {
        if let Some(username) = username {
            self.username = StringData::from(username)?;
            self.connect_flags.set_has_username(true);
        } else {
            self.connect_flags.set_has_username(false);
            self.username = StringData::new();
        }
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
    /// Returns error if `password` is out of range.
    pub fn set_password(&mut self, password: Option<&[u8]>) -> Result<&mut Self, EncodeError> {
        match password {
            Some(password) => {
                self.connect_flags.set_has_password(true);
                self.password = BinaryData::from_slice(password)?;
            }
            None => {
                self.connect_flags.set_has_password(false);
                self.password.clear();
            }
        }
        Ok(self)
    }

    /// Get current password value.
    #[must_use]
    pub fn password(&self) -> &[u8] {
        self.password.as_ref()
    }

    /// Get a mutable reference to will property list.
    pub fn will_properties_mut(&mut self) -> &mut Properties {
        &mut self.will_properties
    }

    /// Get a reference to will property list.
    #[must_use]
    pub const fn will_properties(&self) -> &Properties {
        &self.will_properties
    }

    /// Update will topic.
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

    /// Get current will topic.
    pub fn will_topic(&self) -> Option<&str> {
        self.will_topic.as_ref().map(AsRef::as_ref)
    }

    /// Update will message bytes.
    ///
    /// # Errors
    ///
    /// Returns error if `message` is out of range.
    pub fn set_will_message(&mut self, message: &[u8]) -> Result<&mut Self, EncodeError> {
        self.will_message = BinaryData::from_slice(message)?;
        Ok(self)
    }

    /// Get will message bytes.
    #[must_use]
    pub fn will_message(&self) -> &[u8] {
        self.will_message.as_ref()
    }

    fn get_fixed_header(&self) -> Result<FixedHeader, VarIntError> {
        let mut remaining_length = self.protocol_name.bytes()
            + ProtocolLevel::bytes()
            + ConnectFlags::bytes()
            + KeepAlive::bytes()
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

        // A Server which support multiple protocols uses the Protocol Name to determine
        // whether the data is MQTT. The protocol name MUST be the UTF-8 String "MQTT".
        // If the Server does not want to accept the CONNECT, and wishes to reveal that
        // it is an MQTT Server it MAY send a CONNACK packet with
        // Reason Code of 0x84 (Unsupported Protocol Version), and then
        // it MUST close the Network Connection [MQTT-3.1.2-1].
        let protocol_name = StringData::decode(ba)?;
        if protocol_name.as_ref() != PROTOCOL_NAME {
            return Err(DecodeError::InvalidProtocolName);
        }

        // A Server which supports multiple versions of the MQTT protocol
        // uses the Protocol Version to determine which version of MQTT
        // the Client is using. If the Protocol Version is not 5 and the Server does not want
        // to accept the CONNECT packet, the Server MAY send a CONNACK packet
        // with Reason Code 0x84 (Unsupported Protocol Version) and then
        // MUST close the Network Connection [MQTT-3.1.2-2].
        let protocol_level = ProtocolLevel::try_from(ba.read_byte()?)?;

        let connect_flags = ConnectFlags::decode(ba)?;
        if !connect_flags.will()
            && (connect_flags.will_qos() != QoS::AtMostOnce || connect_flags.will_retain())
        {
            return Err(DecodeError::InvalidConnectFlags);
        }

        // If the User Name Flag is set to 0, the Password Flag MUST be set to 0 [MQTT-3.1.2-22].
        if !connect_flags.has_username() && connect_flags.has_password() {
            return Err(DecodeError::InvalidConnectFlags);
        }

        let keep_alive = KeepAlive::decode(ba)?;
        validate_keep_alive(keep_alive)?;

        let properties = Properties::decode(ba);
        let properties = match properties {
            Ok(properties) => properties,
            Err(err) => {
                log::error!("err: {:?}", err);
                return Err(DecodeError::InvalidPropertyType);
            }
        };
        if let Err(property_type) = check_property_type_list(properties.props(), CONNECT_PROPERTIES)
        {
            log::error!(
                "v5/ConnectPacket: property type {:?} cannot be used in properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        let client_id = StringData::decode(ba).map_err(|_err| DecodeError::InvalidClientId)?;
        if client_id.is_empty() && !connect_flags.clean_session() {
            // If clean_session is false, a client_id is always required.
            return Err(DecodeError::InvalidClientId);
        }
        validate_client_id(client_id.as_ref())?;

        let will_properties = if connect_flags.will() {
            Properties::decode(ba)?
        } else {
            Properties::new()
        };
        if let Err(property_type) =
            check_property_type_list(will_properties.props(), CONNECT_WILL_PROPERTIES)
        {
            log::error!(
                "v5/ConnectPacket: property type {:?} cannot be used in will properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
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
            properties,
            client_id,
            will_properties,
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
            0x10, 0x15, 0x00, 0x04, 0x4d, 0x51, 0x54, 0x54, 0x05, 0x02, 0x00, 0x3c, 0x00, 0x00,
            0x08, 0x77, 0x76, 0x50, 0x54, 0x58, 0x63, 0x43, 0x77,
        ];
        let mut ba = ByteArray::new(&buf);
        let packet = ConnectPacket::decode(&mut ba);
        assert!(packet.is_ok());
        let packet = packet.unwrap();
        assert_eq!(packet.client_id(), "wvPTXcCw");
    }
}
