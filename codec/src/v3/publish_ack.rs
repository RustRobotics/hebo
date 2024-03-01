// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, Packet, PacketId,
    PacketType, VarIntError,
};

/// Acknowledge packet for Publish message in `QoS` 1.
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
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublishAckPacket {
    packet_id: PacketId,
}

impl PublishAckPacket {
    #[must_use]
    pub const fn new(packet_id: PacketId) -> Self {
        Self { packet_id }
    }

    #[must_use]
    #[inline]
    pub const fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl EncodePacket for PublishAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = FixedHeader::new(PacketType::PublishAck, PacketId::bytes())?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        Ok(buf.len() - old_len)
    }
}

impl DecodePacket for PublishAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PublishAck {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length() != PacketId::bytes() {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            let packet_id = PacketId::decode(ba)?;
            Ok(Self { packet_id })
        }
    }
}

impl Packet for PublishAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PublishAck
    }

    fn bytes(&self) -> Result<usize, VarIntError> {
        let fixed_header = FixedHeader::new(PacketType::PublishAck, PacketId::bytes())?;
        Ok(fixed_header.bytes() + PacketId::bytes())
    }
}
