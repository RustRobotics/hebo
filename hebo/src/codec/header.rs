// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use super::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, QoS};

pub const MAX_PACKET_LEN: u32 = 0x7F_FF_FF_FF;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PacketType {
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

impl PacketType {
    #[inline]
    pub fn len(&self) -> usize {
        1
    }
}

impl Into<u8> for PacketType {
    fn into(self) -> u8 {
        let type_bits = match self {
            PacketType::Connect => 1,
            PacketType::ConnectAck => 2,
            PacketType::Publish { .. } => 3,
            PacketType::PublishAck => 4,
            PacketType::PublishReceived => 5,
            PacketType::PublishRelease => 6,
            PacketType::PublishComplete => 7,
            PacketType::Subscribe => 8,
            PacketType::SubscribeAck => 9,
            PacketType::Unsubscribe => 10,
            PacketType::UnsubscribeAck => 11,
            PacketType::PingRequest => 12,
            PacketType::PingResponse => 13,
            PacketType::Disconnect => 14,
        };

        let flags_bits = match self {
            PacketType::Publish { dup, qos, retain } => {
                let dup = if dup { 0b0000_10000 } else { 0b0000_0000 };
                let qos = match qos {
                    QoS::AtMostOnce => 0b0000_0000,
                    QoS::AtLeastOnce => 0b0000_0010,
                    QoS::ExactOnce => 0b0000_0100,
                };

                let retain = if retain { 0b0000_0001 } else { 0b0000_0000 };
                dup | qos | retain
            }
            PacketType::PublishRelease => 0b0000_0010,
            PacketType::Subscribe => 0b0000_0010,
            PacketType::Unsubscribe => 0b0000_0010,
            _ => 0,
        };
        ((type_bits & 0b1111_0000) << 4) | flags_bits
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

            _ => return Err(DecodeError::InvalidPacketType),
        }
    }
}

impl Default for PacketType {
    fn default() -> Self {
        PacketType::Connect
    }
}

/// `Remaining Length` uses variable length encoding method. The 7th bit
/// in a byte is used to indicate are bytes are available. And the maximum number
/// of bytes in the `Remaining Length` field is 4 bytes. The maximum value is
/// `0xFF 0xFF 0xFF 0x7F`, `256MB`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RemainingLength(pub u32);

impl RemainingLength {
    pub fn len(&self) -> usize {
        if self.0 > 0x7F_FF_FF {
            4
        } else if self.0 > 0x7F_FF {
            3
        } else if self.0 > 0x7f {
            2
        } else {
            1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl DecodePacket for RemainingLength {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let mut byte: u32;
        let mut remaining_bytes: u32 = 0;
        let mut multiplier = 1;

        // TODO(Shaohua): Simplify
        // Read variant length
        loop {
            byte = ba.one_byte()? as u32;
            remaining_bytes += (byte & 127) * multiplier;
            multiplier *= 128;

            // TODO(Shaohua): Add comments about magic number
            if multiplier > 128 * 128 * 128 * 128 {
                return Err(DecodeError::InvalidRemainingLength);
            }

            if (byte & 128) == 0 {
                break;
            }
        }

        // No need to compare remaining bytes, sometimes we only receive
        // header part of packet and decide whether to prevent from sending
        // more bytes.
        //if ba.remaining_bytes() < remaining_bytes as usize {
        //    Err(DecodeError::InvalidRemainingLength)
        //} else {
        //    Ok(RemainingLength(remaining_bytes))
        //}
        Ok(RemainingLength(remaining_bytes))
    }
}

impl EncodePacket for RemainingLength {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        if self.0 > MAX_PACKET_LEN {
            return Err(EncodeError::TooManyData);
        }

        let mut n = self.0;
        let mut count = 0;
        // TODO(Shaohua): Simplify
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
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let flag = ba.one_byte()?;

        let packet_type = PacketType::try_from(flag)?;
        let remaining_length = RemainingLength::decode(ba)?;

        Ok(FixedHeader {
            packet_type,
            remaining_length,
        })
    }
}

impl EncodePacket for FixedHeader {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let packet_type: u8 = self.packet_type.into();
        v.push(packet_type);

        self.remaining_length.encode(v)?;

        Ok(self.packet_type.len() + self.remaining_length.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let ret = RemainingLength::decode(&buf, &mut offset);
        assert_eq!(ret.unwrap().0, 126);

        let buf = [0x92, 0x01];
        let mut offset = 0;
        let ret = RemainingLength::decode(&buf, &mut offset);
        assert_eq!(ret.unwrap().0, 146);

        let buf = [0x81, 0x80, 0x01];
        let mut offset = 0;
        let ret = RemainingLength::decode(&buf, &mut offset);
        assert_eq!(ret.unwrap().0, 16_385);

        let buf = [0x81, 0x80, 0x80, 0x01];
        let mut offset = 0;
        let ret = RemainingLength::decode(&buf, &mut offset);
        assert_eq!(ret.unwrap().0, 2_097_153);

        let buf = [0xff, 0xff, 0xff, 0x7f];
        let mut offset = 0;
        let ret = RemainingLength::decode(&buf, &mut offset);
        assert_eq!(ret.unwrap().0, 268_435_455);
    }
}