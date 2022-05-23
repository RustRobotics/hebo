// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::io::Write;

use super::property::check_property_type_list;
use super::{FixedHeader, Packet, PacketType, Properties, PropertyType};
use crate::{
    ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, PacketId, PubTopic, QoS,
    VarIntError,
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
/// | Properties ...        |
/// +-----------------------+
/// | Msg payload ...       |
/// +-----------------------+
/// ```
///
/// Note that `packet_id` only appears in `QoS` 1 and `QoS` 2 packets.
///
/// Response of `PublischPacket`:
/// - `QoS` 0, no response
/// - `QoS` 1, `PublishAckPacket`
/// - `QoS` 2, `PublishReceivedPacket`
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PublishPacket {
    /// If the DUP flag is set to 0, it indicates that this is the first occasion that
    /// the Client or Server has attempted to send this PUBLISH packet.
    /// If the DUP flag is set to 1, it indicates that this might be re-delivery of
    /// an earlier attempt to send the packet.
    ///
    /// The DUP flag MUST be set to 1 by the Client or Server when it attempts to re-deliver
    /// a PUBLISH packet [MQTT-3.3.1-1].
    ///
    /// The DUP flag MUST be set to 0 for all QoS 0 messages [MQTT-3.3.1-2].
    ///
    /// The value of the DUP flag from an incoming PUBLISH packet is not propagated
    /// when the PUBLISH packet is sent to subscribers by the Server. The DUP flag
    /// in the outgoing PUBLISH packet is set independently to the incoming PUBLISH packet,
    /// its value MUST be determined solely by whether the outgoing PUBLISH packet
    /// is a retransmission [MQTT-3.3.1-3].
    dup: bool,

    /// This field indicates the level of assurance for delivery of an Application Message.
    ///
    /// If the Server included a Maximum QoS in its CONNACK response to a Client and
    /// it receives a PUBLISH packet with a QoS greater than this, then it uses DISCONNECT
    /// with Reason Code 0x9B (QoS not supported).
    ///
    /// A PUBLISH Packet MUST NOT have both QoS bits set to 1 [MQTT-3.3.1-4].
    ///
    /// If a Server or Client receives a PUBLISH packet which has both QoS bits set to 1
    /// it is a Malformed Packet. Use DISCONNECT with Reason Code 0x81 (Malformed Packet).
    qos: QoS,

    /// If the RETAIN flag is set to 1 in a PUBLISH packet sent by a Client to a Server,
    /// the Server MUST replace any existing retained message for this topic and
    /// store the Application Message [MQTT-3.3.1-5].
    ///
    /// So that it can be delivered to future subscribers whose subscriptions
    /// match its Topic Name. If the Payload contains zero bytes it is processed normally
    /// by the Server but any retained message with the same topic name MUST be removed
    /// and any future subscribers for the topic will not receive a retained message [MQTT-3.3.1-6].
    ///
    /// A retained message with a Payload containing zero bytes MUST NOT be stored
    /// as a retained message on the Server [MQTT-3.3.1-7].
    ///
    /// If the RETAIN flag is 0 in a PUBLISH packet sent by a Client to a Server,
    /// the Server MUST NOT store the message as a retained message and MUST NOT
    /// remove or replace any existing retained message [MQTT-3.3.1-8].
    ///
    /// If the Server included Retain Available in its CONNACK response to a Client
    /// with its value set to 0 and it receives a PUBLISH packet with the RETAIN flag
    /// is set to 1, then it uses the DISCONNECT Reason Code of 0x9A (Retain not supported).
    ///
    /// When a new Non-shared Subscription is made, the last retained message,
    /// if any, on each matching topic name is sent to the Client as directed by
    /// the Retain Handling Subscription Option. These messages are sent with
    /// the RETAIN flag set to 1. Which retained messages are sent is controlled by
    /// the Retain Handling Subscription Option. At the time of the Subscription:
    ///
    /// - If Retain Handling is set to 0 the Server MUST send the retained messages
    ///   matching the Topic Filter of the subscription to the Client [MQTT-3.3.1-9].
    /// - If Retain Handling is set to 1 then if the subscription did not already exist,
    ///   the Server MUST send all retained message matching the Topic Filter
    ///   of the subscription to the Client, and if the subscription did exist
    ///   the Server MUST NOT send the retained messages. [MQTT-3.3.1-10].
    /// - If Retain Handling is set to 2, the Server MUST NOT send the retained messages [MQTT-3.3.1-11].
    ///
    /// If the Server receives a PUBLISH packet with the RETAIN flag set to 1,
    /// and QoS 0 it SHOULD store the new QoS 0 message as the new retained message
    /// for that topic, but MAY choose to discard it at any time. If this happens
    /// there will be no retained message for that topic.
    ///
    /// If the current retained message for a Topic expires, it is discarded and
    /// there will be no retained message for that topic.
    ///
    /// The setting of the RETAIN flag in an Application Message forwarded by the Server
    /// from an established connection is controlled by the Retain As Published subscription option.
    ///
    /// - If the value of Retain As Published subscription option is set to 0, the Server
    ///   MUST set the RETAIN flag to 0 when forwarding an Application Message
    ///   regardless of how the RETAIN flag was set in the received PUBLISH packet [MQTT-3.3.1-12].
    /// - If the value of Retain As Published subscription option is set to 1, the Server
    ///   MUST set the RETAIN flag equal to the RETAIN flag in the received PUBLISH packet [MQTT-3.3.1-13].
    retain: bool,

    /// The Topic Name identifies the information channel to which Payload data is published.
    ///
    /// The Topic Name MUST be present as the first field in the PUBLISH packet Variable Header.
    /// It MUST be a UTF-8 Encoded String as defined in section 1.5.4 [MQTT-3.3.2-1].
    ///
    /// The Topic Name in the PUBLISH packet MUST NOT contain wildcard characters [MQTT-3.3.2-2].
    ///
    /// The Topic Name in a PUBLISH packet sent by a Server to a subscribing Client MUST match the
    /// Subscription’s Topic Filter according to the matching process defined in section 4.7 [MQTT-3.3.2-3].
    ///
    /// However, as the Server is permitted to map the Topic Name to another name,
    /// it might not be the same as the Topic Name in the original PUBLISH packet.
    ///
    /// To reduce the size of the PUBLISH packet the sender can use a Topic Alias.
    /// It is a Protocol Error if the Topic Name is zero length and there is no Topic Alias.
    topic: PubTopic,

    /// The Packet Identifier field is only present in PUBLISH packets where the QoS level is 1 or 2.
    packet_id: PacketId,

    properties: Properties,

    /// Payload contains `msg` field.
    msg: Vec<u8>,
}

/// Properties available in publish packets.
pub const PUBLISH_PROPERTIES: &[PropertyType] = &[
    // A Server MUST send the Payload Format Indicator unaltered to all subscribers
    // receiving the Application Message [MQTT-3.3.2-4].
    PropertyType::PayloadFormatIndicator,
    // If present, the Four Byte value is the lifetime of the Application Message in seconds.
    //
    // If the Message Expiry Interval has passed and the Server has not managed to
    // start onward delivery to a matching subscriber, then it MUST delete the copy
    // of the message for that subscriber [MQTT-3.3.2-5].
    //
    // If absent, the Application Message does not expire.
    //
    // The PUBLISH packet sent to a Client by the Server MUST contain a Message Expiry Interval
    // set to the received value minus the time that the Application Message has been waiting
    // in the Server [MQTT-3.3.2-6].
    PropertyType::MessageExpiryInterval,
    PropertyType::TopicAlias,
    // The Server MUST send the Response Topic unaltered to all subscribers receiving
    // the Application Message [MQTT-3.3.2-15].
    PropertyType::ResponseTopic,
    // The Server MUST send the Correlation Data unaltered to all subscribers receiving
    // the Application Message [MQTT-3.3.2-16].
    PropertyType::CorrelationData,
    // The Server MUST send all User Properties unaltered in a PUBLISH packet
    // when forwarding the Application Message to a Client [MQTT-3.3.2-17].
    //
    // The Server MUST maintain the order of User Properties when forwarding
    // the Application Message [MQTT-3.3.2-18].
    PropertyType::UserProperty,
    // It is a Protocol Error for a PUBLISH packet to contain any Subscription Identifier
    // other than those received in SUBSCRIBE packet which caused it to flow.
    //
    // A PUBLISH packet sent from a Client to a Server MUST NOT contain a Subscription Identifier [MQTT-3.3.4-6].
    //
    // If the Client specified a Subscription Identifier for any of the overlapping eubscriptions
    // the Server MUST send those Subscription Identifiers in the message which
    // is published as the result of the subscriptions [MQTT-3.3.4-3].
    //
    // If the Server sends a single copy of the message it MUST include in the PUBLISH packet
    // the Subscription Identifiers for all matching subscriptions which have
    // a Subscription Identifiers, their order is not significant [MQTT-3.3.4-4].
    //
    // If the Server sends multiple PUBLISH packets it MUST send, in each of them,
    // the Subscription Identifier of the matching subscription if it has
    // a Subscription Identifier [MQTT-3.3.4-5].
    PropertyType::SubscriptionIdentifier,
    // A Server MUST send the Content Type unaltered to all subscribers receiving
    // the Application Message [MQTT-3.3.2-20].
    PropertyType::ContentType,
];

impl PublishPacket {
    /// Create a new publish packet.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn new(topic: &str, qos: QoS, msg: &[u8]) -> Result<Self, EncodeError> {
        let topic = PubTopic::new(topic)?;
        let msg = msg.to_vec();
        Ok(Self {
            qos,
            dup: false,
            retain: false,
            topic,
            packet_id: PacketId::new(0),
            properties: Properties::new(),
            msg,
        })
    }

    /// Append bytes to messages.
    pub fn append(&mut self, msg_parts: &[u8]) {
        self.msg.extend_from_slice(msg_parts);
    }

    /// Update `retian` flag.
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
    /// Returns error if `dup` flag is set in `QoS` 0 packet.
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

    /// Update `qos` value.
    pub fn set_qos(&mut self, qos: QoS) -> &mut Self {
        if qos == QoS::AtMostOnce {
            self.packet_id = PacketId::new(0);
        }
        self.qos = qos;
        self
    }

    /// Get current `qos` value.
    #[must_use]
    pub const fn qos(&self) -> QoS {
        self.qos
    }

    /// Update packet id.
    ///
    /// The packet id field is only present in publish packets where the `QoS` level is 1 or 2.
    pub fn set_packet_id(&mut self, packet_id: PacketId) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    /// Get current packet id.
    #[must_use]
    pub const fn packet_id(&self) -> PacketId {
        self.packet_id
    }

    /// Update topic value.
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

    /// Get a mutable reference to property list.
    pub fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Get a reference to property list.
    #[must_use]
    pub const fn properties(&self) -> &Properties {
        &self.properties
    }

    /// Get a reference to message payload.
    #[must_use]
    pub fn message(&self) -> &[u8] {
        &self.msg
    }

    #[must_use]
    fn get_fixed_header(&self) -> Result<FixedHeader, VarIntError> {
        // TODO(Shaohua): Add properties.bytes()
        let mut remaining_length = self.topic.bytes()
            //+ self.properties.bytes()
            + self.msg.len();
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

        // Parse packet id.
        //
        // A PUBLISH packet MUST NOT contain a Packet Identifier if its QoS value is
        // set to 0 [MQTT-2.2.1-2].
        let packet_id = if qos == QoS::AtMostOnce {
            PacketId::new(0)
        } else {
            let packet_id = PacketId::decode(ba)?;
            // Each time a Client sends a new SUBSCRIBE, UNSUBSCRIBE,or PUBLISH (where QoS > 0)
            // MQTT Control Packet it MUST assign it a non-zero Packet Identifier
            // that is currently unused [MQTT-2.2.1-3].
            if packet_id.value() == 0 {
                return Err(DecodeError::InvalidPacketId);
            }
            packet_id
        };

        let properties = Properties::decode(ba)?;
        if let Err(property_type) = check_property_type_list(properties.props(), PUBLISH_PROPERTIES)
        {
            log::error!(
                "v5/PublishPacket: property type {:?} cannot be used in properties!",
                property_type
            );
            return Err(DecodeError::InvalidPropertyType);
        }

        // It is valid for a PUBLISH Packet to contain a zero length payload.
        if fixed_header.remaining_length() < topic.bytes() {
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

        let msg = ba.read_bytes(msg_len)?;
        let msg = msg.to_vec();
        Ok(Self {
            dup,
            qos,
            retain,
            topic,
            packet_id,
            properties,
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

        self.properties.encode(v)?;

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
