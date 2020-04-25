// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;
use std::io;

use crate::error::Error;

/// Packet identifier
pub type PacketId = u16;

/// Convert native data types to network byte stream.
pub trait ToNetPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize>;
}

pub trait FromNetPacket: Sized {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error>;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PacketType {
    Unknown = 0,

    /// Request to connect to broker
    Connect = 1,

    /// Broker reply to connect request
    ConnectAck = 2,

    /// Publish message
    Publish = 3,

    /// Publish acknowledgement
    PublishAck = 4,

    /// Publish received
    PublishReceived = 5,

    /// Publish release
    PublishRelease = 6,

    /// Publish complete
    PublishComplete = 7,

    /// Client subscribe request
    Subscribe = 8,

    /// Subscribe acknowledgement
    SubscribeAck = 9,

    /// Unsubscribe request
    Unsubscribe = 10,

    /// Unsubscribe acknowledgement
    UnsubscribeAck = 11,

    /// Client ping request
    PingRequest = 12,

    /// Server ping response
    PingResponse = 13,

    /// Client is disconnecting
    Disconnect = 14,

    Reserved = 15,
}

impl Into<u8> for PacketType {
    fn into(self) -> u8 {
        (self as u8 & 0b0000_1111) << 4
    }
}

impl From<u8> for PacketType {
    fn from(flag: u8) -> Self {
        let packet_type = (flag & 0b1111_0000) >> 4;
        match packet_type {
            0 => PacketType::Unknown,
            1 => PacketType::Connect,
            2 => PacketType::ConnectAck,
            3 => PacketType::Publish,
            4 => PacketType::PublishAck,
            5 => PacketType::PublishReceived,
            6 => PacketType::PublishRelease,
            7 => PacketType::PublishComplete,
            8 => PacketType::Subscribe,
            9 => PacketType::SubscribeAck,
            10 => PacketType::Unsubscribe,
            11 => PacketType::UnsubscribeAck,
            12 => PacketType::PingRequest,
            13 => PacketType::PingResponse,
            14 => PacketType::Disconnect,
            15 => PacketType::Reserved,

            _ => PacketType::Unknown,
        }
    }
}

impl Default for PacketType {
    fn default() -> Self {
        PacketType::Connect
    }
}

/// Packet flags
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PacketFlags {
    /// Request to connect to broker
    Connect,

    /// Broker reply to connect request
    ConnectAck,

    /// Publish message
    Publish { dup: bool, qos: QoS, retain: bool },

    /// Publish acknowledgement
    PublishAck,

    /// Publish received
    PublishReceived,

    /// Publish release
    PublishRelease,

    /// Publish complete
    PublishComplete,

    /// Client subscribe request
    Subscribe,

    /// Subscribe acknowledgement
    SubscribeAck,

    /// Unsubscribe request
    Unsubscribe,

    /// Unsubscribe acknowledgement
    UnsubscribeAck,

    /// Client ping request
    PingRequest,

    /// Server ping response
    PingResponse,

    /// Client is disconnecting
    Disconnect,
}

impl Into<u8> for PacketFlags {
    fn into(self) -> u8 {
        match self {
            PacketFlags::Connect => 0,
            PacketFlags::ConnectAck => 0,
            PacketFlags::Publish { dup, qos, retain } => {
                let dup = if dup { 0b0000_10000 } else { 0b0000_0000 };
                let qos = match qos {
                    QoS::AtMostOnce => 0b0000_0000,
                    QoS::AtLeastOnce => 0b0000_0010,
                    QoS::ExactOnce => 0b0000_0100,
                };

                let retain = if retain { 0b0000_0001 } else { 0b0000_0000 };
                dup | qos | retain
            }
            PacketFlags::PublishAck => 0,
            PacketFlags::PublishReceived => 0,
            PacketFlags::PublishRelease => 0b0000_0010,
            PacketFlags::PublishComplete => 0,
            PacketFlags::Subscribe => 0b0000_0010,
            PacketFlags::SubscribeAck => 0,
            PacketFlags::Unsubscribe => 0b0000_0010,
            PacketFlags::UnsubscribeAck => 0,
            PacketFlags::PingRequest => 0,
            PacketFlags::PingResponse => 0,
            PacketFlags::Disconnect => 0,
        }
    }
}

impl Default for PacketFlags {
    fn default() -> Self {
        PacketFlags::Connect
    }
}

impl PacketFlags {
    pub fn from_u8(packet_type: PacketType, flag: u8) -> PacketFlags {
        let flag = flag & 0b0000_1111;
        match packet_type {
            PacketType::Publish => {
                let dup = (flag & 0b0000_1000) == 0b0000_1000;
                let retain = (flag & 0b0000_0001) == 0b0000_0001;
                let qos = match flag & 0b0000_0110 {
                    0b0000_0000 => QoS::AtMostOnce,
                    0b0000_0010 => QoS::AtLeastOnce,
                    0b0000_0100 => QoS::ExactOnce,
                    // TODO(Shaohua): Handle qos error
                    _ => QoS::AtMostOnce,
                };

                PacketFlags::Publish { dup, qos, retain }
            }
            _ => PacketFlags::default(),
        }
    }
}

/// `Remaining Length` uses variable length encoding method. The 7th bit
/// in a byte is used to indicate are bytes are available. And the maximum number
/// of bytes in the `Remaining Length` field is 4 bytes. The maximum value is
/// `0xFF 0xFF 0xFF 0x7F`, `256MB`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct RemainingLength(pub u32);

impl RemainingLength {
    pub fn len(&self) -> usize {
        if self.0 > 0x7fffff {
            4
        } else if self.0 > 0x7fff {
            3
        } else if self.0 > 0x7f {
            2
        } else {
            1
        }
    }
}

impl FromNetPacket for RemainingLength {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let mut byte: u32;
        let mut value: u32 = 0;
        let mut multiplier = 1;
        loop {
            byte = buf[*offset] as u32;
            *offset += 1;
            value += (byte & 127) * multiplier;
            multiplier *= 128;

            if multiplier > 128 * 128 * 128 * 128 {
                return Err(Error::InvalidRemainingLength);
            }

            if (byte & 128) == 0 {
                break;
            }
        }

        // if buf.len() - *offset < val as usize {
        //     Err(Error::InvalidRemainingLength)
        // } else {
        //     Ok(RemainingLength(val))
        // }
        Ok(RemainingLength(value))
    }
}

impl ToNetPacket for RemainingLength {
    fn to_net(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        if self.0 > 0x7fffffff {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        let mut n = self.0;
        let mut count = 0;
        while n > 0 {
            let mut m = n % 128;
            count += 1;
            n /= 128;
            if n > 0 {
                m = m | 128;
            }
            buf.push(m as u8);
        }
        Ok(count)
    }
}

/// Fixed header part of a mqtt control packet. It consists of as least two bytes.
///  7 6 5 4 3 2 1 0
/// +-------+-------+
/// | Type  | Flags |
/// +-------+-------+
/// | Remaining Len |
/// +-------+-------+
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FixedHeader {
    pub packet_type: PacketType,
    pub packet_flags: PacketFlags,
    pub remaining_length: RemainingLength,
}

impl FromNetPacket for FixedHeader {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let flag = buf[*offset];
        *offset += 1;

        // TODO(Shaohua): Handle invalid packet type.
        let packet_type = PacketType::from(flag);
        let packet_flags = PacketFlags::from_u8(packet_type, flag);

        let remaining_length = RemainingLength::from_net(buf, offset)?;

        Ok(FixedHeader {
            packet_type,
            packet_flags,
            remaining_length,
        })
    }
}

impl ToNetPacket for FixedHeader {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let packet_type: u8 = self.packet_type.into();
        let packet_flags: u8 = self.packet_flags.into();
        v.push(packet_type | packet_flags);
        self.remaining_length.to_net(v)?;

        // TODO(Shaohua): Calc length.
        Ok(1 + self.remaining_length.len())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QoS {
    /// At most once delivery.
    AtMostOnce = 0,

    /// At least once delivery.
    AtLeastOnce = 1,

    /// Exactly once delivery.
    ExactOnce = 2,
}

impl Default for QoS {
    fn default() -> Self {
        QoS::AtMostOnce
    }
}

impl TryFrom<u8> for QoS {
    type Error = Error;

    fn try_from(v: u8) -> Result<QoS, Self::Error> {
        match v {
            0 => Ok(QoS::AtMostOnce),
            1 => Ok(QoS::AtLeastOnce),
            2 => Ok(QoS::ExactOnce),
            _ => Err(Error::InvalidQoS),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RemainingLength;
    use crate::base::{FromNetPacket, ToNetPacket};

    #[test]
    fn test_remaining_length_encode() {
        let mut buf = Vec::with_capacity(4);

        let remaining_len = RemainingLength(126);
        let _ = remaining_len.to_net(&mut buf);
        assert_eq!(&buf, &[0x7e]);
        buf.clear();

        let remaining_len = RemainingLength(146);
        let _ = remaining_len.to_net(&mut buf);
        assert_eq!(&buf, &[0x92, 0x01]);
        buf.clear();

        let remaining_len = RemainingLength(16_385);
        let _ret = remaining_len.to_net(&mut buf);
        assert_eq!(&buf, &[0x81, 0x80, 0x01]);
        buf.clear();

        let remaining_len = RemainingLength(2_097_152);
        let _ret = remaining_len.to_net(&mut buf);
        assert_eq!(&buf, &[0x80, 0x80, 0x80, 0x01]);
        buf.clear();
    }

    #[test]
    fn test_remaining_length_decode() {
        let buf = [0x7e];
        let mut offset = 0;
        let ret = RemainingLength::from_net(&buf, &mut offset);
        assert_eq!(ret.unwrap().0, 126);

        let buf = [0x92, 0x01];
        let mut offset = 0;
        let ret = RemainingLength::from_net(&buf, &mut offset);
        assert_eq!(ret.unwrap().0, 146);

        let buf = [0x81, 0x80, 0x01];
        let mut offset = 0;
        let ret = RemainingLength::from_net(&buf, &mut offset);
        assert_eq!(ret.unwrap().0, 16_385);

        let buf = [0x81, 0x80, 0x80, 0x01];
        let mut offset = 0;
        let ret = RemainingLength::from_net(&buf, &mut offset);
        assert_eq!(ret.unwrap().0, 2_097_153);

        let buf = [0xff, 0xff, 0xff, 0x7f];
        let mut offset = 0;
        let ret = RemainingLength::from_net(&buf, &mut offset);
        assert_eq!(ret.unwrap().0, 268_435_455);
    }
}

/// Check string characters and length.
pub fn validate_utf8_string(s: &str) -> Result<(), Error> {
    if s.len() > u16::MAX as usize {
        return Err(Error::TooManyData);
    }
    for c in s.chars() {
        // Ignore control characters
        // No need to check chars between 0xd800 and 0xfffd as they are invalid coded point and not allowed.
        if (c >= '\u{0000}' && c <= '\u{001f}') || (c >= '\u{007f}' && c <= '\u{009f}') {
            return Err(Error::InvalidString);
        }
    }
    // Empty string is valid.
    Ok(())
}

/// Convert range of bytes to valid UTF-8 string.
pub fn to_utf8_string(buf: &[u8], start: usize, end: usize) -> Result<String, Error> {
    let s = String::from_utf8((&buf[start..end]).to_vec())?;
    validate_utf8_string(&s)?;
    Ok(s)
}

/// Check data length exceeds 64k or not.
pub fn validate_two_bytes_data(data: &[u8]) -> Result<(), Error> {
    if data.len() > u16::MAX as usize {
        Err(Error::TooManyData)
    } else {
        Ok(())
    }
}
