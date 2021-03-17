// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io;

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

use super::base::{
    FixedHeader, FromNetPacket, PacketFlags, PacketId, PacketType, RemainingLength, ToNetPacket,
};
use super::error::Error;

/// Acknowledge packet for Publish message in QoS 1.
///
/// Basic packet structure:
/// ```txt
///  7                  0
/// +--------------------+
/// | Fixed header       |
/// |                    |
/// +--------------------+
/// | Packet id          |
/// |                    |
/// +--------------------+
/// ```
///
/// This type of packet does not contain payload.
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
        assert_eq!(fixed_header.remaining_length.0, 2);

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
            remaining_length: RemainingLength(2),
        };
        fixed_header.to_net(buf)?;
        buf.write_u16::<BigEndian>(self.packet_id)?;
        Ok(buf.len() - old_len)
    }
}
