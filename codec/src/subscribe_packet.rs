// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;
use std::io::{self, Write};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

use crate::base::*;
use crate::error::Error;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SubscribePacket {
    topic: String,
    qos: QoS,
    packet_id: PacketId,
}

impl FromNetPacket for SubscribePacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<SubscribePacket, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::Subscribe);

        let packet_id = BigEndian::read_u16(&buf[*offset..*offset + 2]);
        *offset += 2;

        let topic_len = BigEndian::read_u16(&buf[*offset..*offset + 2]) as usize;
        *offset += 2;
        let topic = String::from_utf8_lossy(&buf[*offset..*offset + topic_len]).to_string();

        let qos_flag = buf[*offset];
        *offset += 1;
        let qos = QoS::try_from(qos_flag & 0b0000_0011)?;

        Ok(SubscribePacket {
            packet_id,
            topic,
            qos,
        })
    }
}

impl ToNetPacket for SubscribePacket {
    fn to_net(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = buf.len();

        let remaining_length = 2 // Variable length
            + 2 // Payload length
            + self.topic.len() // Topic length
            + 1; // Requested QoS
        let fixed_header = FixedHeader {
            packet_type: PacketType::Subscribe,
            packet_flags: PacketFlags::Subscribe,
            remaining_length: RemainingLength(remaining_length as u32),
        };
        fixed_header.to_net(buf)?;

        // Variable header
        buf.write_u16::<BigEndian>(self.packet_id).unwrap();

        // Payload
        buf.write_u16::<BigEndian>(self.topic.len() as u16)?;
        buf.write_all(&self.topic.as_bytes())?;
        let qos: u8 = 0b0000_0011 & (self.qos as u8);
        buf.push(qos);

        Ok(buf.len() - old_len)
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

    pub fn qos(&self) -> QoS {
        self.qos
    }
}
