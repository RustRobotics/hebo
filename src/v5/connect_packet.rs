// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::convert::TryFrom;
use std::io::Write;

use super::{
    property::check_property_type_list, FixedHeader, Packet, PacketType, Properties, Property,
    PropertyType,
};
use crate::utils::{validate_client_id, validate_two_bytes_data};
use crate::{
    consts, BinaryData, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket,
    ProtocolLevel, PubTopic, QoS, StringData,
};

/// Structure of `ConnectFlags` is:
/// ```txt
///         7               6              5          4-3          2            1             0
/// +---------------+---------------+-------------+----------+-----------+---------------+----------+
/// | Username Flag | Password Flag | Will Retain | Will QoS | Will Flag | Clean Session | Reserved |
/// +---------------+---------------+-------------+----------+-----------+---------------+----------+
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConnectFlags {
    /// Position: bit 7 of the Connect Flags.
    ///
    /// If the User Name Flag is set to 0, a User Name MUST NOT be present in the Payload [MQTT-3.1.2-16].
    ///
    /// If the User Name Flag is set to 1, a User Name MUST be present in the Payload [MQTT-3.1.2-17].
    username: bool,

    /// Position: bit 6 of the Connect Flags.
    ///
    /// If the Password Flag is set to 0, a Password MUST NOT be present in the Payload [MQTT-3.1.2-18].
    ///
    /// If the Password Flag is set to 1, a Password MUST be present in the Payload [MQTT-3.1.2-19].
    password: bool,

    /// Position: bit 5 of the Connect Flags.
    ///
    /// This bit specifies if the Will Message is to be retained when it is published.
    ///
    /// If the Will Flag is set to 0, then Will Retain MUST be set to 0 [MQTT-3.1.2-13].
    ///
    /// If the Will Flag is set to 1 and Will Retain is set to 0, the Server MUST publish
    /// the Will Message as a non-retained message [MQTT-3.1.2-14].
    ///
    /// If the Will Flag is set to 1 and Will Retain is set to 1, the Server MUST publish
    /// the Will Message as a retained message [MQTT-3.1.2-15].
    will_retain: bool,

    /// Position: bits 4 and 3 of the Connect Flags.
    ///
    /// These two bits specify the QoS level to be used when publishing the Will Message.
    ///
    /// If the Will Flag is set to 0, then the Will QoS MUST be set to 0 (0x00) [MQTT-3.1.2-11].
    ///
    /// If the Will Flag is set to 1, the value of Will QoS can be 0 (0x00), 1 (0x01), or 2 (0x02) [MQTT-3.1.2-12].
    /// A value of 3 (0x03) is a Malformed Packet.
    will_qos: QoS,

    /// Position: bit 2 of the Connect Flags.
    ///
    /// If the Will Flag is set to 1 this indicates that a Will Message MUST be stored
    /// on the Server and associated with the Session [MQTT-3.1.2-7].
    ///
    /// The Will Message consists of the Will Properties, Will Topic, and Will
    /// Payload fields in the CONNECT Payload. The Will Message MUST be published
    /// after the Network Connection is subsequently closed and either the Will Delay Interval
    /// has elapsed or the Session ends, unless the Will Message has been deleted
    /// by the Server on receipt of a DISCONNECT packet with Reason Code 0x00 (Normal disconnection)
    /// or a new Network Connection for the ClientID is opened before the Will Delay Interval
    /// has elapsed [MQTT-3.1.2-8].
    ///
    /// Situations in which the Will Message is published include, but are not limited to:
    /// - An I/O error or network failure detected by the Server.
    /// - The Client fails to communicate within the Keep Alive time.
    /// - The Client closes the Network Connection without first sending a DISCONNECT packet with
    ///   a Reason Code 0x00 (Normal disconnection).
    /// - The Server closes the Network Connection without first receiving a DISCONNECT packet with
    ///   a Reason Code 0x00 (Normal disconnection).
    ///
    /// If the Will Flag is set to 1, the Will Properties, Will Topic, and Will Payload fields
    /// MUST be present in the Payload [MQTT-3.1.2-9].
    ///
    /// The Will Message MUST be removed from the stored Session State in the Server
    /// once it has been published or the Server has received a DISCONNECT packet with a Reason
    /// Code of 0x00 (Normal disconnection) from the Client [MQTT-3.1.2-10].
    ///
    /// The Server SHOULD publish Will Messages promptly after the Network Connection is closed
    /// and the Will Delay Interval has passed, or when the Session ends, whichever occurs first.
    /// In the case of a Server shutdown or failure, the Server MAY defer publication of Will Messages
    /// until a subsequent restart. If this happens, there might be a delay between the time
    /// the Server experienced failure and when the Will Message is published.
    will: bool,

    /// Position: bit 1 of the Connect Flags.
    /// To control how to handle Session State.
    ///
    /// If a CONNECT packet is received with Clean Start is set to 1, the Client
    /// and Server MUST discard any existing Session and start a new Session [MQTT-3.1.2-4].
    /// Consequently, the Session Present flag in CONNACK is always set to 0 if Clean Start is set to 1.
    ///
    /// If a CONNECT packet is received with Clean Start set to 0 and there is
    /// a Session associated with the Client Identifier, the Server MUST resume communications
    /// with the Client based on state from the existing Session [MQTT-3.1.2-5].
    ///
    /// If a CONNECT packet is received with Clean Start set to 0 and there is no Session
    /// associated with the Client Identifier, the Server MUST create a new Session [MQTT-3.1.2-6].
    clean_session: bool,
}

impl ConnectFlags {
    pub fn set_username(&mut self, username: bool) -> &mut Self {
        self.username = username;
        self
    }

    pub fn username(&self) -> bool {
        self.username
    }

    pub fn set_password(&mut self, password: bool) -> &mut Self {
        self.password = password;
        self
    }

    pub fn password(&self) -> bool {
        self.password
    }

    pub fn set_will_retain(&mut self, will_retain: bool) -> &mut Self {
        self.will_retain = will_retain;
        self
    }

    pub fn will_retain(&self) -> bool {
        self.will_retain
    }

    pub fn set_will_qos(&mut self, qos: QoS) -> &mut Self {
        self.will_qos = qos;
        self
    }

    pub fn will_qos(&self) -> QoS {
        self.will_qos
    }

    pub fn set_will(&mut self, will: bool) -> &mut Self {
        if !will {
            self.will_qos = QoS::AtMostOnce;
            self.will_retain = false;
        }
        self.will = will;
        self
    }

    pub fn will(&self) -> bool {
        self.will
    }

    pub fn set_clean_session(&mut self, clean_session: bool) -> &mut Self {
        self.clean_session = clean_session;
        self
    }

    pub fn clean_session(&self) -> bool {
        self.clean_session
    }
}

impl Default for ConnectFlags {
    fn default() -> Self {
        ConnectFlags {
            username: false,
            password: false,
            will_retain: false,
            will_qos: QoS::AtMostOnce,
            will: false,
            clean_session: true,
        }
    }
}

impl EncodePacket for ConnectFlags {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let flags = {
            let username = if self.username {
                0b1000_0000
            } else {
                0b0000_0000
            };
            let password = if self.password {
                0b0100_0000
            } else {
                0b0000_0000
            };
            let will_retian = if self.will_retain {
                0b0010_0000
            } else {
                0b0000_0000
            };

            let will_qos = match self.will_qos {
                QoS::AtMostOnce => 0b0000_0000,
                QoS::AtLeastOnce => 0b0000_1000,
                QoS::ExactOnce => 0b0001_0000,
            };

            let will = if self.will { 0b0000_0100 } else { 0b0000_0000 };

            let clean_session = if self.clean_session {
                0b0000_0010
            } else {
                0b0000_0000
            };

            username | password | will_retian | will_qos | will | clean_session
        };
        v.push(flags);

        Ok(1)
    }
}

impl DecodePacket for ConnectFlags {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let flags = ba.read_byte()?;
        let username = flags & 0b1000_0000 == 0b1000_0000;
        let password = flags & 0b0100_0000 == 0b0100_0000;
        let will_retain = flags & 0b0010_0000 == 0b0010_0000;
        let will_qos = QoS::try_from((flags & 0b0001_1000) >> 3)?;
        let will = flags & 0b0000_0100 == 0b0000_0100;
        let clean_session = flags & 0b0000_0010 == 0b0000_0010;

        // The Server MUST validate that the reserved flag in the CONNECT packet
        // is set to 0 [MQTT-3.1.2-3]. If the reserved flag is not 0 it is a Malformed Packet.
        let reserved_is_zero = flags & 0b0000_0001 == 0b0000_0000;
        if !reserved_is_zero {
            return Err(DecodeError::InvalidConnectFlags);
        }

        // If the User Name Flag is set to 0, the Password Flag MUST be set to 0. [MQTT-3.1.2-22]
        if !username && password {
            return Err(DecodeError::InvalidConnectFlags);
        }

        Ok(ConnectFlags {
            username,
            password,
            will_retain,
            will_qos,
            will,
            clean_session,
        })
    }
}

/// `ConnectPacket` consists of three parts:
/// * FixedHeader
/// * VariableHeader
/// * Payload
/// Note that fixed header part is same in all packets so that we just ignore it.
///
/// Basic struct of ConnectPacket is as below:
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConnectPacket {
    /// Protocol name can only be `MQTT` in specification.
    protocol_name: String,

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
    keep_alive: u16,

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
    client_id: String,

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
    will_message: Vec<u8>,

    /// If the `username` flag is true in `connect_flags`, then `username` field must be set.
    /// It is a valid UTF-8 string.
    username: StringData,

    /// If the `password` flag is true in `connect_flags`, then `password` field must be set.
    /// It consists of 0 to 64k bytes of binary data.
    password: BinaryData,
}

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
    pub fn new(client_id: &str) -> Result<ConnectPacket, EncodeError> {
        validate_client_id(client_id)?;
        Ok(ConnectPacket {
            protocol_name: consts::PROTOCOL_NAME.to_string(),
            keep_alive: 60,
            client_id: client_id.to_string(),
            ..ConnectPacket::default()
        })
    }

    pub fn set_protcol_level(&mut self, level: ProtocolLevel) -> &mut Self {
        self.protocol_level = level;
        self
    }

    pub fn protocol_level(&self) -> ProtocolLevel {
        self.protocol_level
    }

    pub fn connect_flags(&self) -> &ConnectFlags {
        &self.connect_flags
    }

    pub fn set_keep_alive(&mut self, keep_alive: u16) -> &mut Self {
        self.keep_alive = keep_alive;
        self
    }

    pub fn keep_alive(&self) -> u16 {
        self.keep_alive
    }

    pub fn set_properties(&mut self, properties: &[Property]) -> &mut Self {
        self.properties = properties.to_vec();
        self
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn set_client_id(&mut self, id: &str) -> Result<&mut Self, EncodeError> {
        validate_client_id(id)?;
        self.client_id.clear();
        self.client_id.push_str(id);
        Ok(self)
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn set_qos(&mut self, qos: QoS) -> &mut Self {
        self.connect_flags.will_qos = qos;
        self
    }

    pub fn set_username(&mut self, username: &str) -> Result<&mut Self, DecodeError> {
        if username.is_empty() {
            self.connect_flags.username = false;
            self.connect_flags.password = false;
            self.username.clear();
            self.password.clear();
        } else {
            self.username = StringData::from_str(username)?;
            self.connect_flags.username = true;
        }
        Ok(self)
    }

    pub fn username(&self) -> &str {
        self.username.as_ref()
    }

    pub fn set_password(&mut self, password: &[u8]) -> Result<&mut Self, EncodeError> {
        self.password = BinaryData::from_slice(password)?;
        Ok(self)
    }

    pub fn password(&self) -> &[u8] {
        self.password.as_ref()
    }

    pub fn set_will_properties(&mut self, properties: &[Property]) -> &mut Self {
        self.will_properties = properties.to_vec();
        self
    }

    pub fn will_properties(&self) -> &Properties {
        &self.will_properties
    }

    pub fn set_will_topic(&mut self, topic: Option<&str>) -> Result<&mut Self, EncodeError> {
        if let Some(topic) = topic {
            if topic.is_empty() {
                self.will_topic = None;
                self.connect_flags.will = false;
            } else {
                self.will_topic = Some(PubTopic::new(topic)?);
                self.connect_flags.will = true;
            }
        } else {
            self.connect_flags.will = false;
            self.will_topic = None;
        }
        Ok(self)
    }

    pub fn will_topic(&self) -> Option<&str> {
        self.will_topic.as_ref().map(|s| s.as_ref())
    }

    pub fn set_will_message(&mut self, message: &[u8]) -> Result<&mut Self, DecodeError> {
        validate_two_bytes_data(message)?;
        self.will_message = message.to_vec();
        Ok(self)
    }

    pub fn will_message(&self) -> &[u8] {
        &self.will_message
    }
}

#[inline]
pub fn validate_keep_alive(keep_alive: u16) -> Result<(), DecodeError> {
    if keep_alive != 0 && keep_alive < 5 {
        Err(DecodeError::OtherErrors)
    } else {
        Ok(())
    }
}

impl Packet for ConnectPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Connect
    }
}

impl EncodePacket for ConnectPacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = v.len();

        let mut remaining_length = consts::PROTOCOL_NAME_LENGTH // protocol_name_len
            + self.protocol_name.len() // b"MQTT" protocol name
            + 1 // protocol_level
            + 1 // connect_flags
            + 2 // keep_alive
            + consts::CLIENT_ID_LENGTH_BYTES // client_id_len
            + self.client_id.len();

        // Check username/password/topic/message.
        if self.connect_flags.will {
            assert!(self.will_topic.is_some());
            if let Some(will_topic) = &self.will_topic {
                remaining_length += will_topic.bytes();
            }
            remaining_length += 2 + self.will_message.len();
        }
        if self.connect_flags.username {
            remaining_length += self.username.bytes();
        }
        if self.connect_flags.password {
            remaining_length += self.password.bytes();
        }

        let fixed_header = FixedHeader::new(PacketType::Connect, remaining_length)?;
        // Write fixed header
        fixed_header.encode(v)?;

        // Write variable header
        v.write_u16::<BigEndian>(self.protocol_name.len() as u16)?;
        v.write_all(&self.protocol_name.as_bytes())?;
        self.protocol_level.encode(v)?;
        self.connect_flags.encode(v)?;
        v.write_u16::<BigEndian>(self.keep_alive)?;

        // Write payload
        v.write_u16::<BigEndian>(self.client_id.len() as u16)?;
        v.write_all(&self.client_id.as_bytes())?;
        if self.connect_flags.will {
            assert!(self.will_topic.is_some());
            if let Some(will_topic) = &self.will_topic {
                will_topic.encode(v)?;
            }

            v.write_u16::<BigEndian>(self.will_message.len() as u16)?;
            v.write_all(&self.will_message)?;
        }
        if self.connect_flags.username {
            self.username.encode(v)?;
        }
        if self.connect_flags.password {
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
        let protocol_name_len = ba.read_u16()? as usize;
        let protocol_name = ba.read_string(protocol_name_len)?;
        if protocol_name != consts::PROTOCOL_NAME {
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
        if !connect_flags.username() && connect_flags.password() {
            return Err(DecodeError::InvalidConnectFlags);
        }

        let keep_alive = ba.read_u16()?;
        validate_keep_alive(keep_alive)?;

        let client_id_len = ba.read_u16()? as usize;
        let client_id = if client_id_len > 0 {
            let client_id = ba
                .read_string(client_id_len)
                .map_err(|_err| DecodeError::InvalidClientId)?;
            validate_client_id(&client_id).map_err(|_err| DecodeError::InvalidClientId)?;
            client_id
        } else {
            if !connect_flags.clean_session() {
                return Err(DecodeError::InvalidClientId);
            }
            // An empty client id is used.
            "".to_string()
        };

        let will_topic = if connect_flags.will {
            Some(PubTopic::decode(ba)?)
        } else {
            None
        };
        let will_message = if connect_flags.will {
            let will_message_len = ba.read_u16()? as usize;
            ba.read_bytes(will_message_len)?.to_vec()
        } else {
            Vec::new()
        };

        let username = if connect_flags.username {
            StringData::decode(ba)?
        } else {
            StringData::new()
        };

        let password = if connect_flags.password {
            BinaryData::decode(ba)?
        } else {
            BinaryData::new()
        };

        let properties = Properties::decode(ba)?;
        if let Err(property_type) = check_property_type_list(&properties, CONNECT_PROPERTIES) {
            log::error!(
                "v5/ConnectPacket: property type {:?} cannot be used in properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        let will_properties = Properties::decode(ba)?;
        if let Err(property_type) =
            check_property_type_list(&will_properties, CONNECT_WILL_PROPERTIES)
        {
            log::error!(
                "v5/ConnectPacket: property type {:?} cannot be used in will properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        Ok(ConnectPacket {
            protocol_name,
            protocol_level,
            keep_alive,
            connect_flags,
            properties,
            client_id,
            will_topic,
            will_properties,
            will_message,
            username,
            password,
        })
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
