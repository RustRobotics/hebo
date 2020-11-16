// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::default::Default;
use std::io;

use crate::base::{
    FixedHeader, FromNetPacket, PacketFlags, PacketType, RemainingLength, ToNetPacket,
};
use crate::error::Error;

/// The Disconnect packet is the final packet sent to the Server from a Client.
///
/// When the Server receives this packet, it will close the network connection
/// and will not send any more packets. And the Server will discard any Will message
/// associated with current connection.
///
/// This packet does not contain variable header or payload.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DisconnectPacket {}

impl DisconnectPacket {
    pub fn new() -> DisconnectPacket {
        Self::default()
    }
}

impl ToNetPacket for DisconnectPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let fixed_header = FixedHeader {
            packet_type: PacketType::Disconnect,
            packet_flags: PacketFlags::Disconnect,
            remaining_length: RemainingLength(0), // No payload
        };
        fixed_header.to_net(v)
    }
}

impl FromNetPacket for DisconnectPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<DisconnectPacket, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::Disconnect);
        assert_eq!(fixed_header.remaining_length.0, 0);
        Ok(DisconnectPacket {})
    }
}
