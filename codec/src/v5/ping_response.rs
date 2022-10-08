// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, Packet,
    PacketType, VarIntError,
};

/// The `PingResponse` packet is sent to a Client from the Server to reply to `PingRequest` packet.
///
/// This ping request/response mechanism is used to keep alive.
///
/// Note that this packet does not contain variable header or payload.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PingResponsePacket();

impl PingResponsePacket {
    /// Create a new ping response packet.
    #[must_use]
    pub const fn new() -> Self {
        Self()
    }
}

impl EncodePacket for PingResponsePacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        // Payload is empty
        let fixed_header = FixedHeader::new(PacketType::PingResponse, 0)?;
        fixed_header.encode(v)
    }
}

impl DecodePacket for PingResponsePacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PingResponse {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length() != 0 {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            Ok(Self())
        }
    }
}

impl Packet for PingResponsePacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PingResponse
    }

    fn bytes(&self) -> Result<usize, VarIntError> {
        let fixed_header = FixedHeader::new(PacketType::PingResponse, 0)?;
        Ok(fixed_header.bytes())
    }
}
