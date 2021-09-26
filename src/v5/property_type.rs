// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropertyType {
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
    SessionExpiryInterval,

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
    ReceiveMaximum,

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
