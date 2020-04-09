// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::*;
use super::error::Error;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::io::{self, Write};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
// TODO(Shaohua): Replace with slice
pub struct PublishPacket {
    fixed_header: FixedHeader,
    packet_id: PacketId,
    topic: String,
    msg: Vec<u8>,
}

impl FromNetPacket for PublishPacket {
    fn from_net(v: &[u8]) -> Result<Self, Error> {
        let mut offset: usize = 0;
        let fixed_header = FixedHeader::from_net(v)?;
        offset += 1;
        let remaining_len = v[offset] as usize;
        offset += 1;
        let topic_len = BigEndian::read_u16(&v[offset..offset + 2]) as usize;
        offset += 2;
        let topic = String::from_utf8((&v[offset..offset + topic_len]).to_vec()).unwrap();
        offset += topic_len;
        let msg_len = remaining_len - topic_len - 2;
        let msg = v[offset..offset + msg_len].to_vec();
        Ok(PublishPacket {
            fixed_header,
            topic,
            msg,
            // TODO(Shaohua): Parse packet id
            packet_id: 0,
        })
    }
}

impl ToNetPacket for PublishPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = v.len();
        self.fixed_header.to_net(v)?;
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
        let fixed_header = FixedHeader {
            packet_type: PacketType::Publish,
            packet_flags: PacketFlags::Publish {
                dup: false,
                qos: qos,
                retain: false,
            },
        };
        PublishPacket {
            fixed_header,
            topic: topic.to_string(),
            msg: msg.to_vec(),
            packet_id: 0,
        }
    }

    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn message(&self) -> &[u8] {
        &self.msg
    }
}
