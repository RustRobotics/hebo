// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::convert::TryFrom;

use crate::{
    BinaryData, BoolData, ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket,
    StringData, StringPairData, U16Data, U32Data,
};

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq)]
enum PropertyType {
    PayloadFormatIndicator,
    MessageExpiryInterval,
    ContentType,
    ResponseTopic,
    CorrelationData,
    SubscriptionIdentifier,
    SessionExpiryInterval,
    AssignedClientIdentifier,
    ServerKeepAlive,
    AuthenticationMethod,
    AuthenticationData,
    RequestProblemInformation,
    WillDelayInterval,
    RequestResponseInformation,
    ResponseInformation,
    ServerReference,
    ReasonString,
    ReceiveMaximum,
    TopicAliasMaximum,
    TopicAlias,
    MaximumQoS,
    RetainAvailable,
    UserProperty,
    MaximumPacketSize,
    WildcardSubscriptionAvailable,
    SubscriptionIdentifierAvailable,
    SharedSubscriptionAvailable,
}

impl Into<u8> for PropertyType {
    fn into(self) -> u8 {
        match self {
            Self::PayloadFormatIndicator => 0x01,
            Self::MessageExpiryInterval => 0x02,
            Self::ContentType => 0x03,
            Self::ResponseTopic => 0x08,
            Self::CorrelationData => 0x09,
            Self::SubscriptionIdentifier => 0x0b,
            Self::SessionExpiryInterval => 0x11,
            Self::AssignedClientIdentifier => 0x12,
            Self::ServerKeepAlive => 0x13,
            Self::AuthenticationMethod => 0x15,
            Self::AuthenticationData => 0x16,
            Self::RequestProblemInformation => 0x17,
            Self::WillDelayInterval => 0x18,
            Self::RequestResponseInformation => 0x19,
            Self::ResponseInformation => 0x1a,
            Self::ServerReference => 0x1c,
            Self::ReasonString => 0x1f,
            Self::ReceiveMaximum => 0x21,
            Self::TopicAliasMaximum => 0x22,
            Self::TopicAlias => 0x23,
            Self::MaximumQoS => 0x24,
            Self::RetainAvailable => 0x25,
            Self::UserProperty => 0x26,
            Self::MaximumPacketSize => 0x27,
            Self::WildcardSubscriptionAvailable => 0x28,
            Self::SubscriptionIdentifierAvailable => 0x29,
            Self::SharedSubscriptionAvailable => 0x2a,
        }
    }
}

impl TryFrom<u8> for PropertyType {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<PropertyType, Self::Error> {
        match v {
            0x01 => Ok(Self::PayloadFormatIndicator),
            0x02 => Ok(Self::MessageExpiryInterval),
            0x03 => Ok(Self::ContentType),
            0x08 => Ok(Self::ResponseTopic),
            0x09 => Ok(Self::CorrelationData),
            0x0b => Ok(Self::SubscriptionIdentifier),
            0x11 => Ok(Self::SessionExpiryInterval),
            0x12 => Ok(Self::AssignedClientIdentifier),
            0x13 => Ok(Self::ServerKeepAlive),
            0x15 => Ok(Self::AuthenticationMethod),
            0x16 => Ok(Self::AuthenticationData),
            0x17 => Ok(Self::RequestProblemInformation),
            0x18 => Ok(Self::WillDelayInterval),
            0x19 => Ok(Self::RequestResponseInformation),
            0x1a => Ok(Self::ResponseInformation),
            0x1c => Ok(Self::ServerReference),
            0x1f => Ok(Self::ReasonString),
            0x21 => Ok(Self::ReceiveMaximum),
            0x22 => Ok(Self::TopicAliasMaximum),
            0x23 => Ok(Self::TopicAlias),
            0x24 => Ok(Self::MaximumQoS),
            0x25 => Ok(Self::RetainAvailable),
            0x26 => Ok(Self::UserProperty),
            0x27 => Ok(Self::MaximumPacketSize),
            0x28 => Ok(Self::WildcardSubscriptionAvailable),
            0x29 => Ok(Self::SubscriptionIdentifierAvailable),
            0x2a => Ok(Self::SharedSubscriptionAvailable),
            _ => Err(DecodeError::InvalidPropertyType),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Property {
    /// Payload Format Indicator
    ///
    /// Byte.
    /// Used in PUBLISH, Will Properties.
    ///
    /// Followed by the value of the Payload Format Indicator, either of:
    /// - 0 (0x00) Byte Indicates that the Will Message is unspecified bytes,
    ///   which is equivalent to not sending a Payload Format Indicator.
    /// - 1 (0x01) Byte Indicates that the Will Message is UTF-8 Encoded Character Data.
    ///   The UTF-8 data in the Payload MUST be well-formed UTF-8 as defined
    ///   by the Unicode specification [Unicode] and restated in RFC 3629 [RFC3629].
    ///
    /// It is a Protocol Error to include the Payload Format Indicator more than once.
    /// The Server MAY validate that the Will Message is of the format indicated,
    /// and if it is not send a CONNACK with the Reason Code of 0x99 (Payload format invalid).
    PayloadFormatIndicator(BoolData),

    /// Message Expiry Interval
    ///
    /// Four Byte Integer.
    /// Used in PUBLISH, Will Properties.
    MessageExpiryInterval,

    /// Content Type
    ///
    /// UTF-8 Encoded String.
    /// Used in PUBLISH, Will Properties.
    ContentType,

    /// Response Topic
    ///
    /// UTF-8 Encoded String.
    /// Used in PUBLISH, Will Properties.
    ResponseTopic,

    /// Correlation Data
    ///
    /// Binary Data.
    /// Used in PUBLISH, Will Properties.
    CorrelationData,

    /// Subscription Identifier
    ///
    /// Variable Byte Integer.
    ///
    /// Used in PUBLISH, SUBSCRIBE.
    SubscriptionIdentifier,

    /// Session Expiry Interval
    ///
    /// Four Byte Integer.
    /// Used in CONNECT, CONNACK, DISCONNECT
    ///
    /// Followed by the Four Byte Integer representing the Session Expiry Interval in seconds.
    /// It is a Protocol Error to include the Session Expiry Interval more than once.
    ///
    /// If the Session Expiry Interval is absent the value 0 is used. If it is set to 0,
    /// or is absent, the Session ends when the Network Connection is closed.
    ///
    /// If the Session Expiry Interval is 0xFFFFFFFF (UINT_MAX), the Session does not expire.
    ///
    /// The Client and Server MUST store the Session State after the Network Connection
    /// is closed if the Session Expiry Interval is greater than 0 [MQTT-3.1.2-23].
    ///
    /// When the Session expires the Client and Server need not process the deletion of state atomically.
    SessionExpiryInterval(U32Data),

    /// Assigned Client Identifier
    ///
    /// UTF-8 Encoded String.
    /// Used in CONNACK.
    AssignedClientIdentifier,

    /// Server Keep Alive
    ///
    /// Two Byte Integer.
    /// Used in CONNACK.
    ServerKeepAlive,

    /// Authentication Method
    ///
    /// UTF-8 Encoded String.
    /// Used in CONNECT, CONNACK, AUTH.
    ///
    /// Followed by a UTF-8 Encoded String containing the name of the authentication method
    /// used for extended authentication .It is a Protocol Error to include Authentication Method
    /// more than once.
    ///
    /// If Authentication Method is absent, extended authentication is not performed.
    ///
    /// If a Client sets an Authentication Method in the CONNECT, the Client MUST NOT
    /// send any packets other than AUTH or DISCONNECT packets until it has received
    /// a CONNACK packet [MQTT-3.1.2-30].
    AuthenticationMethod(StringData),

    /// Authentication Data
    ///
    /// Binary Data.
    /// Used in CONNECT, CONNACK, AUTH.
    ///
    /// Followed by Binary Data containing authentication data. It is a Protocol Error
    /// to include Authentication Data if there is no Authentication Method.
    /// It is a Protocol Error to include Authentication Data more than once.
    ///
    /// The contents of this data are defined by the authentication method.
    AuthenticationData(BinaryData),

    /// Request Problem Information
    ///
    /// Byte.
    /// Used in CONNECT.
    /// Followed by a Byte with a value of either 0 or 1. It is a Protocol Error
    /// to include Request Problem Information more than once, or to have a value
    /// other than 0 or 1. If the Request Problem Information is absent, the value of 1 is used.
    ///
    /// The Client uses this value to indicate whether the Reason String or User Properties
    /// are sent in the case of failures.
    ///
    /// If the value of Request Problem Information is 0, the Server MAY return a Reason String
    /// or User Properties on a CONNACK or DISCONNECT packet, but MUST NOT send a Reason String
    /// or User Properties on any packet other than PUBLISH, CONNACK, or DISCONNECT [MQTT-3.1.2-29].
    ///
    /// If the value is 0 and the Client receives a Reason String or User Properties in a packet
    /// other than PUBLISH, CONNACK, or DISCONNECT, it uses a DISCONNECT packet
    /// with Reason Code 0x82 (Protocol Error).
    ///
    /// If this value is 1, the Server MAY return a Reason String or User Properties
    /// on any packet where it is allowed.
    RequestProblemInformation(BoolData),

    /// Will Delay Interval
    ///
    /// Four Byte Integer.
    /// Will Properties.
    ///
    /// Followed by the Four Byte Integer representing the Will Delay Interval in seconds.
    /// It is a Protocol Error to include the Will Delay Interval more than once.
    /// If the Will Delay Interval is absent, the default value is 0 and there is
    /// no delay before the Will Message is published.
    ///
    /// The Server delays publishing the Client’s Will Message until the Will Delay Interval
    /// has passed or the Session ends, whichever happens first. If a new Network Connection
    /// to this Session is made before the Will Delay Interval has passed, the Serverx
    /// MUST NOT send the Will Message [MQTT-3.1.3-9].
    WillDelayInterval(U32Data),

    /// Request Response Information
    ///
    /// Byte.
    /// Used in CONNECT.
    ///
    /// Followed by a Byte with a value of either 0 or 1. It is Protocol Error
    /// to include the Request Response Information more than once, or to have a value
    /// other than 0 or 1. If the Request Response Information is absent, the value of 0 is used.
    ///
    /// The Client uses this value to request the Server to return Response Information
    /// in the CONNACK. A value of 0 indicates that the Server MUST NOT return
    /// Response Information [MQTT-3.1.2-28].
    ///
    /// If the value is 1 the Server MAY return Response Information in the CONNACK packet.
    RequestResponseInformation(BoolData),

    /// Response Information
    ///
    /// UTF-8 Encoded String.
    /// Used in CONNACK.
    ResponseInformation,

    /// Server Reference
    ///
    /// UTF-8 Encoded String.
    /// Used in CONNACK, DISCONNECT.
    ServerReference,

    /// Reason String
    ///
    /// UTF-8 Encoded String.
    /// Used in CONNACK, PUBACK, PUBREC, PUBREL, PUBCOMP, SUBACK,
    /// UNSUBACK, DISCONNECT, AUTH.
    ReasonString,

    /// Receive Maximum
    ///
    /// Two Byte Integer.
    /// Used in CONNECT, CONNACK.
    ///
    /// Followed by the Two Byte Integer representing the Receive Maximum value.
    /// It is a Protocol Error to include the Receive Maximum value more than once
    /// or for it to have the value 0.
    ///
    /// The Client uses this value to limit the number of QoS 1 and QoS 2 publications
    /// that it is willing to process concurrently. There is no mechanism to limit
    /// the QoS 0 publications that the Server might try to send.
    ///
    /// The value of Receive Maximum applies only to the current Network Connection.
    /// If the Receive Maximum value is absent then its value defaults to 65,535.
    ReceiveMaximum(U16Data),

    /// Topic Alias Maximum
    ///
    /// Two Byte Integer.
    /// Used in CONNECT, CONNACK.
    ///
    /// Followed by the Two Byte Integer representing the Topic Alias Maximum value.
    /// It is a Protocol Error to include the Topic Alias Maximum value more than once.
    /// If the Topic Alias Maximum property is absent, the default value is 0.
    ///
    /// This value indicates the highest value that the Client will accept as a Topic Alias
    /// sent by the Server. The Client uses this value to limit the number of Topic Aliases
    /// that it is willing to hold on this Connection. The Server MUST NOT send a Topic Alias
    /// in a PUBLISH packet to the Client greater than Topic Alias Maximum [MQTT-3.1.2-26].
    ///
    /// A value of 0 indicates that the Client does not accept any Topic Aliases
    /// on this connection. If Topic Alias Maximum is absent or zero, the Server
    /// MUST NOT send any Topic Aliases to the Client [MQTT-3.1.2-27].
    TopicAliasMaximum(U16Data),

    /// Topic Alias.
    ///
    /// Two Byte Integer.
    /// Used in PUBLISH.
    TopicAlias,

    /// Maximum QoS
    ///
    /// Byte.
    /// Used in CONNACK.
    MaximumQoS,

    /// Retain Available
    ///
    /// Byte.
    /// Used in CONNACK.
    RetainAvailable,

    /// User Property
    ///
    /// UTF-8 String Pair.
    /// Used in CONNECT, CONNACK, PUBLISH, Will Properties, PUBACK, PUBREC,
    /// PUBREL, PUBCOMP, SUBSCRIBE, SUBACK, UNSUBSCRIBE, UNSUBACK, DISCONNECT, AUTH.
    ///
    /// Followed by a UTF-8 String Pair.
    ///
    /// The User Property is allowed to appear multiple times to represent multiple name,
    /// value pairs. The same name is allowed to appear more than once.
    UserProperty(StringPairData),

    /// Maximum Packet Size
    ///
    /// Four Byte Integer.
    /// Used in CONNECT, CONNACK.
    ///
    /// Followed by a Four Byte Integer representing the Maximum Packet Size the Client
    /// is willing to accept. If the Maximum Packet Size is not present, no limit
    /// on the packet size is imposed beyond the limitations in the protocol
    /// as a result of the remaining length encoding and the protocol header sizes.
    ///
    /// It is a Protocol Error to include the Maximum Packet Size more than once,
    /// or for the value to be set to zero.
    ///
    /// The packet size is the total number of bytes in an MQTT Control Packet.
    /// The Client uses the Maximum Packet Size to inform the Server that it will
    /// not process packets exceeding this limit.
    ///
    /// The Server MUST NOT send packets exceeding Maximum Packet Size to the Client [MQTT-3.1.2-24].
    ///
    /// If a Client receives a packet whose size exceeds this limit, this is a Protocol Error,
    /// the Client uses DISCONNECT with Reason Code 0x95 (Packet too large).
    ///
    /// Where a Packet is too large to send, the Server MUST discard it without sending it
    /// and then behave as if it had completed sending that Application Message [MQTT-3.1.2-25].
    ///
    /// In the case of a Shared Subscription where the message is too large to send to
    /// one or more of the Clients but other Clients can receive it, the Server
    /// can choose either discard the message without sending the message to any of the Clients,
    /// or to send the message to one of the Clients that can receive it.
    MaximumPacketSize(U32Data),

    /// Wildcard Subscription Available
    ///
    /// Byte.
    /// Used in CONNACK.
    WildcardSubscriptionAvailable,

    /// Subscription Identifier Available
    ///
    /// Byte.
    /// Used in CONNACK.
    SubscriptionIdentifierAvailable,

    /// Shared Subscription Available
    ///
    /// Byte.
    /// Used in CONNACK.
    SharedSubscriptionAvailable,
}

impl Property {
    /// The Client uses this value to limit the number of QoS 1 and QoS 2 publications that
    /// it is willing to process concurrently. There is no mechanism to limit
    /// the QoS 0 publications that the Server might try to send.
    pub const fn default_receive_maximum() -> u16 {
        u16::MAX
    }

    pub const fn default_topic_alias_maximum() -> u16 {
        0
    }

    pub const fn default_request_respones_information() -> bool {
        false
    }

    pub const fn default_request_problem_information() -> bool {
        true
    }

    pub const fn default_will_delay_interval() -> u32 {
        0
    }
}

impl DecodePacket for Property {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let property_type_byte = ba.read_byte()?;
        let property_type = PropertyType::try_from(property_type_byte)?;
        match property_type {
            PropertyType::SessionExpiryInterval => {
                let interval = U32Data::decode(ba)?;
                Ok(Self::SessionExpiryInterval(interval))
            }
            PropertyType::ReceiveMaximum => {
                let max = U16Data::decode(ba)?;
                Ok(Self::ReceiveMaximum(max))
            }
            PropertyType::MaximumPacketSize => {
                let max = U32Data::decode(ba)?;
                Ok(Self::MaximumPacketSize(max))
            }
            PropertyType::RequestResponseInformation => {
                let on = BoolData::decode(ba)?;
                Ok(Self::RequestResponseInformation(on))
            }
            PropertyType::RequestProblemInformation => {
                let on = BoolData::decode(ba)?;
                Ok(Self::RequestProblemInformation(on))
            }
            PropertyType::UserProperty => {
                let pair = StringPairData::decode(ba)?;
                Ok(Self::UserProperty(pair))
            }
            PropertyType::AuthenticationMethod => {
                let method = StringData::decode(ba)?;
                Ok(Self::AuthenticationMethod(method))
            }
            PropertyType::AuthenticationData => {
                let data = BinaryData::decode(ba)?;
                Ok(Self::AuthenticationData(data))
            }
            PropertyType::WillDelayInterval => {
                let interval = U32Data::decode(ba)?;
                Ok(Self::WillDelayInterval(interval))
            }
            PropertyType::PayloadFormatIndicator => {
                let on = BoolData::decode(ba)?;
                Ok(Self::PayloadFormatIndicator(on))
            }
            _ => unimplemented!(),
        }
    }
}

impl EncodePacket for Property {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        match self {
            Self::SessionExpiryInterval(interval) => interval.encode(buf),
            Self::ReceiveMaximum(max) => max.encode(buf),
            Self::MaximumPacketSize(max) => max.encode(buf),
            Self::RequestResponseInformation(on) => on.encode(buf),
            Self::RequestProblemInformation(on) => on.encode(buf),
            Self::UserProperty(pair) => pair.encode(buf),
            Self::AuthenticationMethod(method) => method.encode(buf),
            Self::AuthenticationData(data) => data.encode(buf),
            Self::WillDelayInterval(interval) => interval.encode(buf),
            Self::PayloadFormatIndicator(on) => on.encode(buf),
            _ => unimplemented!(),
        }
    }
}

pub type Properties = Vec<Property>;

impl DecodePacket for Properties {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        unimplemented!()
    }
}

impl EncodePacket for Properties {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        unimplemented!()
    }
}
