// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::{FixedHeader, Packet, PacketType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// The PingResponse packet is sent to a Client from the Server to reply to PingRequest packet.
///
/// This ping request/response mechanism is used to keep alive.
///
/// Note that this packet does not contain variable header or payload.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PingResponsePacket();

impl PingResponsePacket {
    pub fn new() -> PingResponsePacket {
        PingResponsePacket()
    }
}

impl EncodePacket for PingResponsePacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        // Payload is empty
        let fixed_header = FixedHeader::new(PacketType::PingResponse, 0)?;
        fixed_header.encode(v)
    }
}

impl Packet for PingResponsePacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PingResponse
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
            Ok(PingResponsePacket())
        }
    }
}