// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;
use std::default::Default;
use std::io::{self, Write};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

use crate::base::*;
use crate::error::Error;

/// Current version of MQTT protocol can be:
/// * 3.1
/// * 3.1.1
/// * 5.0
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProtocolLevel {
    V31 = 3,
    V311 = 4,
    V5 = 5,
}

impl Default for ProtocolLevel {
    fn default() -> Self {
        ProtocolLevel::V311
    }
}

impl TryFrom<u8> for ProtocolLevel {
    type Error = Error;

    fn try_from(v: u8) -> Result<ProtocolLevel, Self::Error> {
        match v {
            3 => Ok(ProtocolLevel::V31),
            4 => Ok(ProtocolLevel::V311),
            5 => Ok(ProtocolLevel::V5),
            _ => Err(Error::InvalidProtocolLevel),
        }
    }
}

impl ToNetPacket for ProtocolLevel {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        v.push(*self as u8);
        Ok(1)
    }
}

/// Structure of `ConnectFlags` is:
/// ```txt
///         7               6              5          4-3          2            1             0
/// +---------------+---------------+-------------+----------+-----------+---------------+----------+
/// | Username Flag | Password Flag | Will Retain | Will QoS | Will Flag | Clean Session | Reserved |
/// +---------------+---------------+-------------+----------+-----------+---------------+----------+
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConnectFlags {
    /// `username` field specifies whether `username` shall be presented in the Payload.
    pub username: bool,

    /// `password` field specifies whether `password` shall be presented in the Payload.
    /// If `username` field is false, then this field shall be false too.
    pub password: bool,

    /// `retain` field specifies if the Will Message is to be Retained when it is published.
    /// If the `will` field is false, then the `retain` field msut be false.
    pub will_retain: bool,

    /// QoS level to be used in the Will Message.
    pub will_qos: QoS,

    /// If this field is set to true, a Will Message will be stored on the Server side when
    /// Client connected, and this message must be sent back when Client connection
    /// is closed abnormally unless it is deleted by the Server on receipt of a Disconnect Packet.
    ///
    /// This Will Message is used mainly to handle errors:
    /// * I/O error or network error
    /// * Keep alive timeout
    /// * network disconnected without Disconnect Packet
    /// * protocol error
    pub will: bool,

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
    pub clean_session: bool,
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

impl ToNetPacket for ConnectFlags {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
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
        log::info!("connect flags: {:x?}", flags);
        v.push(flags);

        Ok(1)
    }
}

impl FromNetPacket for ConnectFlags {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let flags = buf[*offset];
        let username = flags & 0b1000_0000 == 0b1000_0000;
        let password = flags & 0b0100_0000 == 0b0100_0000;
        let will_retain = flags & 0b0010_0000 == 0b0010_0000;
        let will_qos = QoS::try_from((flags & 0b0001_1000) >> 3)?;
        let will = flags & 0b0000_0100 == 0b0000_0100;
        let clean_session = flags & 0b0000_0010 == 0b0000_0010;
        *offset += 1;
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
/// 7                            0
/// +----------------------------+
/// | protocol level             |
/// +----------------------------+
/// | connect flags              |
/// +----------------------------+
/// | keep alive                 |
/// |                            |
/// +----------------------------+
/// | client id length           |
/// |                            |
/// +----------------------------+
/// | client id string           |
/// +----------------------------+
/// | will topic length          |
/// |                            |
/// +----------------------------+
/// | will topic string          |
/// +----------------------------+
/// | will message length        |
/// | ...                        |
/// |                            |
/// +----------------------------+
/// | will message bytes         |
/// +----------------------------+
/// | username length            |
/// |                            |
/// +----------------------------+
/// | username string            |
/// +----------------------------+
/// | password length            |
/// |                            |
/// +----------------------------+
/// | password bytes             |
/// +----------------------------+
/// ```
///
// TODO(shaohua): UTF-8 string MUST NOT contain null characters
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ConnectPacket {
    /// Protocol name can only be `MQTT` in specification.
    protocol_name: String,
    pub protocol_level: ProtocolLevel,
    pub connect_flags: ConnectFlags,

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
        ConnectPacket {
            protocol_name: "MQTT".to_string(),
            keep_alive: 60,
            client_id: client_id.to_string(),
            ..ConnectPacket::default()
        }
    }

    pub fn protocol_name(&self) -> &str {
        &self.protocol_name
    }

    pub fn validate_client_id(id: &str) -> Result<(), Error> {
        if id.len() < 1 || id.len() > 23 {
            return Err(Error::InvalidClientId);
        }
        for byte in id.bytes() {
            if !((byte >= b'0' && byte <= b'9')
                || (byte >= b'a' && byte <= b'z')
                || (byte >= b'A' && byte <= b'Z'))
            {
                return Err(Error::InvalidClientId);
            }
        }
        Ok(())
    }

    pub fn set_client_id(&mut self, id: &str) -> Result<(), Error> {
        self.client_id.clear();
        ConnectPacket::validate_client_id(id)?;
        self.client_id.push_str(id);
        Ok(())
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn set_qos(&mut self, qos: QoS) {
        self.connect_flags.will_qos = qos;
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn set_password(&mut self, password: &[u8]) {
        self.password = password.to_vec();
    }

    pub fn password(&self) -> &[u8] {
        &self.password
    }

    pub fn set_will_topic(&mut self, topic: &str) {
        self.will_topic = topic.to_string();
    }

    pub fn will_topic(&self) -> &str {
        &self.will_topic
    }

    pub fn set_will_message(&mut self, message: &[u8]) {
        self.will_message = message.to_vec();
    }

    pub fn will_message(&self) -> &[u8] {
        &self.will_message
    }
}

impl ToNetPacket for ConnectPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = v.len();
        // FIXME(Shaohua): username/password/topic/message are ignored.
        let remaining_length = 2 // protocol_name_len
            + self.protocol_name.len() // b"MQTT" protocol name
            + 1 // protocol_level
            + 1 // connect_flags
            + 2 // keep_alive
            + 2 // client_id_len
            + self.client_id.len();

        let fixed_header = FixedHeader {
            packet_type: PacketType::Connect,
            packet_flags: PacketFlags::Connect,
            remaining_length: RemainingLength(remaining_length as u32),
        };
        // Write fixed header
        fixed_header.to_net(v)?;

        // Write variable header
        v.write_u16::<BigEndian>(self.protocol_name.len() as u16)?;
        v.write_all(&self.protocol_name.as_bytes())?;
        self.protocol_level.to_net(v)?;
        self.connect_flags.to_net(v)?;
        v.write_u16::<BigEndian>(self.keep_alive)?;

        // Write payload
        v.write_u16::<BigEndian>(self.client_id.len() as u16)?;
        v.write_all(&self.client_id.as_bytes())?;
        if self.connect_flags.will {
            v.write_u16::<BigEndian>(self.will_topic.len() as u16)?;
            v.write_all(&self.will_topic.as_bytes())?;

            // FIXME(Shaohua): message length is invalid here.
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

impl FromNetPacket for ConnectPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::Connect);

        let protocol_name_len = BigEndian::read_u16(&buf[*offset..*offset + 2]) as usize;
        *offset += 2;
        let protocol_name =
            String::from_utf8_lossy(&buf[*offset..*offset + protocol_name_len]).to_string();
        *offset += protocol_name_len;

        let protocol_level = ProtocolLevel::try_from(buf[*offset])?;
        *offset += 1;

        let connect_flags = ConnectFlags::from_net(buf, offset)?;

        let keep_alive = BigEndian::read_u16(&buf[*offset..*offset + 2]);
        *offset += 2;

        let client_id_len = BigEndian::read_u16(&buf[*offset..*offset + 2]) as usize;
        *offset += 2;
        let client_id = String::from_utf8_lossy(&buf[*offset..*offset + client_id_len]).to_string();
        *offset += client_id_len;

        // TODO(Shaohua): Read username and password
        let will_topic = if connect_flags.will {
            let will_topic_len = BigEndian::read_u16(&buf[*offset..*offset + 2]) as usize;
            *offset += 2;
            let will_topic =
                String::from_utf8_lossy(&buf[*offset..*offset + will_topic_len]).to_string();
            *offset += will_topic_len;
            will_topic
        } else {
            String::new()
        };
        let will_message = if connect_flags.will {
            // FIXME(Shaohua): Read variant msg length
            let will_message_len = BigEndian::read_u16(&buf[*offset..*offset + 2]) as usize;
            *offset += 2;
            let will_message = buf[*offset..*offset + will_message_len].to_vec();
            *offset += will_message_len;
            will_message
        } else {
            Vec::new()
        };

        let username = if connect_flags.username {
            let username_len = BigEndian::read_u16(&buf[*offset..*offset + 2]) as usize;
            *offset += 2;
            let username =
                String::from_utf8_lossy(&buf[*offset..*offset + username_len]).to_string();
            *offset += username_len;
            username
        } else {
            String::new()
        };

        let password = if connect_flags.password {
            let password_len = BigEndian::read_u16(&buf[*offset..*offset + 2]) as usize;
            *offset += 2;
            let password = buf[*offset..*offset + password_len].to_vec();
            *offset += password_len;
            password
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
