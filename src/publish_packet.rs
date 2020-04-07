// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::*;
use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Result, Write};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
// TODO(Shaohua): Replace with slice
pub struct PublishPacket {
    topic: String,
    qos: QoSLevel,
    msg: Vec<u8>,
}

impl ToNetPacket for PublishPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize> {
        let old_len = v.len();

        let fixed_header = FixedHeader {
            packet_type: PacketType::Publish,
            packet_flags: PacketFlags::Publish {
                dup: false,
                qos: self.qos,
                retain: false,
            },
        };
        fixed_header.to_net(v)?;
        let msg_len = 2 // Topic length bytes
            + self.topic.len() // Topic length
            + self.msg.len(); // Message length
        v.push(msg_len as u8);
        v.write_u16::<BigEndian>(self.topic.len() as u16)?;
        v.write(&self.topic.as_bytes())?;
        v.write(&self.msg)?;

        Ok(v.len() - old_len)
    }
}

impl PublishPacket {
    pub fn new(topic: &str, qos: QoSLevel, msg: &[u8]) -> PublishPacket {
        PublishPacket {
            topic: topic.to_string(),
            qos: qos,
            msg: msg.to_vec(),
        }
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn message(&self) -> &[u8] {
        &self.msg
    }
}
