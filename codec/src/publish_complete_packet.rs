// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io;

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

use crate::base::{
    FixedHeader, FromNetPacket, PacketFlags, PacketId, PacketType, RemainingLength, ToNetPacket,
};
use crate::error::Error;

/// Response to a Publish packet with QoS 2. It is the fourth and final packet of
/// the QoS 2 protocol exchange.
///
/// Packet structre is:
/// ```txt
///  7                     0
/// +-----------------------+
/// | Fixed header          |
/// |                       |
/// +-----------------------+
/// | Packet id             |
/// |                       |
/// +-----------------------+
/// ```
///
/// This packet does not contain payload part.
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
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::PublishComplete);
        assert_eq!(fixed_header.remaining_length.0, 2);

        let packet_id = BigEndian::read_u16(&buf[*offset..*offset + 2]) as PacketId;
        *offset += 2;

        Ok(PublishCompletePacket { packet_id })
    }
}

impl ToNetPacket for PublishCompletePacket {
    fn to_net(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = buf.len();

        let fixed_header = FixedHeader {
            packet_type: PacketType::PublishComplete,
            packet_flags: PacketFlags::PublishComplete,
            remaining_length: RemainingLength(2),
        };
        fixed_header.to_net(buf)?;
        buf.write_u16::<BigEndian>(self.packet_id)?;
        Ok(buf.len() - old_len)
    }
}
