// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, QoS, VarInt};

pub trait Packet {
    fn packet_type(&self) -> PacketType;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    /// Authentication exchange
    Auth,
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
            PacketType::Auth => 15,
        };

        let flags_bits = match self {
            PacketType::Publish { dup, qos, retain } => {
                let dup = if dup { 0b0000_1000 } else { 0b0000_0000 };
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
            _ => 0b0000_0000,
        };
        (type_bits << 4) | flags_bits
    }
}

impl TryFrom<u8> for PacketType {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<PacketType, Self::Error> {
        let type_bits = (v & 0b1111_0000) >> 4;
        let flag = v & 0b0000_1111;
        // Where a flag bit is marked as “Reserved”, it is reserved for future use and MUST
        // be set to the value listed [MQTT-2.1.3-1]
        match type_bits {
            1 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in Connect: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::Connect)
                }
            }
            2 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in ConnectAck: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::ConnectAck)
                }
            }
            3 => {
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
            4 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in PublishAck: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::PublishAck)
                }
            }
            5 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in PublishReceived: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::PublishReceived)
                }
            }
            6 => {
                if flag != 0b0000_0010 {
                    log::error!("header: Got packet flag in PublishRelease: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::PublishRelease)
                }
            }
            7 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in PublishComplete: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::PublishComplete)
                }
            }
            8 => {
                if flag != 0b0000_0010 {
                    log::error!("header: Got packet flag in Subscribe: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::Subscribe)
                }
            }
            9 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in Subscribe: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::SubscribeAck)
                }
            }
            10 => {
                if flag != 0b0000_0010 {
                    log::error!("header: Got packet flag in Unsubscribe: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::Unsubscribe)
                }
            }
            11 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in UnsubscribeAck: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::UnsubscribeAck)
                }
            }
            12 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in PingRequest: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::PingRequest)
                }
            }
            13 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in PingResponse: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::PingResponse)
                }
            }
            14 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in Disconnect: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::Disconnect)
                }
            }
            15 => {
                if flag != 0b0000_0000 {
                    log::error!("header: Got packet flag in Auth: {:#b}", flag);
                    Err(DecodeError::InvalidPacketFlags)
                } else {
                    Ok(PacketType::Auth)
                }
            }
            t => {
                log::error!("Invlaid type_bits: {:#b}", t);
                Err(DecodeError::InvalidPacketType)
            }
        }
    }
}

impl Default for PacketType {
    fn default() -> Self {
        PacketType::Connect
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
    packet_type: PacketType,

    /// `Remaining Length` uses variable length encoding method. The 7th bit
    /// in a byte is used to indicate are bytes are available. And the maximum number
    /// of bytes in the `Remaining Length` field is 4 bytes. The maximum value is
    /// `0xFF 0xFF 0xFF 0x7F`, `256MB`.
    remaining_length: VarInt,
}

impl FixedHeader {
    pub fn new(packet_type: PacketType, remaining_length: usize) -> Result<Self, EncodeError> {
        let remaining_length = VarInt::new(remaining_length)?;
        Ok(Self {
            packet_type,
            remaining_length,
        })
    }

    pub fn packet_type(&self) -> PacketType {
        self.packet_type
    }

    pub fn remaining_length(&self) -> usize {
        self.remaining_length.len()
    }

    pub fn remaining_bytes(&self) -> usize {
        self.remaining_length.bytes()
    }
}

impl DecodePacket for FixedHeader {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let flag = ba.read_byte()?;

        let packet_type = PacketType::try_from(flag)?;
        let remaining_length = VarInt::decode(ba)?;

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
