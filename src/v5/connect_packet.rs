// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::convert::TryFrom;
use std::io::Write;

use super::{FixedHeader, Packet, PacketType};
use crate::utils::{self, StringError};
use crate::{
    consts, topic, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, ProtocolLevel,
    QoS,
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConnectPacket {
    /// Protocol name can only be `MQTT` in specification.
    protocol_name: String,

    protocol_level: ProtocolLevel,

    connect_flags: ConnectFlags,

    /// Time interval between two packets in seconds.
    /// Client must send PingRequest Packet before exceeding this interval.
    /// If this value is not zero and time exceeds after last packet, the Server
    /// will disconnect the network.
    ///
    /// If this value is zero, the Server is not required to disconnect the network.
    pub keep_alive: u16,

    /// Payload is `client_id`.
    /// `client_id` is generated in client side. Normally it can be `device_id` or just
    /// randomly generated string.
    /// `client_id` is used to identify client connections in server. Session is based on this field.
    /// It must be valid UTF-8 string, length shall be between 1 and 23 bytes.
    /// It can only contain the characters: "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    /// If `client_id` is invalid, the Server will reply ConnectAck Packet with return code
    /// 0x02(Identifier rejected).
    client_id: String,

    /// If the `will` flag is true in `connect_flags`, then `will_topic` field must be set.
    /// It will be used as the topic of Will Message.
    will_topic: String,

    /// If the `will` flag is true in `connect_flags`, then `will_message` field must be set.
    /// It will be used as the payload of Will Message.
    /// It consists of 0 to 64k bytes of binary data.
    will_message: Vec<u8>,

    /// If the `username` flag is true in `connect_flags`, then `username` field must be set.
    /// It is a valid UTF-8 string.
    username: String,

    /// If the `password` flag is true in `connect_flags`, then `password` field must be set.
    /// It consists of 0 to 64k bytes of binary data.
    password: Vec<u8>,
}

impl ConnectPacket {
    pub fn new(client_id: &str) -> ConnectPacket {
        // TODO(Shaohua): Validate client_id.
        ConnectPacket {
            protocol_name: consts::PROTOCOL_NAME.to_string(),
            keep_alive: 60,
            client_id: client_id.to_string(),
            ..ConnectPacket::default()
        }
    }

    pub fn set_protcol_level(&mut self, level: ProtocolLevel) -> &Self {
        self.protocol_level = level;
        self
    }

    pub fn protocol_level(&self) -> ProtocolLevel {
        self.protocol_level
    }

    pub fn set_connect_flags(&mut self, flags: ConnectFlags) -> &Self {
        self.connect_flags = flags;
        self
    }

    pub fn connect_flags(&self) -> &ConnectFlags {
        &self.connect_flags
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
        utils::validate_utf8_string(username)?;
        self.username = username.to_string();
        Ok(self)
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn set_password(&mut self, password: &[u8]) -> Result<&mut Self, DecodeError> {
        utils::validate_two_bytes_data(password)?;
        self.password = password.to_vec();
        Ok(self)
    }

    pub fn password(&self) -> &[u8] {
        &self.password
    }

    pub fn set_will_topic(&mut self, topic: &str) -> Result<&mut Self, DecodeError> {
        utils::validate_utf8_string(topic)?;
        topic::validate_pub_topic(topic)?;
        self.will_topic = topic.to_string();
        Ok(self)
    }

    pub fn will_topic(&self) -> &str {
        &self.will_topic
    }

    pub fn set_will_message(&mut self, message: &[u8]) -> Result<&mut Self, DecodeError> {
        utils::validate_two_bytes_data(message)?;
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

/// ClientId is based on rules below:
/// TODO(Shaohua): Add more spec rules
pub fn validate_client_id(id: &str) -> Result<(), StringError> {
    if id.is_empty() || id.len() > 23 {
        return Err(StringError::InvalidLength);
    }
    for byte in id.bytes() {
        if !((b'0'..=b'9').contains(&byte)
            || (b'a'..=b'z').contains(&byte)
            || (b'A'..=b'Z').contains(&byte))
        {
            return Err(StringError::InvalidChar);
        }
    }
    Ok(())
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
            remaining_length += 2 + self.will_topic.len();
            remaining_length += 2 + self.will_message.len();
        }
        if self.connect_flags.username {
            remaining_length += 2 + self.username.len();
        }
        if self.connect_flags.password {
            remaining_length += 2 + self.password.len();
        }

        let fixed_header = FixedHeader::new(PacketType::Connect, remaining_length);
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
            v.write_u16::<BigEndian>(self.will_topic.len() as u16)?;
            v.write_all(&self.will_topic.as_bytes())?;

            v.write_u16::<BigEndian>(self.will_message.len() as u16)?;
            v.write_all(&self.will_message)?;
        }
        if self.connect_flags.username {
            v.write_u16::<BigEndian>(self.username.len() as u16)?;
            v.write_all(&self.username.as_bytes())?;
        }
        if self.connect_flags.password {
            v.write_u16::<BigEndian>(self.password.len() as u16)?;
            v.write_all(&self.password)?;
        }

        Ok(v.len() - old_len)
    }
}

impl Packet for ConnectPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Connect
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
            "".to_string()
        };

        let will_topic = if connect_flags.will {
            let will_topic_len = ba.read_u16()? as usize;
            let will_topic = ba.read_string(will_topic_len)?;
            utils::validate_utf8_string(&will_topic)?;
            topic::validate_pub_topic(&will_topic)?;
            will_topic
        } else {
            String::new()
        };
        let will_message = if connect_flags.will {
            let will_message_len = ba.read_u16()? as usize;
            ba.read_bytes(will_message_len)?.to_vec()
        } else {
            Vec::new()
        };

        let username = if connect_flags.username {
            let username_len = ba.read_u16()? as usize;
            ba.read_string(username_len)?
        } else {
            String::new()
        };

        let password = if connect_flags.password {
            let password_len = ba.read_u16()? as usize;
            ba.read_bytes(password_len)?.to_vec()
        } else {
            Vec::new()
        };

        Ok(ConnectPacket {
            protocol_name,
            protocol_level,
            keep_alive,
            connect_flags,
            client_id,
            will_topic,
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
