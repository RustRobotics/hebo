// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::convert::TryFrom;

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

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
    PayloadFormatIndicator,

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
    SessionExpiryInterval(u32),

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
    AuthenticationMethod,

    /// Authentication Data
    ///
    /// Binary Data.
    /// Used in CONNECT, CONNACK, AUTH.
    AuthenticationData,

    /// Request Problem Information
    ///
    /// Byte.
    /// Used in CONNECT.
    RequestProblemInformation,

    /// Will Delay Interval
    ///
    /// Four Byte Integer.
    /// Will Properties.
    WillDelayInterval,

    /// Request Response Information
    ///
    /// Byte.
    /// Used in CONNECT.
    RequestResponseInformation,

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
    ReceiveMaximum(u16),

    /// Topic Alias Maximum
    ///
    /// Two Byte Integer.
    /// Used in CONNECT, CONNACK.
    TopicAliasMaximum,

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
    UserProperty,

    /// Maximum Packet Size
    ///
    /// Four Byte Integer.
    /// Used in CONNECT, CONNACK.
    MaximumPacketSize,

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

impl DecodePacket for Property {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let property_type_byte = ba.read_byte()?;
        let property_type = PropertyType::try_from(property_type_byte)?;
        match property_type {
            PropertyType::SessionExpiryInterval => {
                let interval = ba.read_u32()?;
                Ok(Self::SessionExpiryInterval(interval))
            }
            PropertyType::ReceiveMaximum => {
                let max = ba.read_u16()?;
                Ok(Self::ReceiveMaximum(max))
            }
            _ => unimplemented!(),
        }
    }
}

impl EncodePacket for Property {
    fn encode(&self, v: &mut Vec<u8>) -> Result<usize, EncodeError> {
        match self {
            Self::SessionExpiryInterval(interval) => {
                v.write_u32::<BigEndian>(*interval)?;
                Ok(4)
            }
            Self::ReceiveMaximum(max) => {
                v.write_u16::<BigEndian>(*max)?;
                Ok(2)
            }
            _ => unimplemented!(),
        }
    }
}
