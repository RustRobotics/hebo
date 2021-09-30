// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::{FixedHeader, Packet, PacketType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// If the Server sends a ConnectAck packet with non-zero return code, it MUST
/// close the network connection.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectReasonCode {
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
    Unauthorized = 5,

    /// 6-255 are reserved.
    Reserved = 6,
}

impl Default for ConnectReasonCode {
    fn default() -> ConnectReasonCode {
        ConnectReasonCode::Accepted
    }
}

impl From<u8> for ConnectReasonCode {
    fn from(v: u8) -> ConnectReasonCode {
        match v {
            0 => ConnectReasonCode::Accepted,
            1 => ConnectReasonCode::UnacceptedProtocol,
            2 => ConnectReasonCode::IdentifierRejected,
            3 => ConnectReasonCode::ServerUnavailable,
            4 => ConnectReasonCode::MalformedUsernamePassword,
            5 => ConnectReasonCode::Unauthorized,
            _ => ConnectReasonCode::Reserved,
        }
    }
}

/// The CONNACK packet is the packet sent by the Server in response to a CONNECT packet
/// received from a Client.
///
/// The Server MUST send a CONNACK with a 0x00 (Success) Reason Code before
/// sending any Packet other than AUTH [MQTT-3.2.0-1].
///
/// The Server MUST NOT send more than one CONNACK in a Network Connection [MQTT-3.2.0-2].
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
/// | Reason code             |
/// +-------------------------+
/// | Properties              |
/// |                         |
/// +-------------------------+
/// ```
/// he Variable Header of the CONNACK Packet contains the following fields in the order:
/// - Connect Acknowledge Flags
/// - Connect Reason Code
/// - Properties.
///
/// If the Client does not receive a CONNACK packet from the Server within a reasonable
/// amount of time, the Client SHOULD close the Network Connection. A "reasonable"
/// amount of time depends on the type of application and the communications infrastructure.
///
/// This packet does not contain payload.
#[derive(Clone, Debug, Default, PartialEq)]
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
    reason_code: ConnectReasonCode,
}

impl ConnectAckPacket {
    pub fn new(mut session_present: bool, reason_code: ConnectReasonCode) -> ConnectAckPacket {
        // If a Server sends a CONNACK packet containing a non-zero Reason Code
        // it MUST set Session Present to 0 [MQTT-3.2.2-6].
        if reason_code != ConnectReasonCode::Accepted {
            session_present = false;
        }
        ConnectAckPacket {
            session_present,
            reason_code,
        }
    }

    pub fn set_reason_code(&mut self, code: ConnectReasonCode) -> &mut Self {
        if code != ConnectReasonCode::Accepted {
            self.session_present = false;
        }
        self.reason_code = code;
        self
    }

    pub fn reason_code(&self) -> ConnectReasonCode {
        self.reason_code
    }

    pub fn set_session_present(&mut self, present: bool) -> &mut Self {
        self.session_present = present;
        self
    }

    pub fn session_present(&self) -> bool {
        self.session_present
    }
}

impl DecodePacket for ConnectAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        assert_eq!(fixed_header.packet_type(), PacketType::ConnectAck);

        let ack_flags = ba.read_byte()?;
        let session_present = ack_flags & 0b0000_0001 == 0b0000_0001;
        let reason_code = ConnectReasonCode::from(ba.read_byte()?);

        Ok(ConnectAckPacket {
            session_present,
            reason_code,
        })
    }
}

impl EncodePacket for ConnectAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();
        let fixed_header = FixedHeader::new(PacketType::ConnectAck, 2)?;
        fixed_header.encode(buf)?;

        let ack_flags = if self.session_present { 0b0000_0001 } else { 0 };
        buf.push(ack_flags);
        buf.push(self.reason_code as u8);

        Ok(buf.len() - old_len)
    }
}

impl Packet for ConnectAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::ConnectAck
    }
}

#[cfg(test)]
mod tests {
    use super::{ByteArray, ConnectAckPacket, DecodePacket};

    #[test]
    fn test_decode() {
        let buf: Vec<u8> = vec![0x20, 0x02, 0x00, 0x00];
        let mut ba = ByteArray::new(&buf);
        let packet = ConnectAckPacket::decode(&mut ba);
        assert!(packet.is_ok());
        let packet = packet.unwrap();
        assert_eq!(packet.session_present, false);
    }
}
