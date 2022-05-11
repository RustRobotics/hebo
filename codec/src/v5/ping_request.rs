// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::{FixedHeader, Packet, PacketType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// The `PingRequest` packet is sent to the Server from a Client.
///
/// It is used to:
/// 1. Notify the Server that this Client is still alive.
/// 2. To check if the Server is alive.
/// 3. To check the network connection is ok.
///
/// This packet does not contain variable header or payload.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PingRequestPacket();

impl PingRequestPacket {
    /// Create a new ping request packet.
    #[must_use]
    pub const fn new() -> Self {
        Self()
    }
}

impl EncodePacket for PingRequestPacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        // Payload is empty
        let fixed_header = FixedHeader::new(PacketType::PingRequest, 0)?;
        fixed_header.encode(v)
    }
}

impl Packet for PingRequestPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::PingRequest
    }
}

impl DecodePacket for PingRequestPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::PingRequest {
            Err(DecodeError::InvalidPacketType)
        } else if fixed_header.remaining_length() != 0 {
            Err(DecodeError::InvalidRemainingLength)
        } else {
            Ok(Self())
        }
    }
}
