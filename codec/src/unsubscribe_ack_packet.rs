// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::io;

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

use crate::base::{
    FixedHeader, FromNetPacket, PacketFlags, PacketId, PacketType, RemainingLength, ToNetPacket,
};
use crate::error::Error;

/// UnsubscribeAck packet is sent by the Server to the Client to confirm receipt of an
/// Unsubscribe packet.
///
/// Basic struct of packet:
/// ```txt
///  7                       0
/// +-------------------------+
/// | Fixed header            |
/// |                         |
/// +-------------------------+
/// | Packet id               |
/// |                         |
/// +-------------------------+
/// ```
///
/// Note that this packet does not contain payload message.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UnsubscribeAckPacket {
    /// `packet_id` field is read from Unsubscribe packet.
    packet_id: PacketId,
}

impl FromNetPacket for UnsubscribeAckPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<UnsubscribeAckPacket, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::UnsubscribeAck);
        assert_eq!(fixed_header.remaining_length.0, 2);

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
            remaining_length: RemainingLength(2),
        };
        fixed_header.to_net(buf)?;
        buf.write_u16::<BigEndian>(self.packet_id)?;
        Ok(buf.len() - old_len)
    }
}

impl UnsubscribeAckPacket {
    pub fn new(packet_id: PacketId) -> UnsubscribeAckPacket {
        UnsubscribeAckPacket { packet_id }
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}
