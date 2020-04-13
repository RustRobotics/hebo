// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::*;
use crate::error::Error;
use std::io;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PingResponsePacket();

impl PingResponsePacket {
    pub fn new() -> PingResponsePacket {
        PingResponsePacket()
    }
}

impl ToNetPacket for PingResponsePacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = v.len();

        let fixed_header = FixedHeader {
            packet_type: PacketType::PingResponse,
            packet_flags: PacketFlags::PingResponse,
        };
        fixed_header.to_net(v)?;
        let remaining_len = 0; // Payload is empty
        v.push(remaining_len);

        Ok(v.len() - old_len)
    }
}

impl FromNetPacket for PingResponsePacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::PingResponse);
        *offset += 1;

        let remaining_len = buf[*offset] as usize;
        *offset += 1;
        assert_eq!(remaining_len, 0);

        Ok(PingResponsePacket())
    }
}
