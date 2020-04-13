// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::error::Error;
use std::convert::TryFrom;
use std::io;

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

/// Header flags of a mqtt packet.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FixedHeader {
    pub packet_type: PacketType,
    pub packet_flags: PacketFlags,
}

impl FromNetPacket for FixedHeader {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let flag = buf[*offset];
        *offset += 1;

        // TODO(Shaohua): Handle invalid packet type.
        let packet_type = PacketType::from(flag);
        let packet_flags = PacketFlags::from_u8(packet_type, flag);
        Ok(FixedHeader {
            packet_type,
            packet_flags,
        })
    }
}

impl ToNetPacket for FixedHeader {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        let packet_type: u8 = self.packet_type.into();
        let packet_flags: u8 = self.packet_flags.into();
        v.push(packet_type | packet_flags);

        Ok(1)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProtocolLevel {
    V31 = 3,
    V311 = 4,
    V5 = 5,
}

impl Default for ProtocolLevel {
    fn default() -> Self {
        ProtocolLevel::V311
    }
}

impl TryFrom<u8> for ProtocolLevel {
    type Error = Error;

    fn try_from(v: u8) -> Result<ProtocolLevel, Self::Error> {
        match v {
            3 => Ok(ProtocolLevel::V31),
            4 => Ok(ProtocolLevel::V311),
            5 => Ok(ProtocolLevel::V5),
            _ => Err(Error::InvalidProtocolLevel),
        }
    }
}

impl ToNetPacket for ProtocolLevel {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        v.push(*self as u8);
        Ok(1)
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
