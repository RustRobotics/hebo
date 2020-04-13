// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::*;
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::io;

/// Acknowledge packet for Publish message in QoS1.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PublishAckPacket {
    packet_id: PacketId,
}

impl PublishAckPacket {
    pub fn new(packet_id: PacketId) -> PublishAckPacket {
        PublishAckPacket { packet_id }
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl FromNetPacket for PublishAckPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::PublishAck);

        let remaining_len = buf[*offset] as usize;
        assert_eq!(remaining_len, 2);
        *offset += 1;
        let packet_id = BigEndian::read_u16(&buf[*offset..*offset + 2]) as PacketId;
        *offset += 2;

        Ok(PublishAckPacket { packet_id })
    }
}

impl ToNetPacket for PublishAckPacket {
    fn to_net(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = buf.len();

        let fixed_header = FixedHeader {
            packet_type: PacketType::PublishAck,
            packet_flags: PacketFlags::PublishAck,
        };
        fixed_header.to_net(buf)?;

        let remaining_len = 2;
        buf.push(remaining_len);
        buf.write_u16::<BigEndian>(self.packet_id)?;

        Ok(buf.len() - old_len)
    }
}
