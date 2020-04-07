// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::base::*;
use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Result, Write};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SubscribePacket {
    pub fixed_header: FixedHeader,
    topic: Vec<u8>,
}

impl ToNetPacket for SubscribePacket {
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize> {
        let old_len = v.len();
        self.fixed_header.to_net(v)?;
        v.push(self.msg_len());
        v.write_u16::<BigEndian>(self.topic.len() as u16)?;
        v.write(&self.topic)?;

        Ok(v.len() - old_len)
    }
}

impl SubscribePacket {
    pub fn new(topic: &[u8], qos: QoSLevel) -> PublishPacket {
        let fixed_header = FixedHeader {
            packet_type: PacketType::Publish,
            packet_flags: PacketFlags::Publish {
                dup: false,
                qos: qos,
                retain: false,
            },
        };
        PublishPacket {
            fixed_header: fixed_header,
            topic: Vec::from(topic),
        }
    }

    pub fn msg_len(&self) -> u8 {
        (
            2 // topic len
         + self.topic.len()
            // topic
        ) as u8
    }
}
