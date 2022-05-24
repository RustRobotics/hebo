// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use bytes::BytesMut;
use std::io::Write;

use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, FixedHeader, Packet, PacketId,
    PacketType, PubTopic, QoS, VarIntError,
};

/// `PublishPacket` is used to transport application messages from the Client to the Server,
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
/// Note that `packet_id` only appears in `QoS` 1 and `QoS` 2 packets.
///
/// Response of `PublischPacket`:
/// * `QoS` 0, no response
/// * `QoS` 1, `PublishAckPacket`
/// * `QoS` 2, `PublishRecPacket`
#[allow(clippy::module_name_repetitions)]
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
    topic: PubTopic,

    /// `packet_id` field is useless if QoS is 0.
    packet_id: PacketId,

    /// Payload contains `msg` field.
    // TODO(Shaohua): Replace with Bytes or Vec<u8>, BytewMut is useless.
    msg: BytesMut,
}

impl PublishPacket {
    /// Create a new publish packet.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn new(topic: &str, qos: QoS, msg: &[u8]) -> Result<Self, EncodeError> {
        // TODO(Shaohua): No need to copy topic and msg
        let topic = PubTopic::new(topic)?;
        Ok(Self {
            qos,
            dup: false,
            retain: false,
            topic,
            packet_id: PacketId::new(0),
            msg: BytesMut::from(msg),
        })
    }

    pub fn append(&mut self, msg_parts: &[u8]) {
        self.msg.extend_from_slice(msg_parts);
    }

    /// Update `retain` flag.
    pub fn set_retain(&mut self, retain: bool) -> &mut Self {
        self.retain = retain;
        self
    }

    /// Get current `retain` flag.
    #[must_use]
    pub const fn retain(&self) -> bool {
        self.retain
    }

    /// Update `dup` flag.
    ///
    /// # Errors
    ///
    /// Returns error if `dup` is set in `QoS` 0 packet.
    pub fn set_dup(&mut self, dup: bool) -> Result<&mut Self, EncodeError> {
        // The DUP flag MUST be set to 0 for all QoS 0 messages [MQTT-3.3.1-2].
        if dup && self.qos == QoS::AtMostOnce {
            return Err(EncodeError::InvalidPacketType);
        }
        self.dup = dup;
        Ok(self)
    }

    /// Get current `dup` flag.
    #[must_use]
    pub const fn dup(&self) -> bool {
        self.dup
    }

    /// Update `QoS` value.
    pub fn set_qos(&mut self, qos: QoS) -> &mut Self {
        if qos == QoS::AtMostOnce {
            self.packet_id = PacketId::new(0);
        }
        self.qos = qos;
        self
    }

    /// Get current `QoS`.
    #[must_use]
    pub const fn qos(&self) -> QoS {
        self.qos
    }

    /// The Packet Identifier field is only present in PUBLISH Packets where the `QoS` level is 1 or 2.
    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    #[must_use]
    pub const fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    /// Update topic.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn set_topic(&mut self, topic: &str) -> Result<&mut Self, EncodeError> {
        self.topic = PubTopic::new(topic)?;
        Ok(self)
    }

    /// Get current topic.
    #[must_use]
    pub fn topic(&self) -> &str {
        self.topic.as_ref()
    }

    #[must_use]
    pub fn message(&self) -> &[u8] {
        &self.msg
    }

    // TODO(Shaohua): Add message related operations.

    fn get_fixed_header(&self) -> Result<FixedHeader, VarIntError> {
        let mut remaining_length = self.topic.bytes() + self.msg.len();
        if self.qos != QoS::AtMostOnce {
            remaining_length += PacketId::bytes();
        }

        let packet_type = PacketType::Publish {
            dup: self.dup,
            retain: self.retain,
            qos: self.qos,
        };
        FixedHeader::new(packet_type, remaining_length)
    }
}

impl DecodePacket for PublishPacket {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let fixed_header = FixedHeader::decode(ba)?;

        let (dup, qos, retain) =
            if let PacketType::Publish { dup, qos, retain } = fixed_header.packet_type() {
                (dup, qos, retain)
            } else {
                return Err(DecodeError::InvalidPacketType);
            };

        // The DUP flag MUST be set to 0 for all QoS 0 messages [MQTT-3.3.1-2].
        if dup && qos == QoS::AtMostOnce {
            return Err(DecodeError::InvalidPacketFlags);
        }

        // In the QoS 1 delivery protocol, the Sender MUST send a PUBLISH Packet
        // containing this Packet Identifier with QoS=1, DUP=0.
        // [MQTT-4.3.2-1].
        if dup && qos == QoS::AtLeastOnce {
            return Err(DecodeError::InvalidPacketFlags);
        }

        let topic = PubTopic::decode(ba)?;
        log::info!("topic: {:?}", &topic);

        // Parse packet id.
        // The Packet Identifier field is only present in PUBLISH Packets where the QoS level is 1 or 2.
        let packet_id = if qos == QoS::AtMostOnce {
            PacketId::new(0)
        } else {
            let packet_id = PacketId::decode(ba)?;
            if packet_id.value() == 0 {
                // SUBSCRIBE, UNSUBSCRIBE, and PUBLISH (in cases where QoS > 0) Control Packets
                // MUST contain a non-zero 16-bit Packet Identifier. [MQTT-2.3.1-1]
                return Err(DecodeError::InvalidPacketId);
            }
            packet_id
        };

        // It is valid for a PUBLISH Packet to contain a zero length payload.
        if fixed_header.remaining_length() < topic.bytes() {
            log::info!(
                "remaining length: {}, topic bytes: {}",
                fixed_header.remaining_length(),
                topic.bytes()
            );
            return Err(DecodeError::InvalidRemainingLength);
        }
        let mut msg_len = fixed_header.remaining_length() - topic.bytes();
        if qos != QoS::AtMostOnce {
            if msg_len < PacketId::bytes() {
                return Err(DecodeError::InvalidRemainingLength);
            }

            // Packet identifier is presesnt in QoS1/QoS2 packets.
            msg_len -= PacketId::bytes();
        }

        let msg = BytesMut::from(ba.read_bytes(msg_len)?);
        Ok(Self {
            dup,
            qos,
            retain,
            topic,
            packet_id,
            msg,
        })
    }
}

impl EncodePacket for PublishPacket {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let old_len = v.len();

        let fixed_header = self.get_fixed_header()?;
        fixed_header.encode(v)?;

        // Write variable header
        self.topic.encode(v)?;

        // The Packet Identifier field is only present in PUBLISH Packets where the QoS level is 1 or 2.
        if self.qos() != QoS::AtMostOnce {
            self.packet_id.encode(v)?;
        }

        // Write payload
        v.write_all(&self.msg)?;

        Ok(v.len() - old_len)
    }
}

impl Packet for PublishPacket {
    fn packet_type(&self) -> PacketType {
        PacketType::Publish {
            dup: self.dup,
            retain: self.retain,
            qos: self.qos,
        }
    }

    fn bytes(&self) -> Result<usize, VarIntError> {
        let fixed_header = self.get_fixed_header()?;
        Ok(fixed_header.bytes() + fixed_header.remaining_length())
    }
}
