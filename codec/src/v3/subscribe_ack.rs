// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::{FixedHeader, Packet, PacketType};
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId, QoS};

/// Reply to each subscribed topic.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscribeAck {
    /// Maximum level of QoS the Server granted for this topic.
    QoS(QoS),

    /// This subscription if failed or not.
    Failed,
}

impl Default for SubscribeAck {
    fn default() -> Self {
        Self::Failed
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
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SubscribeAckPacket {
    /// `packet_id` field is identical in Subscribe packet.
    packet_id: PacketId,

    /// A list of acknowledgement to subscribed topics.
    ///
    /// The order of acknowledgement match the order of topic in Subscribe packet.
    acknowledgements: Vec<SubscribeAck>,
}

impl SubscribeAckPacket {
    /// Create a subscribe ack packet with `ack`.
    #[must_use]
    pub fn new(packet_id: PacketId, ack: SubscribeAck) -> Self {
        Self {
            packet_id,
            acknowledgements: vec![ack],
        }
    }

    /// Create a subscribe ack packet with multiple `acknowledgements`.
    #[must_use]
    pub fn with_vec(packet_id: PacketId, acknowledgements: Vec<SubscribeAck>) -> Self {
        Self {
            packet_id,
            acknowledgements,
        }
    }

    /// Update packet id.
    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    /// Get current packet id.
    #[must_use]
    pub const fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    /// Update acknowledgement list.
    pub fn set_ack(&mut self, ack: &[SubscribeAck]) -> &mut Self {
        self.acknowledgements.clear();
        self.acknowledgements.extend(ack);
        self
    }

    /// Get current acknowledgements.
    #[must_use]
    pub fn acknowledgements(&self) -> &[SubscribeAck] {
        &self.acknowledgements
    }
}

impl DecodePacket for SubscribeAckPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type() != PacketType::SubscribeAck {
            return Err(DecodeError::InvalidPacketType);
        }

        let packet_id = PacketId::decode(ba)?;

        let mut acknowledgements = Vec::new();
        let mut remaining_length = PacketId::bytes();

        while remaining_length < fixed_header.remaining_length() {
            let payload = ba.read_byte()?;
            remaining_length += QoS::bytes();
            match payload & 0b1000_0011 {
                0b1000_0000 => acknowledgements.push(SubscribeAck::Failed),
                0b0000_0010 => acknowledgements.push(SubscribeAck::QoS(QoS::ExactOnce)),
                0b0000_0001 => acknowledgements.push(SubscribeAck::QoS(QoS::AtLeastOnce)),
                0b0000_0000 => acknowledgements.push(SubscribeAck::QoS(QoS::AtMostOnce)),

                _ => return Err(DecodeError::InvalidQoS),
            }
        }

        Ok(Self {
            packet_id,
            acknowledgements,
        })
    }
}

impl EncodePacket for SubscribeAckPacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();
        let remaining_length = PacketId::bytes() + QoS::bytes() * self.acknowledgements.len();
        let fixed_header = FixedHeader::new(PacketType::SubscribeAck, remaining_length)?;
        fixed_header.encode(buf)?;
        self.packet_id.encode(buf)?;

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

impl Packet for SubscribeAckPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::SubscribeAck
    }
}
