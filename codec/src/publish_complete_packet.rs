// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

use super::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, PacketId,
    PacketType, RemainingLength,
};

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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PublishCompletePacket {
    packet_id: PacketId,
}

impl PublishCompletePacket {
    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl DecodePacket for PublishCompletePacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type != PacketType::PublishComplete {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length.0 != 2 {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            let packet_id = BigEndian::read_u16(ba.read_bytes(2)?) as PacketId;
            Ok(PublishCompletePacket { packet_id })
        }
    }
}

impl EncodePacket for PublishCompletePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = FixedHeader {
            packet_type: PacketType::PublishComplete,
            remaining_length: RemainingLength(2),
        };
        fixed_header.encode(buf)?;
        buf.write_u16::<BigEndian>(self.packet_id)?;
        Ok(buf.len() - old_len)
    }
}