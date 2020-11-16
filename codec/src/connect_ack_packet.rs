// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io;

use crate::base::{
    FixedHeader, FromNetPacket, PacketFlags, PacketType, RemainingLength, ToNetPacket,
};
use crate::error::Error;

/// If the Server sends a ConnectAck packet with non-zero return code, it MUST
/// close the network connection.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConnectReturnCode {
    /// Connection accepted.
    Accepted = 0,

    /// The server do not support the level of the MQTT protocol requested by the Client.
    UnacceptedProtocol = 1,

    /// The Client identifier is correct UTF-8 but not allowed by the Server.
    IdentifierRejected = 2,

    /// The Network Connection has been made but the MQTT service is unavailable.
    ServerUnavailable = 3,

    /// The data in the username or password is malformed.
    MalformedUsernamePassword = 4,

    /// The Client is not authorized to connect.
    Unauthorithed = 5,

    /// 6-255 are reserved.
    Reserved = 6,
}

impl Default for ConnectReturnCode {
    fn default() -> ConnectReturnCode {
        ConnectReturnCode::Accepted
    }
}

impl From<u8> for ConnectReturnCode {
    fn from(v: u8) -> ConnectReturnCode {
        match v {
            0 => ConnectReturnCode::Accepted,
            1 => ConnectReturnCode::UnacceptedProtocol,
            2 => ConnectReturnCode::IdentifierRejected,
            3 => ConnectReturnCode::ServerUnavailable,
            4 => ConnectReturnCode::MalformedUsernamePassword,
            5 => ConnectReturnCode::Unauthorithed,
            _ => ConnectReturnCode::Reserved,
        }
    }
}

/// The first packet sent to the Client from the Server must be ConnectAckPacket.
/// If the Client does not receive ConnectAckPacket in a reasonable time, it MUST
/// close the network connection.
///
/// Basic packet structure:
/// ```txt
///  7                       0
/// +-------------------------+
/// | Fixed header            |
/// |                         |
/// +-------------------------+
/// | Ack flags               |
/// +-------------------------+
/// | Return code             |
/// +-------------------------+
/// ```
///
/// This packet does not contain payload.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ConnectAckPacket {
    /// Acknowledge flags is the first byte in variable header.
    /// Session Present flag is set in bit 0 of Ack flags, bits 7-1 are reserved.
    ///
    /// If CleanSession flag in ConnectPacket is true, then this flag must be false
    /// and return code is set to zero.
    ///
    /// If CleanSession flag in ConnectPacket is false, and the Server have stored
    /// SessionState with the same ClientId, then this field is set to true, indicating
    /// that there is already a session state value present on the Server side.
    ///
    /// If return code is not zero, then this flag MUST be false.
    session_present: bool,

    /// Byte 2 in the connection return code.
    return_code: ConnectReturnCode,
}

impl ConnectAckPacket {
    pub fn new(session_present: bool, return_code: ConnectReturnCode) -> ConnectAckPacket {
        ConnectAckPacket {
            session_present,
            return_code,
        }
    }

    pub fn return_code(&self) -> ConnectReturnCode {
        self.return_code
    }

    pub fn session_present(&self) -> bool {
        self.session_present
    }
}

impl FromNetPacket for ConnectAckPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::ConnectAck);

        let ack_flags = buf[*offset];
        let session_present = ack_flags & 0b0000_0001 == 0b0000_0001;
        *offset += 1;
        let return_code = ConnectReturnCode::from(buf[*offset]);
        *offset += 1;

        Ok(ConnectAckPacket {
            session_present,
            return_code,
        })
    }
}

impl ToNetPacket for ConnectAckPacket {
    fn to_net(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = buf.len();
        let fixed_header = FixedHeader {
            packet_type: PacketType::ConnectAck,
            packet_flags: PacketFlags::ConnectAck,
            remaining_length: RemainingLength(2),
        };
        fixed_header.to_net(buf)?;

        let ack_flags = if self.session_present { 0b0000_0001 } else { 0 };
        buf.push(ack_flags);
        buf.push(self.return_code as u8);

        Ok(buf.len() - old_len)
    }
}

#[cfg(test)]
mod tests {
    use crate::connect_ack_packet::ConnectAckPacket;
    use crate::base::FromNetPacket;

    #[test]
    fn test_from_net() {
        let buf: Vec<u8> = vec![0x20, 0x02, 0x00, 0x00];
        let mut offset = 0;
        let packet = ConnectAckPacket::from_net(&buf, &mut offset);
        assert!(packet.is_ok());
        let packet = packet.unwrap();
        assert_eq!(packet.session_present, false);
    }
}