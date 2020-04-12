// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::*;
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::convert::TryFrom;
use std::default::Default;
use std::io::{self, Write};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConnectFlags {
    pub username: bool,
    pub password: bool,
    pub retain: bool,
    pub qos: QoS,
    pub will: bool,
    pub clean_session: bool,
}

impl Default for ConnectFlags {
    fn default() -> Self {
        ConnectFlags {
            username: false,
            password: false,
            retain: false,
            qos: QoS::AtMostOnce,
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
            let retian = if self.retain {
                0b0010_0000
            } else {
                0b0000_0000
            };

            let qos = match self.qos {
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

            username | password | retian | qos | will | clean_session
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
        let retain = flags & 0b0010_0000 == 0b0010_0000;
        let qos = QoS::try_from((flags & 0b0001_1000) >> 3)?;
        let will = flags & 0b0000_0100 == 0b0000_0100;
        let clean_session = flags & 0b0000_0010 == 0b0000_0010;
        *offset += 1;
        Ok(ConnectFlags {
            username,
            password,
            retain,
            qos,
            will,
            clean_session,
        })
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ConnectPacket {
    protocol_name: String,
    pub protocol_level: ProtocolLevel,
    pub connect_flags: ConnectFlags,
    pub keepalive: u16,
    client_id: String,
}

impl ConnectPacket {
    pub fn new() -> ConnectPacket {
        ConnectPacket {
            protocol_name: "MQTT".to_string(),
            keepalive: 60,
            ..ConnectPacket::default()
        }
    }

    pub fn set_client_id(&mut self, id: &str) {
        self.client_id.clear();
        self.client_id.push_str(id);
    }

    pub fn set_qos(&mut self, qos: QoS) {
        self.connect_flags.qos = qos;
    }

    fn qos(&self) -> QoS {
        self.connect_flags.qos
    }
}

impl ToNetPacket for ConnectPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = v.len();
        let fixed_header = FixedHeader {
            packet_type: PacketType::Connect,
            packet_flags: PacketFlags::Connect,
        };
        fixed_header.to_net(v)?;

        let remaining_len: u8 = 2 // protocol_name_len
            + self.protocol_name.len() as u8 // b"MQTT" protocol name
            + 1 // protocol_level
            + 1 // connect_flags
            + 2 // keepalive
            + 2 // client_id_len
            + self.client_id.len() as u8;

        v.push(remaining_len);
        v.write_u16::<BigEndian>(self.protocol_name.len() as u16)?;
        v.write(&self.protocol_name.as_bytes())?;
        self.protocol_level.to_net(v)?;
        self.connect_flags.to_net(v)?;
        v.write_u16::<BigEndian>(self.keepalive)?;
        v.write_u16::<BigEndian>(self.client_id.len() as u16)?;
        v.write(&self.client_id.as_bytes())?;
        Ok(v.len() - old_len)
    }
}

impl FromNetPacket for ConnectPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::Connect);
        *offset += 1;
        let remaining_len = buf[*offset] as usize;
        *offset += 1;

        let protocol_name_len = BigEndian::read_u16(&buf[*offset..*offset + 2]) as usize;
        *offset += 2;
        let protocol_name =
            String::from_utf8_lossy(&buf[*offset..*offset + protocol_name_len]).to_string();
        *offset += protocol_name_len;

        let protocol_level = ProtocolLevel::try_from(buf[*offset])?;
        *offset += 1;

        let connect_flags = ConnectFlags::from_net(buf, offset)?;

        let keepalive = BigEndian::read_u16(&buf[*offset..*offset + 2]);
        *offset += 2;

        let client_id_len = BigEndian::read_u16(&buf[*offset..*offset + 2]) as usize;
        *offset += 2;
        let client_id = String::from_utf8_lossy(&buf[*offset..*offset + client_id_len]).to_string();
        *offset += client_id_len;

        // TODO(Shaohua): Read username and password

        Ok(ConnectPacket {
            protocol_name,
            protocol_level,
            keepalive,
            connect_flags,
            client_id,
        })
    }
}
