// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::convert::TryFrom;
use std::io::Write;

use super::{FixedHeader, Packet, PacketType};
use crate::utils::{validate_client_id, validate_two_bytes_data, validate_utf8_string};
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
    /// `username` field specifies whether `username` shall be presented in the Payload.
    username: bool,

    /// `password` field specifies whether `password` shall be presented in the Payload.
    /// If `username` field is false, then this field shall be false too.
    password: bool,

    /// `retain` field specifies if the Will Message is to be Retained when it is published.
    /// If the `will` field is false, then the `retain` field msut be false.
    will_retain: bool,

    /// QoS level to be used in the Will Message.
    will_qos: QoS,

    /// If this field is set to true, a Will Message will be stored on the Server side when
    /// Client connected, and this message must be sent back when Client connection
    /// is closed abnormally unless it is deleted by the Server on receipt of a Disconnect Packet.
    ///
    /// This Will Message is used mainly to handle errors:
    /// * I/O error or network error
    /// * Keep alive timeout
    /// * network disconnected without Disconnect Packet
    /// * protocol error
    will: bool,

    /// To control how to handle Session State.
    /// If `clean_sessions` is true, the Client and Server must discard any previous Session State
    /// and start a new once until end of Disconnect. So that State data cannot be reused in subsequent
    /// connections.
    ///
    /// Client side of Session State consists of:
    /// * QoS 1 and QoS 2 messages which have been sent to server but not be acknowledged yet.
    /// * QoS 2 messages which have been received from server but have not been fully acknowledged yet.
    ///
    /// Server side of Session State consists of:
    /// * Client subscriptions
    /// * QoS 1 and QoS 2 messages which have been sent to subscribed Clients, but have not been acknowledged yet.
    /// * QoS 1 and QoS 2 messages pending transmission to the Client.
    /// * QoS 2 messages which have been received from the Clients, but have not been fully acknowledged yet.
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

        // The Server MUST validate that the reserved flag in the CONNECT Control Packet
        // is set to zero and disconnect the Client if it is not zero [MQTT-3.1.2-3].
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
    keep_alive: u16,

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
    pub fn new(client_id: &str) -> Result<ConnectPacket, EncodeError> {
        validate_client_id(client_id)?;
        Ok(ConnectPacket {
            protocol_name: consts::PROTOCOL_NAME.to_string(),
            keep_alive: 60,
            client_id: client_id.to_string(),
            ..ConnectPacket::default()
        })
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

    pub fn set_keep_alive(&mut self, keep_alive: u16) -> &mut Self {
        self.keep_alive = keep_alive;
        self
    }

    pub fn keep_alive(&self) -> u16 {
        self.keep_alive
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
        validate_utf8_string(username)?;
        self.username = username.to_string();
        Ok(self)
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn set_password(&mut self, password: &[u8]) -> Result<&mut Self, DecodeError> {
        validate_two_bytes_data(password)?;
        self.password = password.to_vec();
        Ok(self)
    }

    pub fn password(&self) -> &[u8] {
        &self.password
    }

    pub fn set_will_topic(&mut self, topic: &str) -> Result<&mut Self, DecodeError> {
        validate_utf8_string(topic)?;
        topic::validate_pub_topic(topic)?;
        self.will_topic = topic.to_string();
        Ok(self)
    }

    pub fn will_topic(&self) -> &str {
        &self.will_topic
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

        let protocol_name_len = ba.read_u16()? as usize;
        let protocol_name = ba.read_string(protocol_name_len)?;
        if protocol_name != consts::PROTOCOL_NAME {
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
        if !connect_flags.username() && connect_flags.password() {
            return Err(DecodeError::InvalidConnectFlags);
        }

        let keep_alive = ba.read_u16()?;
        validate_keep_alive(keep_alive)?;

        let client_id_len = ba.read_u16()? as usize;
        // A Server MAY allow a Client to supply a ClientId that has a length of zero bytes,
        // however if it does so the Server MUST treat this as a special case and assign
        // a unique ClientId to that Client. It MUST then process the CONNECT packet
        // as if the Client had provided that unique ClientId [MQTT-3.1.3-6].
        let client_id = if client_id_len > 0 {
            let client_id = ba
                .read_string(client_id_len)
                .map_err(|_err| DecodeError::InvalidClientId)?;
            validate_client_id(&client_id).map_err(|_err| DecodeError::InvalidClientId)?;
            client_id
        } else {
            // If the Client supplies a zero-byte ClientId, the Client MUST also set CleanSession
            // to 1 [MQTT-3.1.3-7].
            //
            // If the Client supplies a zero-byte ClientId with CleanSession set to 0, the Server
            // MUST respond to the CONNECT Packet with a CONNACK return code 0x02 (Identifier rejected)
            // and then close the Network Connection [MQTT-3.1.3-8].
            if !connect_flags.clean_session() {
                return Err(DecodeError::InvalidClientId);
            }
            "".to_string()
        };

        let will_topic = if connect_flags.will {
            let will_topic_len = ba.read_u16()? as usize;
            let will_topic = ba.read_string(will_topic_len)?;
            validate_utf8_string(&will_topic)?;
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
