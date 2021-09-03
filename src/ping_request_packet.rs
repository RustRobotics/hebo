// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use super::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, PacketType,
    RemainingLength,
};

/// The PingRequest packet is sent to the Server from a Client. It is used to:
/// 1. Notify the Server that this Client is still alive.
/// 2. To check if the Server is alive.
/// 3. To check the network connection is ok.
///
/// This packet does not contain variable header or payload.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PingRequestPacket();

impl PingRequestPacket {
    pub fn new() -> PingRequestPacket {
        PingRequestPacket()
    }
}

impl EncodePacket for PingRequestPacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let fixed_header = FixedHeader {
            packet_type: PacketType::PingRequest,
            remaining_length: RemainingLength(0), // Payload is empty
        };
        fixed_header.encode(v)
    }
}

impl DecodePacket for PingRequestPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type != PacketType::PingRequest {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length.0 != 0 {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            Ok(PingRequestPacket())
        }
    }
}
