// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io::Write;

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

use super::topic::Topic;
use super::utils;
use super::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, PacketId,
    PacketType, QoS, RemainingLength,
};

/// PublishPacket is used to transport application messages from the Client to the Server,
/// or from the Server to the Client.
///
/// Basic structure of packet:
///
/// ```txt
///  7                     0
/// +-----------------------+
/// | Fixed header          |
/// |                       |
/// +-----------------------+
/// | Topic name length     |
/// |                       |
/// +-----------------------+
/// | Topic name ...        |
/// +-----------------------+
/// | Packet Identifier     |
/// |                       |
/// +-----------------------+
/// | Msg payload ...       |
/// +-----------------------+
/// ```
///
/// Note that `packet_id` only appears in QoS 1 and QoS 2 packets.
///
/// Response of PublischPacket:
/// * QoS 0, no response
/// * QoS 1, PublishAckPacket
/// * QoS 2, PublishRecPacket
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PublishPacket {
    /// If dup field is false, it indicates that this is the first time to send this packet.
    /// If it is true, then this packet might be re-delivery of an earlier attempt to send the
    /// Packet.
    ///
    /// It must be false if QoS is 0.
    dup: bool,

    /// `qos` field indicates the level of assurance for delivery of packet.
    qos: QoS,

    /// Usage of `retain` flag in PublishPacket is complex:
    ///
    /// If `retain` flag is true in the packet the Client sent to the Server,
    /// this packet is stored on the server so that it can be delivered to future
    /// subscribers. When a new subscription is established, the last retained packet
    /// will be sent to the subscriber. If the Server receives a QoS 0 message with
    /// the `retain` flag set to true, it must discard any message previously retained
    /// for the same topic. The Server should store the new QoS 0 message as the new
    /// retained message for that topic, but may choose to discard it at any time.
    ///
    /// When sending a PublishPacket the Server must set the `retain` flag to true if
    /// a message is sent as a result of a new subscription. The Server must set
    /// `retain` flag to false when sending PublishPacket to already connected subscribers.
    ///
    /// If a PublishPacket sent to the Server with `retain` flag on and payload contains
    /// zero bytes, this packet is normally delivered to subscribers. And this packet is
    /// used as notification to the Server to delete any retained messages on the same topic.
    /// And any future subscribers for the same topic will not receive any retained messages
    /// any more. So this means one-time shot.
    ///
    /// If `retain` flag is false in PublishPacket sent to the Server, status of
    /// the retained message of that topic is not removed or replaced.
    retain: bool,

    /// `topic` name must not contain wildcard characters.
    topic: String,

    /// `packet_id` field is useless if QoS is 0.
    packet_id: PacketId,

    /// Payload contains `msg` field.
    msg: Vec<u8>,
}

impl PublishPacket {
    // TODO(Shaohua): No need to copy topic and msg
    pub fn new(topic: &str, qos: QoS, msg: &[u8]) -> Result<PublishPacket, EncodeError> {
        utils::validate_utf8_string(topic)?;
        Topic::validate_pub_topic(topic)?;

        Ok(PublishPacket {
            qos,
            dup: false,
            retain: false,
            topic: topic.to_string(),
            packet_id: 0,
            msg: msg.to_vec(),
        })
    }

    pub fn set_retain(&mut self, retain: bool) -> &mut Self {
        self.retain = retain;
        self
    }

    pub fn retain(&self) -> bool {
        self.retain
    }

    pub fn set_dup(&mut self, dup: bool) -> &mut Self {
        self.dup = dup;
        self
    }

    pub fn dup(&self) -> bool {
        self.dup
    }

    pub fn qos(&self) -> QoS {
        self.qos
    }

    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn message(&self) -> &[u8] {
        &self.msg
    }
}

impl DecodePacket for PublishPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;

        let (dup, qos, retain) =
            if let PacketType::Publish { dup, qos, retain } = fixed_header.packet_type {
                (dup, qos, retain)
            } else {
                return Err(DecodeError::InvalidPacketType);
            };

        let topic_len = ba.read_u16()? as usize;
        let topic = ba.read_string(topic_len)?;
        Topic::validate_pub_topic(&topic)?;

        // Parse packet id
        let packet_id = if qos != QoS::AtMostOnce {
            ba.read_u16()?
        } else {
            0
        };

        let mut msg_len = fixed_header.remaining_length.0 as usize
            - 2 // topic length bytes
            - topic_len; // topic
        if qos != QoS::AtMostOnce {
            // Packet identifier
            msg_len -= 2;
        }

        // TODO(Shaohua): Only copy a reference.
        let msg = ba.read_bytes(msg_len)?.to_vec();
        Ok(PublishPacket {
            qos,
            retain,
            dup,
            topic,
            packet_id,
            msg,
        })
    }
}

impl EncodePacket for PublishPacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = v.len();

        let mut remaining_length = 2 // Topic length bytes
            + self.topic.len() // Topic length
            + self.msg.len(); // Message length
        if self.qos != QoS::AtMostOnce {
            // For `packet_id` field.
            remaining_length += 2;
        }

        let fixed_header = FixedHeader {
            packet_type: PacketType::Publish {
                dup: self.dup,
                retain: self.retain,
                qos: self.qos,
            },
            remaining_length: RemainingLength(remaining_length as u32),
        };
        fixed_header.encode(v)?;

        // Write variable header
        v.write_u16::<BigEndian>(self.topic.len() as u16)?;
        v.write_all(&self.topic.as_bytes())?;
        if self.qos() != QoS::AtMostOnce {
            v.write_u16::<BigEndian>(self.packet_id())?;
        }

        // Write payload
        v.write_all(&self.msg)?;

        Ok(v.len() - old_len)
    }
}
