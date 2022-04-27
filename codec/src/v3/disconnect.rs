// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::default::Default;

use super::{FixedHeader, Packet, PacketType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

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
        // No payload
        let fixed_header = FixedHeader::new(PacketType::Disconnect, 0)?;
        fixed_header.encode(v)
    }
}

impl Packet for DisconnectPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Disconnect
    }
}

impl DecodePacket for DisconnectPacket {
    fn decode(ba: &mut ByteArray) -> Result<DisconnectPacket, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::Disconnect {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length() != 0 {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            Ok(DisconnectPacket {})
        }
    }
}
