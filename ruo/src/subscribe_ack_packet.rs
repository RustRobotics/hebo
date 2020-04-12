// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::*;
use super::error::Error;
use byteorder::{BigEndian, ByteOrder};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SubscribeAckPacket {
    qos: QoS,
    failed: bool,
    packet_id: PacketId,
}

impl SubscribeAckPacket {
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
        *offset += 1;
        let remaining_len = buf[*offset] as usize;
        assert_eq!(remaining_len, 3);
        *offset += 1;
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
