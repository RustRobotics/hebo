// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::*;
use crate::error::Error;
use std::io;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConnectReturnCode {
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

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ConnectAckPacket {
    return_code: ConnectReturnCode,
    session_persistent: bool,
}

impl ConnectAckPacket {
    pub fn new(return_code: ConnectReturnCode, session_persistent: bool) -> ConnectAckPacket {
        ConnectAckPacket {
            return_code,
            session_persistent,
        }
    }

    pub fn return_code(&self) -> ConnectReturnCode {
        self.return_code
    }

    pub fn session_persistent(&self) -> bool {
        self.session_persistent
    }
}

impl FromNetPacket for ConnectAckPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::ConnectAck);

        let remaining_len = buf[*offset] as usize;
        assert_eq!(remaining_len, 2);
        *offset += 1;
        let ack_flags = buf[*offset];
        let session_persistent = ack_flags & 0b0000_0001 == 0b0000_0001;
        *offset += 1;
        let return_code = ConnectReturnCode::from(buf[*offset]);
        *offset += 1;

        Ok(ConnectAckPacket {
            return_code,
            session_persistent,
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

        let ack_flags = if self.session_persistent {
            0b0000_0001
        } else {
            0
        };
        buf.push(ack_flags);
        buf.push(self.return_code as u8);

        Ok(buf.len() - old_len)
    }
}
