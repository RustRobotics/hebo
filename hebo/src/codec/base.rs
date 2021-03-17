// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::convert::TryFrom;
use std::io;

use super::error::{DecodeError, EncodeError};

/// Packet identifier
pub type PacketId = u16;

/// Convert native data types to network byte stream.
pub trait EncodePacket {
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError>;
}

pub trait DecodePacket: Sized {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, DecodeError>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PacketType {
    /// Request to connect to broker
    Connect,

    /// Broker reply to connect request
    ConnectAck,

    /// Publish message
    Publish {
        dup: bool,
        qos: QoS,
        retain: bool,
    },

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

    Reserved,
}

impl Into<u8> for PacketType {
    fn into(self) -> u8 {
        0b01
        //(self as u8 & 0b0000_1111) << 4
    }
}

impl TryFrom<u8> for PacketType {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<PacketType, Self::Error> {
        let type_bits = (v & 0b1111_0000) >> 4;
        match type_bits {
            1 => Ok(PacketType::Connect),
            2 => Ok(PacketType::ConnectAck),
            3 => {
                let flag = v & 0b0000_1111;
                let dup = (flag & 0b0000_1000) == 0b0000_1000;
                let retain = (flag & 0b0000_0001) == 0b0000_0001;
                let qos = match flag & 0b0000_0110 {
                    0b0000_0000 => QoS::AtMostOnce,
                    0b0000_0010 => QoS::AtLeastOnce,
                    0b0000_0100 => QoS::ExactOnce,

                    _ => return Err(DecodeError::InvalidQoS),
                };

                Ok(PacketType::Publish { dup, retain, qos })
            }
            4 => Ok(PacketType::PublishAck),
            5 => Ok(PacketType::PublishReceived),
            6 => Ok(PacketType::PublishRelease),
            7 => Ok(PacketType::PublishComplete),
            8 => Ok(PacketType::Subscribe),
            9 => Ok(PacketType::SubscribeAck),
            10 => Ok(PacketType::Unsubscribe),
            11 => Ok(PacketType::UnsubscribeAck),
            12 => Ok(PacketType::PingRequest),
            13 => Ok(PacketType::PingResponse),
            14 => Ok(PacketType::Disconnect),
            15 => Ok(PacketType::Reserved),

            _ => return Err(DecodeError::InvalidPacketType),
        }
    }
}

impl Default for PacketType {
    fn default() -> Self {
        PacketType::Connect
    }
}

/*
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
*/

/*
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
*/

/// `Remaining Length` uses variable length encoding method. The 7th bit
/// in a byte is used to indicate are bytes are available. And the maximum number
/// of bytes in the `Remaining Length` field is 4 bytes. The maximum value is
/// `0xFF 0xFF 0xFF 0x7F`, `256MB`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct RemainingLength(pub u32);

impl RemainingLength {
    pub fn len(&self) -> usize {
        // TODO(Shaohua): Imply is_empty()
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

impl DecodePacket for RemainingLength {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
        let mut byte: u32;
        let mut value: u32 = 0;
        let mut multiplier = 1;
        loop {
            byte = buf[*offset] as u32;
            *offset += 1;
            value += (byte & 127) * multiplier;
            multiplier *= 128;

            if multiplier > 128 * 128 * 128 * 128 {
                return Err(DecodeError::InvalidRemainingLength);
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

impl EncodePacket for RemainingLength {
    fn to_net(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        if self.0 > 0x7fffffff {
            return Err(EncodeError::InvalidData);
        }

        let mut n = self.0;
        let mut count = 0;
        while n > 0 {
            let mut m = n % 128;
            count += 1;
            n /= 128;
            if n > 0 {
                m |= 128;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FixedHeader {
    pub packet_type: PacketType,
    pub remaining_length: RemainingLength,
}

impl DecodePacket for FixedHeader {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
        let flag = buf[*offset];
        *offset += 1;

        // TODO(Shaohua): Handle invalid packet type.
        let packet_type = PacketType::try_from(flag)?;

        let remaining_length = RemainingLength::from_net(buf, offset)?;

        Ok(FixedHeader {
            packet_type,
            remaining_length,
        })
    }
}

impl EncodePacket for FixedHeader {
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let packet_type: u8 = self.packet_type.into();
        v.push(packet_type);
        self.remaining_length.to_net(v)?;

        // TODO(Shaohua): Calc length.
        Ok(1 + self.remaining_length.len())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<QoS, Self::Error> {
        match v {
            0 => Ok(QoS::AtMostOnce),
            1 => Ok(QoS::AtLeastOnce),
            2 => Ok(QoS::ExactOnce),
            _ => Err(DecodeError::InvalidQoS),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RemainingLength;
    use crate::base::{DecodePacket, EncodePacket};

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
