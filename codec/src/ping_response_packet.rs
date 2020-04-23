// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::io;

use crate::base::*;
use crate::error::Error;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PingResponsePacket();

impl PingResponsePacket {
    pub fn new() -> PingResponsePacket {
        PingResponsePacket()
    }
}

impl ToNetPacket for PingResponsePacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let fixed_header = FixedHeader {
            packet_type: PacketType::PingResponse,
            packet_flags: PacketFlags::PingResponse,
            remaining_length: RemainingLength(0), // Payload is empty
        };
        fixed_header.to_net(v)
    }
}

impl FromNetPacket for PingResponsePacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::PingResponse);
        Ok(PingResponsePacket())
    }
}
