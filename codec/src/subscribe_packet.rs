// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::convert::TryFrom;
use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};

use super::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, PacketId,
    PacketType, QoS, RemainingLength, Topic,
};

/// Topic/QoS pair.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubscribeTopic {
    /// Subscribed `topic` contains wildcard characters to match interested topics with patterns.
    pub topic: String,

    /// Maximum level of QoS of packet the Server can send to the Client.
    pub qos: QoS,
}

/// Subscribe packet is sent from the Client to the Server to subscribe one or more topics.
/// This packet also specifies the maximum QoS with which the Server can send Application
/// message to the Client.
///
/// Basic struct of this packet:
///
/// ```txt
/// +----------------------------+
/// | Fixed header               |
/// |                            |
/// +----------------------------+
/// | Packet Id                  |
/// |                            |
/// +----------------------------+
/// | Topic 0 length             |
/// |                            |
/// +----------------------------+
/// | Topic 0 ...                |
/// +----------------------------+
/// | Topic 0 QoS                |
/// +----------------------------+
/// | Topic 1 length             |
/// |                            |
/// +----------------------------+
/// | Topic 1 ...                |
/// +----------------------------+
/// | Tpoic 1 QoS                |
/// +----------------------------+
/// | ...                        |
/// +----------------------------+
/// ```
///
/// Each topic name is followed by associated QoS flag.
///
/// If a Server receives a Subscribe packet containing a Topic Filter that is identical
/// to an existing Subscription's Topic Filter then it must completely replace existing
/// Subscription with a new Subscription. The Topic Filter in the new Subscription will
/// be identical to the previous Subscription, also QoS may be different. Any existing
/// retained message will be re-sent to the new Subscrption.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubscribePacket {
    /// `packet_id` is used by the Server to reply SubscribeAckPacket to the client.
    packet_id: PacketId,

    /// A list of topic the Client subscribes to.
    topics: Vec<SubscribeTopic>,
}

impl SubscribePacket {
    pub fn new(topic: &str, qos: QoS, packet_id: PacketId) -> Result<SubscribePacket, DecodeError> {
        // TODO(Shaohua): Do not copy topic string.
        Topic::validate_sub_topic(&topic)?;
        let topic = topic.to_string();
        Ok(SubscribePacket {
            packet_id,
            topics: vec![SubscribeTopic { topic, qos }],
        })
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn topics(&self) -> &[SubscribeTopic] {
        &self.topics
    }

    pub fn mut_topics(self) -> Vec<SubscribeTopic> {
        self.topics
    }
}

impl DecodePacket for SubscribePacket {
    fn decode(ba: &mut ByteArray) -> Result<SubscribePacket, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type != PacketType::Subscribe {
            return Err(DecodeError::InvalidPacketType);
        }

        let packet_id = ba.read_u16()?;

        let mut topics = Vec::new();
        let mut remaining_length = 2;

        // Parse topic/qos list.
        while remaining_length < fixed_header.remaining_length.0 {
            let topic_len = ba.read_u16()? as usize;
            remaining_length += 2;

            let topic = ba.read_string(topic_len)?;
            Topic::validate_sub_topic(&topic)?;
            remaining_length += topic_len as u32;

            let qos_flag = ba.read_byte()?;
            remaining_length += 1;
            let qos = QoS::try_from(qos_flag & 0b0000_0011)?;

            topics.push(SubscribeTopic { topic, qos });
        }

        if topics.is_empty() {
            return Err(DecodeError::EmptyTopics);
        }

        Ok(SubscribePacket { packet_id, topics })
    }
}

impl EncodePacket for SubscribePacket {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = buf.len();

        let mut remaining_length = 2; // Variable length
        for topic in &self.topics {
            remaining_length += 2 // Topic length bytes
                + topic.topic.len() // Topic
                + 1; // Requested QoS
        }

        let fixed_header = FixedHeader {
            packet_type: PacketType::Subscribe,
            remaining_length: RemainingLength(remaining_length as u32),
        };
        fixed_header.encode(buf)?;

        // Variable header
        buf.write_u16::<BigEndian>(self.packet_id)?;

        // Payload
        for topic in &self.topics {
            buf.write_u16::<BigEndian>(topic.topic.len() as u16)?;
            buf.write_all(&topic.topic.as_bytes())?;
            let qos: u8 = 0b0000_0011 & (topic.qos as u8);
            buf.push(qos);
        }

        Ok(buf.len() - old_len)
    }
}
