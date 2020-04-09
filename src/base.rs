// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::error::Error;
use std::io;

/// Packet identifier
pub type PacketId = u16;

/// Convert native data types to network byte stream.
pub trait ToNetPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize>;
}

pub trait FromNetPacket: Sized {
    fn from_net(buf: &[u8]) -> Result<Self, Error>;
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
    PubAck = 4,

    /// Publish received
    PubRecv = 5,

    /// Publish release
    PubRel = 6,

    /// Publish complete
    PubCompl = 7,

    /// Client subscribe request
    Subscribe = 8,

    /// Subscribe acknowledgement
    SubAck = 9,

    /// Unsubscribe request
    Unsubscribe = 10,

    /// Unsubscribe acknowledgement
    UnsubAck = 11,

    /// Client ping request
    PingReq = 12,

    /// Server ping response
    PingResp = 13,

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
            4 => PacketType::PubAck,
            5 => PacketType::PubRecv,
            6 => PacketType::PubRel,
            7 => PacketType::PubCompl,
            8 => PacketType::Subscribe,
            9 => PacketType::SubAck,
            10 => PacketType::Unsubscribe,
            11 => PacketType::UnsubAck,
            12 => PacketType::PingReq,
            13 => PacketType::PingResp,
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
    Publish {
        dup: bool,
        qos: QoSLevel,
        retain: bool,
    },

    /// Publish acknowledgement
    PubAck,

    /// Publish received
    PubRecv,

    /// Publish release
    PubRel,

    /// Publish complete
    PubCompl,

    /// Client subscribe request
    Subscribe,

    /// Subscribe acknowledgement
    SubAck,

    /// Unsubscribe request
    Unsubscribe,

    /// Unsubscribe acknowledgement
    UnsubAck,

    /// Client ping request
    PingReq,

    /// Server ping response
    PingResp,

    /// Client is disconnecting
    Disconnect,
}

impl Into<u8> for PacketFlags {
    fn into(self) -> u8 {
        match self {
            Self::Connect => 0,
            Self::ConnectAck => 0,
            Self::Publish { dup, qos, retain } => {
                let dup = if dup { 0b0000_10000 } else { 0b0000_0000 };
                let qos = match qos {
                    QoSLevel::QoS0 => 0b0000_0000,
                    QoSLevel::QoS1 => 0b0000_0010,
                    QoSLevel::QoS2 => 0b0000_0100,
                };

                let retain = if retain { 0b0000_0001 } else { 0b0000_0000 };
                dup | qos | retain
            }
            Self::PubAck => 0,
            Self::PubRecv => 0,
            Self::PubRel => 0b0000_0010,
            Self::PubCompl => 0,
            Self::Subscribe => 0b0000_0010,
            Self::SubAck => 0,
            Self::Unsubscribe => 0b0000_0010,
            Self::UnsubAck => 0,
            Self::PingReq => 0,
            Self::PingResp => 0,
            Self::Disconnect => 0,
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
                    0b0000_0000 => QoSLevel::QoS0,
                    0b0000_0010 => QoSLevel::QoS1,
                    0b0000_0100 => QoSLevel::QoS2,
                    // TODO(Shaohua): Handle qos error
                    _ => QoSLevel::QoS0,
                };

                PacketFlags::Publish { dup, qos, retain }
            }
            _ => Self::default(),
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
    fn from_net(buf: &[u8]) -> Result<Self, Error> {
        if buf.len() == 0 {
            return Err(Error::PacketEmpty);
        }
        let flag = buf[0];
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
pub enum Version {
    V31 = 3,
    V311 = 4,
    V5 = 5,
}

impl Default for Version {
    fn default() -> Self {
        Version::V311
    }
}

impl ToNetPacket for Version {
    fn to_net(&self, v: &mut Vec<u8>) -> io::Result<usize> {
        v.push(*self as u8);
        Ok(1)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QoSLevel {
    QoS0 = 0,
    QoS1 = 1,
    QoS2 = 2,
}

impl Default for QoSLevel {
    fn default() -> Self {
        QoSLevel::QoS0
    }
}
