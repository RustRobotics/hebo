// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::*;
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UnsubscribeAckPacket {
    packet_id: PacketId,
}

impl FromNetPacket for UnsubscribeAckPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<UnsubscribeAckPacket, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::UnsubscribeAck);

        let remaining_len = buf[*offset] as usize;
        assert_eq!(remaining_len, 2);
        *offset += 1;
        let packet_id = BigEndian::read_u16(&buf[*offset..*offset + 2]) as PacketId;
        *offset += 2;

        Ok(UnsubscribeAckPacket { packet_id })
    }
}

impl ToNetPacket for UnsubscribeAckPacket {
    fn to_net(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = buf.len();

        let fixed_header = FixedHeader {
            packet_type: PacketType::UnsubscribeAck,
            packet_flags: PacketFlags::UnsubscribeAck,
        };
        fixed_header.to_net(buf)?;

        let remaining_len = 2;
        buf.push(remaining_len);
        buf.write_u16::<BigEndian>(self.packet_id)?;

        Ok(buf.len() - old_len)
    }
}

impl UnsubscribeAckPacket {
    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}
