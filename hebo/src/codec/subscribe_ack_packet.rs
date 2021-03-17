// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io;

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

use super::base::{
    FixedHeader, DecodePacket, PacketFlags, PacketId, PacketType, QoS, RemainingLength,
    EncodePacket,
};
use super::error::Error;

/// Reply to each subscribed topic.
#[repr(u8)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SubscribeAck {
    /// Maximum level of QoS the Server granted for this topic.
    QoS(QoS),

    /// This subscription if failed or not.
    Failed,
}

impl Default for SubscribeAck {
    fn default() -> Self {
        SubscribeAck::Failed
    }
}

/// Reply to Subscribe packet.
///
/// Basic structure of packet is:
/// ```txt
/// +---------------------------+
/// | Fixed header              |
/// |                           |
/// +---------------------------+
/// | Packet id                 |
/// |                           |
/// +---------------------------+
/// | Ack 0                     |
/// +---------------------------+
/// | Ack 1                     |
/// +---------------------------+
/// | Ack N ...                 |
/// +---------------------------+
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SubscribeAckPacket {
    /// `packet_id` field is identical in Subscribe packet.
    packet_id: PacketId,

    /// A list of acknowledgement to subscribed topics.
    /// The order of acknowledgement match the order of topic in Subscribe packet.
    acknowledgements: Vec<SubscribeAck>,
}

impl SubscribeAckPacket {
    pub fn new(ack: SubscribeAck, packet_id: PacketId) -> SubscribeAckPacket {
        SubscribeAckPacket {
            packet_id,
            acknowledgements: vec![ack],
        }
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn acknowledgements(&self) -> &[SubscribeAck] {
        &self.acknowledgements
    }
}

impl DecodePacket for SubscribeAckPacket {
    fn decode(buf: &[u8], offset: &mut usize) -> Result<Self, Error> {
        let fixed_header = FixedHeader::decode(buf, offset)?;
        assert_eq!(fixed_header.packet_type, PacketType::SubscribeAck);

        let packet_id = BigEndian::read_u16(&buf[*offset..*offset + 2]) as PacketId;
        *offset += 2;

        let mut acknowledgements = Vec::new();
        let mut remaining_length = 2;

        while remaining_length < fixed_header.remaining_length.0 {
            let payload = buf[*offset];
            *offset += 1;
            remaining_length += 1;
            match payload & 0b1000_0011 {
                0b1000_0000 => acknowledgements.push(SubscribeAck::Failed),
                0b0000_0010 => acknowledgements.push(SubscribeAck::QoS(QoS::ExactOnce)),
                0b0000_0001 => acknowledgements.push(SubscribeAck::QoS(QoS::AtLeastOnce)),
                0b0000_0000 => acknowledgements.push(SubscribeAck::QoS(QoS::AtMostOnce)),
                _ => return Err(Error::InvalidQoS),
            }
        }

        Ok(SubscribeAckPacket {
            packet_id,
            acknowledgements,
        })
    }
}

impl EncodePacket for SubscribeAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let old_len = buf.len();
        let fixed_header = FixedHeader {
            packet_type: PacketType::SubscribeAck,
            packet_flags: PacketFlags::SubscribeAck,
            remaining_length: RemainingLength(3),
        };
        fixed_header.encode(buf)?;
        buf.write_u16::<BigEndian>(self.packet_id)?;

        for ack in &self.acknowledgements {
            let flag = {
                match *ack {
                    SubscribeAck::Failed => 0b1000_0000,
                    SubscribeAck::QoS(qos) => qos as u8,
                }
            };
            buf.push(flag);
        }

        Ok(buf.len() - old_len)
    }
}
