// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

use super::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, PacketId,
    PacketType, RemainingLength,
};

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
#[derive(Clone, Debug, Default, PartialEq)]
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

impl EncodePacket for PublishAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = FixedHeader {
            packet_type: PacketType::PublishAck,
            remaining_length: RemainingLength(2),
        };
        fixed_header.encode(buf)?;
        buf.write_u16::<BigEndian>(self.packet_id)?;
        Ok(buf.len() - old_len)
    }
}

impl DecodePacket for PublishAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type != PacketType::PublishAck {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length.0 != 2 {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            let packet_id = BigEndian::read_u16(ba.read_bytes(2)?) as PacketId;
            Ok(PublishAckPacket { packet_id })
        }
    }
}
