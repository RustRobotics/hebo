// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::default::Default;
use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};

use super::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, PacketId,
    PacketType, RemainingLength,
};

/// The Client request to unsubscribe topics from the Server.
/// When the Server receives this packet, no more Publish packet will be sent to the Client.
/// Unfinished QoS 1 and QoS 2 packets will be delivered as usual.
///
/// Basic packet struct:
/// ```txt
/// +-------------------------+
/// | Fixed header            |
/// |                         |
/// +-------------------------+
/// | Packet id               |
/// |                         |
/// +-------------------------+
/// | Topic 0 length          |
/// |                         |
/// +-------------------------+
/// | Topic 0 ...             |
/// +-------------------------+
/// | Topic 1 length          |
/// |                         |
/// +-------------------------+
/// | Topic 1 ...             |
/// +-------------------------+
/// | Topic N length          |
/// |                         |
/// +-------------------------+
/// | Topic N ...             |
/// +-------------------------+
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnsubscribePacket {
    /// Used in UnsubscribeAck packet.
    packet_id: PacketId,

    /// Topic filters to be unsubscribed.
    /// Note that these strings must exactly identical to the topic filters used in
    /// Subscribe packets.
    topics: Vec<String>,
}

impl UnsubscribePacket {
    pub fn new(topic: &str, packet_id: PacketId) -> Self {
        UnsubscribePacket {
            packet_id,
            topics: vec![topic.to_string()],
        }
    }

    pub fn new_with_topics(topics: &[&str], packet_id: PacketId) -> Self {
        UnsubscribePacket {
            packet_id,
            topics: topics.iter().map(|t| t.to_string()).collect(),
        }
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn add_topic(&mut self, topic: &str) -> &mut Self {
        self.topics.push(topic.to_string());
        self
    }

    pub fn topics(&self) -> impl Iterator<Item = &str> {
        self.topics.iter().map(|s| s.as_str())
    }
}

impl DecodePacket for UnsubscribePacket {
    fn decode(ba: &mut ByteArray) -> Result<UnsubscribePacket, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;
        if fixed_header.packet_type != PacketType::Unsubscribe {
            return Err(DecodeError::InvalidPacketType);
        }

        let packet_id = ba.read_u16()? as PacketId;

        let mut remaining_length = 2;
        let mut topics = Vec::new();
        while remaining_length < fixed_header.remaining_length.0 {
            let topic_len = ba.read_u16()? as usize;
            remaining_length += 2;
            let topic = ba.read_string(topic_len)?;
            remaining_length += topic_len as u32;
            topics.push(topic);
        }

        Ok(UnsubscribePacket { packet_id, topics })
    }
}

impl EncodePacket for UnsubscribePacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = v.len();
        let mut remaining_length: usize = 2; // packet id
        for topic in &self.topics {
            remaining_length += 2 // topic length bytes
                + topic.len(); // topic
        }

        let fixed_header = FixedHeader {
            packet_type: PacketType::Unsubscribe,
            remaining_length: RemainingLength(remaining_length as u32),
        };
        fixed_header.encode(v)?;

        v.write_u16::<BigEndian>(self.packet_id)?;
        for topic in &self.topics {
            v.write_u16::<BigEndian>(topic.len() as u16)?;
            v.write_all(&topic.as_bytes())?;
        }

        Ok(v.len() - old_len)
    }
}
