// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::default::Default;
use std::io;

use super::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, PacketType,
    RemainingLength,
};

/// The Disconnect packet is the final packet sent to the Server from a Client.
///
/// When the Server receives this packet, it will close the network connection
/// and will not send any more packets. And the Server will discard any Will message
/// associated with current connection.
///
/// This packet does not contain variable header or payload.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DisconnectPacket {}

impl DisconnectPacket {
    pub fn new() -> DisconnectPacket {
        Self::default()
    }
}

impl EncodePacket for DisconnectPacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let fixed_header = FixedHeader {
            packet_type: PacketType::Disconnect,
            remaining_length: RemainingLength(0), // No payload
        };
        fixed_header.encode(v)
    }
}

impl DecodePacket for DisconnectPacket {
    fn decode(ba: &mut ByteArray) -> Result<DisconnectPacket, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type != PacketType::Disconnect {
            Err(DecodeError::InvalidPacketType)
        } else if (fixed_header.remaining_length.0 != 0) {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            Ok(DisconnectPacket {})
        }
    }
}
