// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::*;
use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Result, Write};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SubscribePacket {
    topic: String,
    qos: QoS,
    packet_id: PacketId,
}

impl ToNetPacket for SubscribePacket {
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize> {
        let old_len = v.len();

        let fixed_header = FixedHeader {
            packet_type: PacketType::Subscribe,
            packet_flags: PacketFlags::Subscribe,
        };
        fixed_header.to_net(v)?;

        let msg_len = 2 // Variable length
            + 2 // Payload length
            + self.topic.len() // Topic length
            + 1; // Requested QoS
        v.push(msg_len as u8);

        // Variable header
        v.write_u16::<BigEndian>(self.packet_id).unwrap();

        // Payload
        v.write_u16::<BigEndian>(self.topic.len() as u16)?;
        v.write(&self.topic.as_bytes())?;
        let qos: u8 = 0b0000_0011 & (self.qos as u8);
        v.push(qos);

        Ok(v.len() - old_len)
    }
}

impl SubscribePacket {
    pub fn new(topic: &str, qos: QoS, packet_id: PacketId) -> SubscribePacket {
        SubscribePacket {
            topic: topic.to_string(),
            qos,
            packet_id,
        }
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}
