// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};

use super::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, Packet, PacketId,
    PacketType,
};

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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnsubscribeAckPacket {
    /// `packet_id` field is read from Unsubscribe packet.
    packet_id: PacketId,
}

impl UnsubscribeAckPacket {
    pub fn new(packet_id: PacketId) -> Self {
        Self { packet_id }
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl DecodePacket for UnsubscribeAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<UnsubscribeAckPacket, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::UnsubscribeAck {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length() != 2 {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            let packet_id = ba.read_u16()? as PacketId;
            Ok(UnsubscribeAckPacket { packet_id })
        }
    }
}

impl EncodePacket for UnsubscribeAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = FixedHeader::new(PacketType::UnsubscribeAck, 2);
        fixed_header.encode(buf)?;
        buf.write_u16::<BigEndian>(self.packet_id)?;
        Ok(buf.len() - old_len)
    }
}

impl Packet for UnsubscribeAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::UnsubscribeAck
    }
}
