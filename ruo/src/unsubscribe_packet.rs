// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::*;
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::default::Default;
use std::io;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct UnsubscribePacket {
    topics: Vec<String>,
    packet_id: PacketId,
}

impl UnsubscribePacket {
    pub fn new(topics: &[&str], packet_id: PacketId) -> Self {
        UnsubscribePacket {
            topics: topics.iter().map(|t| t.to_string()).collect(),
            packet_id,
        }
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn topics(&self) -> &[String] {
        &self.topics
    }
}

impl FromNetPacket for UnsubscribePacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<UnsubscribePacket, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::PublishAck);

        let _remaining_len = buf[*offset] as usize;

        *offset += 1;
        let packet_id = BigEndian::read_u16(&buf[*offset..*offset + 2]) as PacketId;
        *offset += 2;

        // TODO(Shaohua): Parse topics
        Ok(UnsubscribePacket {
            packet_id,
            topics: Vec::new(),
        })
    }
}

impl ToNetPacket for UnsubscribePacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let fixed_header = FixedHeader {
            packet_type: PacketType::Unsubscribe,
            packet_flags: PacketFlags::Unsubscribe,
        };
        // TODO(Shaohua): Add variable header and payload
        fixed_header.to_net(v)
    }
}
