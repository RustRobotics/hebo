// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

use super::{
    consts, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, Packet,
    PacketId, PacketType,
};

/// Response to a Publish packet with QoS 2. It is the third packet of the QoS 2 protocol
/// exchange.
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
pub struct PublishReleasePacket {
    packet_id: PacketId,
}

impl PublishReleasePacket {
    pub fn new(packet_id: PacketId) -> Self {
        Self { packet_id }
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl DecodePacket for PublishReleasePacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PublishRelease {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length() != consts::PACKET_ID_BYTES {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            let packet_id =
                BigEndian::read_u16(ba.read_bytes(consts::PACKET_ID_BYTES)?) as PacketId;
            Ok(PublishReleasePacket { packet_id })
        }
    }
}

impl EncodePacket for PublishReleasePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = FixedHeader::new(PacketType::PublishRelease, consts::PACKET_ID_BYTES);
        fixed_header.encode(buf)?;
        buf.write_u16::<BigEndian>(self.packet_id)?;
        Ok(buf.len() - old_len)
    }
}

impl Packet for PublishReleasePacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PublishRelease
    }
}
