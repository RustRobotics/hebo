// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::base::*;
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SubscribeAckPacket {
    qos: QoS,
    failed: bool,
    packet_id: PacketId,
}

impl SubscribeAckPacket {
    pub fn new(qos: QoS, failed: bool, packet_id: PacketId) -> SubscribeAckPacket {
        SubscribeAckPacket {
            qos,
            failed,
            packet_id,
        }
    }

    pub fn qos(&self) -> QoS {
        self.qos
    }

    pub fn failed(&self) -> bool {
        self.failed
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }
}

impl FromNetPacket for SubscribeAckPacket {
    fn from_net(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::from_net(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::SubscribeAck);

        let packet_id = BigEndian::read_u16(&buf[*offset..*offset + 2]) as PacketId;
        *offset += 2;
        let payload = buf[*offset];
        *offset += 1;

        let failed = payload & 0b1000_0000 == 0b1000_0000;
        let qos = {
            match payload & 0b0000_0011 {
                0b0000_0010 => QoS::ExactOnce,
                0b0000_0001 => QoS::AtLeastOnce,
                0b0000_0000 => QoS::AtMostOnce,
                _ => return Err(Error::InvalidQoS),
            }
        };

        Ok(SubscribeAckPacket {
            packet_id,
            failed,
            qos,
        })
    }
}

impl ToNetPacket for SubscribeAckPacket {
    fn to_net(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = buf.len();
        let fixed_header = FixedHeader {
            packet_type: PacketType::SubscribeAck,
            packet_flags: PacketFlags::SubscribeAck,
            remaining_length: RemainingLength(3),
        };
        fixed_header.to_net(buf)?;
        buf.write_u16::<BigEndian>(self.packet_id).unwrap();

        let flag = if self.failed {
            0b1000_0000
        } else {
            self.qos as u8
        };
        buf.push(flag);

        Ok(buf.len() - old_len)
    }
}
