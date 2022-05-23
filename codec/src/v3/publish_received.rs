// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::{FixedHeader, Packet, PacketType};
use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId, VarIntError,
};

/// Response to a Publish packet with `QoS` 2.
///
/// It is the second packet of the `QoS` 2 protocol exchange.
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
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublishReceivedPacket {
    packet_id: PacketId,
}

impl PublishReceivedPacket {
    /// Create a new publish received packet with `packet_id`.
    #[must_use]
    pub const fn new(packet_id: PacketId) -> Self {
        Self { packet_id }
    }

    /// Get current packet id.
    #[must_use]
    pub const fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl EncodePacket for PublishReceivedPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = FixedHeader::new(PacketType::PublishReceived, PacketId::bytes())?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        Ok(buf.len() - old_len)
    }
}

impl DecodePacket for PublishReceivedPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PublishReceived {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length() != PacketId::bytes() {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            let packet_id = PacketId::decode(ba)?;
            Ok(Self { packet_id })
        }
    }
}

impl Packet for PublishReceivedPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PublishReceived
    }

    fn bytes(&self) -> Result<usize, VarIntError> {
        let fixed_header = FixedHeader::new(PacketType::PublishReceived, PacketId::bytes())?;
        Ok(fixed_header.bytes() + PacketId::bytes())
    }
}
