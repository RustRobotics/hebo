// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::io::Result;

/// Convert native data types to network byte stream.
pub trait ToNetPacket {
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize>;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MsgType {
    Unknown = 0,

    /// Request to connect to broker.
    ConnectCmd = 1,

    /// Broker reply to connect request.
    ConnectAck = 2,

    Publish = 3,
}

impl Default for MsgType {
    fn default() -> Self {
        MsgType::ConnectCmd
    }
}

/// Reserved flags
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Reserved {
    Reserved,
    Publish {
        dup: bool,
        qos: QoSLevel,
        retain: bool,
    },
}

impl Default for Reserved {
    fn default() -> Self {
        Reserved::Reserved
    }
}

/// Header flags of a mqtt packet.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct HeaderFlags {
    pub msg_type: MsgType,
    pub reserved: Reserved,
}

impl ToNetPacket for HeaderFlags {
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize> {
        let msg_type = (self.msg_type as u8 & 0b00001111) << 4;
        let reserved = match self.reserved {
            Reserved::Reserved => 0b0000_0000,
            Reserved::Publish { dup, qos, retain } => {
                let dup = if dup { 0b0000_10000 } else { 0b0000_0000 };
                let qos = match qos {
                    QoSLevel::QoS0 => 0b0000_0000,
                    QoSLevel::QoS1 => 0b0000_0010,
                    QoSLevel::QoS2 => 0b0000_0100,
                };

                let retain = if retain { 0b0000_0001 } else { 0b0000_0000 };
                dup + qos + retain
            }
        };
        let flags = msg_type + reserved;
        v.push(flags);

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
    fn to_net(&self, v: &mut Vec<u8>) -> Result<usize> {
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
