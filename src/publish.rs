// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::base::*;
use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Result, Write};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PublishMessage {
    pub header_flags: HeaderFlags,
    topic: Vec<u8>,
    msg: Vec<u8>,
}

impl ToNetPacket for PublishMessage {
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize> {
        let old_len = v.len();
        self.header_flags.to_net(v)?;
        v.push(self.msg_len());
        v.write_u16::<BigEndian>(self.topic.len() as u16)?;
        v.write(&self.topic)?;
        v.write(&self.msg)?;

        Ok(v.len() - old_len)
    }
}

impl PublishMessage {
    pub fn new(topic: &[u8]) -> PublishMessage {
        let header_flags = HeaderFlags {
            msg_type: MsgType::Publish,
            reserved: Reserved::Publish {
                dup: false,
                qos: QoSLevel::QoS0,
                retain: false,
            },
        };
        PublishMessage {
            header_flags: header_flags,
            topic: Vec::from(topic),
            msg: vec![],
        }
    }

    pub fn set_topic(&mut self, topic: &[u8]) -> Result<usize> {
        self.topic.clear();
        self.topic.write(topic)
    }

    pub fn set_message(&mut self, msg: &[u8]) -> Result<usize> {
        self.msg.clear();
        self.msg.write(msg)
    }

    pub fn msg_len(&self) -> u8 {
        (
            2 // topic len
         + self.topic.len() // topic
         + self.msg.len()
            // message
        ) as u8
    }
}
