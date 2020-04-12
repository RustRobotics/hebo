// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::*;
use super::error::Error;
use byteorder::{BigEndian, WriteBytesExt};
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
    pub reserved: bool,
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
            reserved: false,
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

            let qoa = match self.qos {
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

            let reserved = if self.reserved {
                0b0000_0001
            } else {
                0b0000_0000
            };

            username | password | retian | qoa | will | clean_session | reserved
        };
        log::info!("connect flags: {:x?}", flags);
        v.push(flags);

        Ok(1)
    }
}

impl FromNetPacket for ConnectFlags {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let flags = buf[*offset];
        *offset += 1;
        Ok(ConnectFlags {
            username: false,
            password: false,
            retain: false,
            qos: QoS::AtMostOnce,
            will: false,
            clean_session: true,
            reserved: false,
        })
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ConnectPacket {
    pub fixed_header: FixedHeader,
    msg_len: u8,
    protocol_name: Vec<u8>,
    pub version: Version,
    pub connect_flags: ConnectFlags,
    pub keepalive: u16,
    client_id: Vec<u8>,
}

impl ConnectPacket {
    pub fn new() -> ConnectPacket {
        ConnectPacket {
            protocol_name: b"MQTT".to_vec(),
            keepalive: 60,
            ..ConnectPacket::default()
        }
    }

    pub fn set_protocol_name(&mut self, name: &[u8]) {
        self.protocol_name.clear();
        self.protocol_name.write(name).unwrap();
    }

    pub fn set_client_id(&mut self, id: &[u8]) -> io::Result<usize> {
        self.client_id.clear();
        self.client_id.write(id)
    }

    pub fn msg_len(&self) -> u8 {
        (2 // protocol_name_len
         + self.protocol_name.len()
         + 1 // version
         + 1 // connect_flags
         + 2 // keepalive
         + 2 // client_id_len
         + self.client_id.len()) as u8
    }
}

impl ToNetPacket for ConnectPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = v.len();
        self.fixed_header.to_net(v)?;
        v.push(self.msg_len());
        v.write_u16::<BigEndian>(self.protocol_name.len() as u16)?;
        v.write(&self.protocol_name)?;
        self.version.to_net(v)?;
        self.connect_flags.to_net(v)?;
        v.write_u16::<BigEndian>(self.keepalive)?;
        v.write_u16::<BigEndian>(self.client_id.len() as u16)?;
        v.write(&self.client_id)?;
        Ok(v.len() - old_len)
    }
}

impl FromNetPacket for ConnectPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        *offset += 1;
        let remaining_len = buf[*offset] as usize;
        assert_eq!(remaining_len, 2);
        *offset += 1;
        let ack_flags = buf[*offset];
        let session_persistent = ack_flags & 0b0000_0001 == 0b0000_0001;
        *offset += 1;
        let connect_flags = ConnectFlags::from_net(buf, offset)?;
        let keepalive = 0;
        let client_id = Vec::new();
        let version = Version::V311;

        Ok(ConnectPacket {
            fixed_header,
            msg_len: 0,
            protocol_name: Vec::new(),
            version,
            keepalive,
            connect_flags,
            client_id,
        })
    }
}
