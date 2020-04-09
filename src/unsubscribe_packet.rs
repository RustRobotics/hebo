// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::base::*;
use byteorder::{BigEndian, WriteBytesExt};
use std::default::Default;
use std::io::{Result, Write};

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
}

impl ToNetPacket for UnsubscribePacket {
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize> {
        let fixed_header = FixedHeader {
            packet_type: PacketType::Unsubscribe,
            packet_flags: PacketFlags::Unsubscribe,
        };
        // TODO(Shaohua): Add variable header and payload
        fixed_header.to_net(v)
    }
}
