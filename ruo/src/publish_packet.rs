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
    qos: QoS,
    dup: bool,
    retain: bool,
    packet_id: PacketId,
    topic: String,
    msg: Vec<u8>,
}

impl FromNetPacket for PublishPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        if fixed_header.packet_type != PacketType::Publish {
            return Err(Error::InvalidFixedHeader);
        }
        let (dup, qos, retain) =
            if let PacketFlags::Publish { dup, qos, retain } = fixed_header.packet_flags {
                (dup, qos, retain)
            } else {
                return Err(Error::InvalidFixedHeader);
            };

        *offset += 1;
        let remaining_len = buf[*offset] as usize;
        *offset += 1;
        let topic_len = BigEndian::read_u16(&buf[*offset..*offset + 2]) as usize;
        *offset += 2;
        let topic = String::from_utf8((&buf[*offset..*offset + topic_len]).to_vec()).unwrap();
        *offset += topic_len;
        let msg_len = remaining_len - topic_len - 2;
        let msg = buf[*offset..*offset + msg_len].to_vec();
        Ok(PublishPacket {
            qos,
            retain,
            dup,
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
        let fixed_header = FixedHeader {
            packet_type: PacketType::Publish,
            packet_flags: PacketFlags::Publish {
                dup: self.dup,
                retain: self.retain,
                qos: self.qos,
            },
        };
        fixed_header.to_net(v)?;
        let msg_len = 2 // Topic length bytes
            + self.topic.len() // Topic length
            + self.msg.len(); // Message length
        v.push(msg_len as u8);

        // Write variable header
        v.write_u16::<BigEndian>(self.topic.len() as u16)?;
        v.write(&self.topic.as_bytes())?;
        if self.qos() != QoS::AtMostOnce {
            v.write_u16::<BigEndian>(self.packet_id())?;
        }

        // Write payload
        v.write(&self.msg)?;

        Ok(v.len() - old_len)
    }
}

impl PublishPacket {
    pub fn new(topic: &str, qos: QoS, msg: &[u8]) -> PublishPacket {
        PublishPacket {
            qos: qos,
            dup: false,
            retain: false,
            topic: topic.to_string(),
            msg: msg.to_vec(),
            packet_id: 0,
        }
    }

    pub fn set_retain(&mut self, retain: bool) {
        self.retain = retain;
    }

    pub fn retain(&self) -> bool {
        self.retain
    }

    pub fn set_dup(&mut self, dup: bool) {
        self.dup = dup
    }

    pub fn dup(&self) -> bool {
        self.dup
    }

    pub fn qos(&self) -> QoS {
        self.qos
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
