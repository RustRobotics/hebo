// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};

use super::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, PacketId,
    PacketType, QoS, RemainingLength,
};

/// Reply to each subscribed topic.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubscribeAckPacket {
    /// `packet_id` field is identical in Subscribe packet.
    packet_id: PacketId,

    /// A list of acknowledgement to subscribed topics.
    /// The order of acknowledgement match the order of topic in Subscribe packet.
    acknowledgements: Vec<SubscribeAck>,
}

impl SubscribeAckPacket {
    pub fn new(packet_id: PacketId, ack: SubscribeAck) -> Self {
        Self {
            packet_id,
            acknowledgements: vec![ack],
        }
    }

    pub fn with_vec(packet_id: PacketId, acknowledgements: Vec<SubscribeAck>) -> Self {
        Self {
            packet_id,
            acknowledgements,
        }
    }

    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn set_ack(&mut self, ack: &[SubscribeAck]) -> &mut Self {
        self.acknowledgements.clear();
        self.acknowledgements.extend(ack);
        self
    }

    pub fn acknowledgements(&self) -> &[SubscribeAck] {
        &self.acknowledgements
    }
}

impl DecodePacket for SubscribeAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type != PacketType::SubscribeAck {
            return Err(DecodeError::InvalidPacketType);
        }

        let packet_id = ba.read_u16()? as PacketId;

        let mut acknowledgements = Vec::new();
        let mut remaining_length = 2;

        while remaining_length < fixed_header.remaining_length.0 {
            let payload = ba.read_byte()?;
            remaining_length += 1;
            match payload & 0b1000_0011 {
                0b1000_0000 => acknowledgements.push(SubscribeAck::Failed),
                0b0000_0010 => acknowledgements.push(SubscribeAck::QoS(QoS::ExactOnce)),
                0b0000_0001 => acknowledgements.push(SubscribeAck::QoS(QoS::AtLeastOnce)),
                0b0000_0000 => acknowledgements.push(SubscribeAck::QoS(QoS::AtMostOnce)),

                _ => return Err(DecodeError::InvalidQoS),
            }
        }

        Ok(SubscribeAckPacket {
            packet_id,
            acknowledgements,
        })
    }
}

impl EncodePacket for SubscribeAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();
        let fixed_header = FixedHeader {
            packet_type: PacketType::SubscribeAck,
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
