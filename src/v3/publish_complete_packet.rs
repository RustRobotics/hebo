// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};

use super::{FixedHeader, Packet, PacketType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublishCompletePacket {
    packet_id: PacketId,
}

impl PublishCompletePacket {
    pub fn new(packet_id: PacketId) -> Self {
        Self { packet_id }
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl DecodePacket for PublishCompletePacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PublishComplete {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length() != 2 {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            let packet_id = ba.read_u16()? as PacketId;
            Ok(PublishCompletePacket { packet_id })
        }
    }
}

impl EncodePacket for PublishCompletePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = FixedHeader::new(PacketType::PublishComplete, 2);
        fixed_header.encode(buf)?;
        buf.write_u16::<BigEndian>(self.packet_id)?;
        Ok(buf.len() - old_len)
    }
}

impl Packet for PublishCompletePacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PublishComplete
    }
}
