// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::*;
use super::error::Error;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::io::Write;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PublishCompletePacket {
    packet_id: PacketId,
}

impl PublishCompletePacket {
    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl FromNetPacket for PublishCompletePacket {
    fn from_net(buf: &[u8]) -> Result<Self, Error> {
        if buf.len() == 0 {
            return Err(Error::PacketEmpty);
        }
        let mut offset = 0;
        let fixed_header = FixedHeader::from_net(buf)?;
        offset += 1;
        let remaining_len = buf[offset] as usize;
        assert_eq!(remaining_len, 2);
        offset += 1;
        let packet_id = BigEndian::read_u16(&buf[offset..offset + 2]) as PacketId;
        offset += 2;

        Ok(PublishCompletePacket { packet_id })
    }
}
