// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io;

use super::base::*;
use super::error::Error;

/// The PingRequest packet is sent to the Server from a Client. It is used to:
/// 1. Notify the Server that this Client is still alive.
/// 2. To check if the Server is alive.
/// 3. To check the network connection is ok.
///
/// This packet does not contain variable header or payload.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PingRequestPacket();

impl PingRequestPacket {
    pub fn new() -> PingRequestPacket {
        PingRequestPacket()
    }
}

impl EncodePacket for PingRequestPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let fixed_header = FixedHeader {
            packet_type: PacketType::PingRequest,
            packet_flags: PacketFlags::PingRequest,
            remaining_length: RemainingLength(0), // Payload is empty
        };
        fixed_header.to_net(v)
    }
}

impl DecodePacket for PingRequestPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::PingRequest);
        assert_eq!(fixed_header.remaining_length.0, 0);
        Ok(PingRequestPacket())
    }
}
