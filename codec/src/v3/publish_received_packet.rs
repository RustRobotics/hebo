// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::{FixedHeader, Packet, PacketType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId};

/// Response to a Publish packet with QoS 2. It is the second packet of the QoS 2 protocol
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
pub struct PublishReceivedPacket {
    packet_id: PacketId,
}

impl PublishReceivedPacket {
    pub fn new(packet_id: PacketId) -> Self {
        Self { packet_id }
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl EncodePacket for PublishReceivedPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let fixed_header = FixedHeader::new(PacketType::PublishReceived, self.packet_id.bytes())?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;
        Ok(buf.len() - old_len)
    }
}

impl Packet for PublishReceivedPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PublishReceived
    }
}

impl DecodePacket for PublishReceivedPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PublishReceived {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length() != PacketId::const_bytes() {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            let packet_id = PacketId::decode(ba)?;
            Ok(PublishReceivedPacket { packet_id })
        }
    }
}
