// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use crate::{
    utils::validate_client_id, BinaryData, BoolData, ByteArray, DecodeError, DecodePacket,
    EncodeError, EncodePacket, PubTopic, QoS, StringData, StringPairData, U16Data, U32Data, VarInt,
};

pub const MULTIPLE_PROPERTIES: &[PropertyType] = &[
    PropertyType::UserProperty,
    PropertyType::SubscriptionIdentifier,
];

pub fn check_multiple_subscription_identifiers(
    properties: &[Property],
) -> Result<(), PropertyType> {
    let count = properties
        .iter()
        .filter(|p| p.property_type() == PropertyType::SubscriptionIdentifier)
        .count();
    if count > 1 {
        return Err(PropertyType::SubscriptionIdentifier);
    }
    Ok(())
}

pub fn check_property_type_list(
    properties: &[Property],
    types: &[PropertyType],
) -> Result<(), PropertyType> {
    if properties.is_empty() {
        return Ok(());
    }

    for property in properties {
        if !types.contains(&property.property_type()) {
            return Err(property.property_type());
        }
    }

    for property_type in types {
        let count = properties
            .iter()
            .filter(|p| p.property_type() == *property_type)
            .count();
        if count > 1 && !MULTIPLE_PROPERTIES.contains(property_type) {
            return Err(*property_type);
        }
    }

    Ok(())
}

#[allow(clippy::module_name_repetitions)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType {
    PayloadFormatIndicator = 0x01,
    MessageExpiryInterval = 0x02,
    ContentType = 0x03,
    ResponseTopic = 0x08,
    CorrelationData = 0x09,
    SubscriptionIdentifier = 0x0b,
    SessionExpiryInterval = 0x11,
    AssignedClientIdentifier = 0x12,
    ServerKeepAlive = 0x13,
    AuthenticationMethod = 0x15,
    AuthenticationData = 0x16,
    RequestProblemInformation = 0x17,
    WillDelayInterval = 0x18,
    RequestResponseInformation = 0x19,
    ResponseInformation = 0x1a,
    ServerReference = 0x1c,
    ReasonString = 0x1f,
    ReceiveMaximum = 0x21,
    TopicAliasMaximum = 0x22,
    TopicAlias = 0x23,
    MaximumQoS = 0x24,
    RetainAvailable = 0x25,
    UserProperty = 0x26,
    MaximumPacketSize = 0x27,
    WildcardSubscriptionAvailable = 0x28,
    SubscriptionIdentifierAvailable = 0x29,
    SharedSubscriptionAvailable = 0x2a,
}

impl PropertyType {
    /// Get byte length used in packet.
    #[must_use]
    pub const fn bytes() -> usize {
        1
    }
}

impl TryFrom<u8> for PropertyType {
    type Error = DecodeError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
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
    ///
    /// [Unicode]: https://unicode.org/standard/standard.html
    /// [RFC3629]: https://datatracker.ietf.org/doc/html/rfc3629
    PayloadFormatIndicator(BoolData),

    /// Message Expiry Interval
    ///
    /// Four Byte Integer.
    /// Used in PUBLISH, Will Properties.
    ///
    /// Followed by the Four Byte Integer representing the Message Expiry Interval.
    /// It is a Protocol Error to include the Message Expiry Interval more than once.
    ///
    /// If present, the Four Byte value is the lifetime of the Will Message in seconds
    /// and is sent as the Publication Expiry Interval when the Server publishes the Will Message.
    ///
    /// If absent, no Message Expiry Interval is sent when the Server publishes the Will Message.
    MessageExpiryInterval(U32Data),

    /// Content Type
    ///
    /// UTF-8 Encoded String.
    /// Used in PUBLISH, Will Properties.
    ///
    /// Followed by a UTF-8 Encoded String describing the content of the Will Message.
    /// It is a Protocol Error to include the Content Type more than once.
    /// The value of the Content Type is defined by the sending and receiving application.
    ContentType(StringData),

    /// Response Topic
    ///
    /// UTF-8 Encoded String.
    /// Used in PUBLISH, Will Properties.
    ///
    /// Followed by a UTF-8 Encoded String which is used as the Topic Name for a response message.
    /// It is a Protocol Error to include the Response Topic more than once. The presence
    /// of a Response Topic identifies the Will Message as a Request.
    ResponseTopic(PubTopic),

    /// Correlation Data
    ///
    /// Binary Data.
    /// Used in PUBLISH, Will Properties.
    ///
    /// Followed by Binary Data. The Correlation Data is used by the sender of
    /// the Request Message to identify which request the Response Message is for
    /// when it is received. It is a Protocol Error to include Correlation Data
    /// more than once. If the Correlation Data is not present, the Requester
    /// does not require any correlation data.
    ///
    /// The value of the Correlation Data only has meaning to the sender of
    /// the Request Message and receiver of the Response Message
    CorrelationData(BinaryData),

    /// Subscription Identifier
    ///
    /// Variable Byte Integer.
    ///
    /// Used in PUBLISH, SUBSCRIBE.
    ///
    /// Followed by a Variable Byte Integer representing the identifier of the subscription.
    ///
    /// The Subscription Identifier can have the value of 1 to 268,435,455.
    /// It is a Protocol Error if the Subscription Identifier has a value of 0.
    /// Multiple Subscription Identifiers will be included if the publication
    /// is the result of a match to more than one subscription, in this case
    /// their order is not significant.
    SubscriptionIdentifier(VarInt),

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
    ///
    /// Followed by the UTF-8 string which is the Assigned Client Identifier. It is
    /// a Protocol Error to include the Assigned Client Identifier more than once.
    ///
    /// The Client Identifier which was assigned by the Server because a zero length Client Identifier
    /// was found in the CONNECT packet.
    ///
    /// If the Client connects using a zero length Client Identifier, the Server
    /// MUST respond with a CONNACK containing an Assigned Client Identifier.
    /// The Assigned Client Identifier MUST be a new Client Identifier not used
    /// by any other Session currently in the Server [MQTT-3.2.2-16].
    AssignedClientIdentifier(StringData),

    /// Server Keep Alive
    ///
    /// Two Byte Integer.
    /// Used in CONNACK.
    ///
    /// Followed by a Two Byte Integer with the Keep Alive time assigned by the Server.
    /// If the Server sends a Server Keep Alive on the CONNACK packet, the Client
    /// MUST use this value instead of the Keep Alive value the Client sent
    /// on CONNECT [MQTT-3.2.2-21].
    ///
    /// If the Server does not send the Server Keep Alive, the Server MUST use
    /// the Keep Alive value set by the Client on CONNECT [MQTT-3.2.2-22].
    ///
    /// It is a Protocol Error to include the Server Keep Alive more than once.
    ServerKeepAlive(U16Data),

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
    ///
    /// Followed by a UTF-8 Encoded String which is used as the basis for creating
    /// a Response Topic. The way in which the Client creates a Response Topic
    /// from the Response Information is not defined by this specification.
    /// It is a Protocol Error to include the Response Information more than once.
    ///
    /// If the Client sends a Request Response Information with a value 1, it is
    /// OPTIONAL for the Server to send the Response Information in the CONNACK.
    ResponseInformation(StringData),

    /// Server Reference
    ///
    /// UTF-8 Encoded String.
    /// Used in CONNACK, DISCONNECT.
    ///
    /// Followed by a UTF-8 Encoded String which can be used by the Client to identify another
    /// Server to use. It is a Protocol Error to include the Server Reference more than once.
    ///
    /// The Server uses a Server Reference in either a CONNACK or DISCONNECT packet
    /// with Reason code of 0x9C (Use another server) or Reason Code 0x9D (Server moved).
    ServerReference(StringData),

    /// Reason String
    ///
    /// UTF-8 Encoded String.
    /// Used in CONNACK, PUBACK, PUBREC, PUBREL, PUBCOMP, SUBACK,
    /// UNSUBACK, DISCONNECT, AUTH.
    ///
    /// Followed by the UTF-8 Encoded String representing the reason associated with
    /// this response. This Reason String is a human readable string designed
    /// for diagnostics and SHOULD NOT be parsed by the Client.
    ///
    /// The Server uses this value to give additional information to the Client.
    /// The Server MUST NOT send this property if it would increase the size
    /// of the CONNACK packet beyond the Maximum Packet Size specified
    /// by the Client [MQTT-3.2.2-19].
    ///
    /// It is a Protocol Error to include the Reason String more than once.
    ReasonString(StringData),

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
    ///
    /// Followed by the Two Byte integer representing the Topic Alias value. It is
    /// a Protocol Error to include the Topic Alias value more than once.
    ///
    /// A Topic Alias is an integer value that is used to identify the Topic instead of
    /// using the Topic Name. This reduces the size of the PUBLISH packet, and is useful
    /// when the Topic Names are long and the same Topic Names are used repetitively
    /// within a Network Connection.
    ///
    /// The sender decides whether to use a Topic Alias and chooses the value. It sets a Topic Alias
    /// mapping by including a non-zero length Topic Name and a Topic Alias in the PUBLISH packet.
    /// The receiver processes the PUBLISH as normal but also sets the specified Topic Alias
    /// mapping to this Topic Name.
    ///
    /// If a Topic Alias mapping has been set at the receiver, a sender can send a PUBLISH packet
    /// that contains that Topic Alias and a zero length Topic Name. The receiver then treats
    /// the incoming PUBLISH as if it had contained the Topic Name of the Topic Alias.
    ///
    /// A sender can modify the Topic Alias mapping by sending another PUBLISH in the same
    /// Network Connection with the same Topic Alias value and a different non-zero length Topic Name.
    ///
    /// Topic Alias mappings exist only within a Network Connection and last only for the lifetime
    /// of that Network Connection. A receiver MUST NOT carry forward any Topic Alias mappings
    /// from one Network Connection to another [MQTT-3.3.2-7].
    ///
    /// A Topic Alias of 0 is not permitted. A sender MUST NOT send a PUBLISH packet
    /// containing a Topic Alias which has the value 0 [MQTT-3.3.2-8].
    ///
    /// A Client MUST NOT send a PUBLISH packet with a Topic Alias greater than
    /// the Topic Alias Maximum value returned by the Server in the CONNACK packet [MQTT-3.3.2-9].
    ///
    /// A Client MUST accept all Topic Alias values greater than 0 and less than or equal to
    /// the Topic Alias Maximum value that it sent in the CONNECT packet [MQTT-3.3.2-10].
    ///
    /// A Server MUST NOT send a PUBLISH packet with a Topic Alias greater than
    /// the Topic Alias Maximum value sent by the Client in the CONNECT packet [MQTT-3.3.2-11].
    ///
    /// A Server MUST accept all Topic Alias values greater than 0 and less than or equal to
    /// the Topic Alias Maximum value that it returned in the CONNACK packet [MQTT-3.3.2-12].
    ///
    /// The Topic Alias mappings used by the Client and Server are independent from each other.
    /// Thus, when a Client sends a PUBLISH containing a Topic Alias value of 1 to a Server
    /// and the Server sends a PUBLISH with a Topic Alias value of 1 to that Client
    /// they will in general be referring to different Topics.
    TopicAlias(U16Data),

    /// Maximum QoS
    ///
    /// Byte.
    /// Used in CONNACK.
    ///
    /// Followed by a Byte with a value of either 0 or 1. It is a Protocol Error
    /// to include Maximum QoS more than once, or to have a value other than 0 or 1.
    /// If the Maximum QoS is absent, the Client uses a Maximum QoS of 2.
    ///
    /// If a Server does not support QoS 1 or QoS 2 PUBLISH packets it MUST send
    /// a Maximum QoS in the CONNACK packet specifying the highest QoS it supports [MQTT-3.2.2-9].
    ///
    /// A Server that does not support QoS 1 or QoS 2 PUBLISH packets MUST still
    /// accept SUBSCRIBE packets containing a Requested QoS of 0, 1 or 2 [MQTT-3.2.2-10].
    ///
    /// If a Client receives a Maximum QoS from a Server, it MUST NOT send PUBLISH packets
    /// at a QoS level exceeding the Maximum QoS level specified [MQTT-3.2.2-11].
    ///
    /// It is a Protocol Error if the Server receives a PUBLISH packet with a QoS
    /// greater than the Maximum QoS it specified. In this case use DISCONNECT
    /// with Reason Code 0x9B (QoS not supported).
    ///
    /// If a Server receives a CONNECT packet containing a Will QoS that exceeds
    /// its capabilities, it MUST reject the connection. It SHOULD use a CONNACK packet
    /// with Reason Code 0x9B (QoS not supported) Handling errors, and MUST close
    /// the Network Connection [MQTT-3.2.2-12].
    MaximumQoS(QoS),

    /// Retain Available
    ///
    /// Byte.
    /// Used in CONNACK.
    ///
    /// Followed by a Byte field. If present, this byte declares whether the Server supports
    /// retained messages. A value of 0 means that retained messages are not supported.
    /// A value of 1 means retained messages are supported. If not present, then
    /// retained messages are supported. It is a Protocol Error to include Retain Available
    /// more than once or to use a value other than 0 or 1.
    RetainAvailable(BoolData),

    /// User Property
    ///
    /// UTF-8 String Pair.
    /// Used in CONNECT, CONNACK, PUBLISH, Will Properties, PUBACK, PUBREC,
    /// PUBREL, PUBCOMP, SUBSCRIBE, SUBACK, UNSUBSCRIBE, UNSUBACK, DISCONNECT, AUTH.
    ///
    /// Followed by a UTF-8 String Pair.
    /// The User Property is allowed to appear multiple times to represent multiple name,
    /// value pairs. The same name is allowed to appear more than once.
    ///
    /// The Server MUST maintain the order of User Properties when publishing
    /// the Will Message [MQTT-3.1.3-10].
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
    ///
    /// Followed by a Byte field. If present, this byte declares whether the Server
    /// supports Wildcard Subscriptions. A value is 0 means that Wildcard Subscriptions
    /// are not supported. A value of 1 means Wildcard Subscriptions are supported.
    /// If not present, then Wildcard Subscriptions are supported. It is a Protocol Error
    /// to include the Wildcard Subscription Available more than once or to send
    /// a value other than 0 or 1.
    ///
    /// If the Server receives a SUBSCRIBE packet containing a Wildcard Subscription
    /// and it does not support Wildcard Subscriptions, this is a Protocol Error.
    /// The Server uses DISCONNECT with Reason Code 0xA2 (Wildcard Subscriptions not supported).
    ///
    /// If a Server supports Wildcard Subscriptions, it can still reject a particular
    /// subscribe request containing a Wildcard Subscription. In this case the Server MAY
    /// send a SUBACK Control Packet with a Reason Code 0xA2 (Wildcard Subscriptions not supported).
    WildcardSubscriptionAvailable(BoolData),

    /// Subscription Identifier Available
    ///
    /// Byte.
    /// Used in CONNACK.
    ///
    /// Followed by a Byte field. If present, this byte declares whether the Server
    /// supports Subscription Identifiers. A value is 0 means that Subscription Identifiers
    /// are not supported. A value of 1 means Subscription Identifiers are supported.
    /// If not present, then Subscription Identifiers are supported. It is a
    /// Protocol Error to include the Subscription Identifier Available more than once,
    /// or to send a value other than 0 or 1.
    SubscriptionIdentifierAvailable(BoolData),

    /// Shared Subscription Available
    ///
    /// Byte.
    /// Used in CONNACK.
    ///
    /// Followed by a Byte field. If present, this byte declares whether the Server
    /// supports Shared Subscriptions. A value is 0 means that Shared Subscriptions
    /// are not supported. A value of 1 means Shared Subscriptions are supported.
    /// If not present, then Shared Subscriptions are supported. It is a Protocol Error
    /// to include the Shared Subscription Available more than once or to send
    /// a value other than 0 or 1.
    ///
    /// If the Server receives a SUBSCRIBE packet containing Shared Subscriptions
    /// and it does not support Shared Subscriptions, this is a Protocol Error.
    /// The Server uses DISCONNECT with Reason Code 0x9E (Shared Subscriptions not supported).
    SharedSubscriptionAvailable(BoolData),
}

impl Property {
    /// Get type of the property.
    #[must_use]
    pub const fn property_type(&self) -> PropertyType {
        match self {
            Self::PayloadFormatIndicator(_) => PropertyType::PayloadFormatIndicator,
            Self::MessageExpiryInterval(_) => PropertyType::MessageExpiryInterval,
            Self::ContentType(_) => PropertyType::ContentType,
            Self::ResponseTopic(_) => PropertyType::ResponseTopic,
            Self::CorrelationData(_) => PropertyType::CorrelationData,
            Self::SubscriptionIdentifier(_) => PropertyType::SubscriptionIdentifier,
            Self::SessionExpiryInterval(_) => PropertyType::SessionExpiryInterval,
            Self::AssignedClientIdentifier(_) => PropertyType::AssignedClientIdentifier,
            Self::ServerKeepAlive(_) => PropertyType::ServerKeepAlive,
            Self::AuthenticationMethod(_) => PropertyType::AuthenticationMethod,
            Self::AuthenticationData(_) => PropertyType::AuthenticationData,
            Self::RequestProblemInformation(_) => PropertyType::RequestProblemInformation,
            Self::WillDelayInterval(_) => PropertyType::WillDelayInterval,
            Self::RequestResponseInformation(_) => PropertyType::RequestResponseInformation,
            Self::ResponseInformation(_) => PropertyType::ResponseInformation,
            Self::ServerReference(_) => PropertyType::ServerReference,
            Self::ReasonString(_) => PropertyType::ReasonString,
            Self::ReceiveMaximum(_) => PropertyType::ReceiveMaximum,
            Self::TopicAliasMaximum(_) => PropertyType::TopicAliasMaximum,
            Self::TopicAlias(_) => PropertyType::TopicAlias,
            Self::MaximumQoS(_) => PropertyType::MaximumQoS,
            Self::RetainAvailable(_) => PropertyType::RetainAvailable,
            Self::UserProperty(_) => PropertyType::UserProperty,
            Self::MaximumPacketSize(_) => PropertyType::MaximumPacketSize,
            Self::WildcardSubscriptionAvailable(_) => PropertyType::WildcardSubscriptionAvailable,
            Self::SubscriptionIdentifierAvailable(_) => {
                PropertyType::SubscriptionIdentifierAvailable
            }
            Self::SharedSubscriptionAvailable(_) => PropertyType::SharedSubscriptionAvailable,
        }
    }

    /// Get byte length used in packets.
    #[allow(clippy::match_same_arms)]
    #[must_use]
    pub fn bytes(&self) -> usize {
        let value_bytes = match self {
            Self::AssignedClientIdentifier(value) => value.bytes(),
            Self::AuthenticationData(value) => value.bytes(),
            Self::AuthenticationMethod(value) => value.bytes(),
            Self::ContentType(value) => value.bytes(),
            Self::CorrelationData(value) => value.bytes(),
            Self::MaximumPacketSize(..) => U32Data::bytes(),
            Self::MaximumQoS(..) => QoS::bytes(),
            Self::MessageExpiryInterval(..) => U32Data::bytes(),
            Self::PayloadFormatIndicator(..) => BoolData::bytes(),
            Self::ReasonString(value) => value.bytes(),
            Self::ReceiveMaximum(..) => U16Data::bytes(),
            Self::RequestProblemInformation(..) => BoolData::bytes(),
            Self::RequestResponseInformation(..) => BoolData::bytes(),
            Self::ResponseInformation(value) => value.bytes(),
            Self::ResponseTopic(value) => value.bytes(),
            Self::RetainAvailable(..) => BoolData::bytes(),
            Self::ServerKeepAlive(..) => U16Data::bytes(),
            Self::ServerReference(value) => value.bytes(),
            Self::SessionExpiryInterval(..) => U32Data::bytes(),
            Self::SharedSubscriptionAvailable(..) => BoolData::bytes(),
            Self::SubscriptionIdentifier(value) => value.bytes(),
            Self::SubscriptionIdentifierAvailable(..) => BoolData::bytes(),
            Self::TopicAlias(..) => U16Data::bytes(),
            Self::TopicAliasMaximum(..) => U16Data::bytes(),
            Self::UserProperty(value) => value.bytes(),
            Self::WildcardSubscriptionAvailable(..) => BoolData::bytes(),
            Self::WillDelayInterval(..) => U32Data::bytes(),
        };

        PropertyType::bytes() + value_bytes
    }
}

impl Property {
    /// The Client uses this value to limit the number of `QoS` 1 and `QoS` 2 publications that
    /// it is willing to process concurrently.
    ///
    /// There is no mechanism to limit the `QoS` 0 publications that the Server might try to send.
    #[must_use]
    pub const fn default_receive_maximum() -> u16 {
        u16::MAX
    }

    #[must_use]
    pub const fn default_topic_alias_maximum() -> u16 {
        0
    }

    #[must_use]
    pub const fn default_request_respones_information() -> bool {
        false
    }

    #[must_use]
    pub const fn default_request_problem_information() -> bool {
        true
    }

    #[must_use]
    pub const fn default_will_delay_interval() -> u32 {
        0
    }

    #[must_use]
    pub const fn default_wildcard_subscription_available() -> bool {
        true
    }

    #[must_use]
    pub const fn default_subscription_identifier_available() -> bool {
        true
    }

    #[must_use]
    pub const fn default_shared_subscription_available() -> bool {
        true
    }
}

impl DecodePacket for Property {
    #[allow(clippy::too_many_lines)]
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let property_type_byte = ba.read_byte()?;
        log::info!("property_type_byte: {}", property_type_byte);
        let property_type = PropertyType::try_from(property_type_byte)?;
        log::info!("property_type: {:?}", property_type);
        match property_type {
            PropertyType::SessionExpiryInterval => {
                log::info!("SessionExpiryInterval");
                let interval = U32Data::decode(ba)?;
                log::info!("interval: {}", interval);
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
            PropertyType::MessageExpiryInterval => {
                let interval = U32Data::decode(ba)?;
                Ok(Self::MessageExpiryInterval(interval))
            }
            PropertyType::ContentType => {
                let content_type = StringData::decode(ba)?;
                Ok(Self::ContentType(content_type))
            }
            PropertyType::ResponseTopic => {
                let topic = PubTopic::decode(ba)?;
                Ok(Self::ResponseTopic(topic))
            }
            PropertyType::CorrelationData => {
                let data = BinaryData::decode(ba)?;
                Ok(Self::CorrelationData(data))
            }
            PropertyType::MaximumQoS => {
                let qos = QoS::decode(ba)?;
                if qos != QoS::AtLeastOnce && qos != QoS::AtMostOnce {
                    return Err(DecodeError::InvalidPropertyValue);
                }
                Ok(Self::MaximumQoS(qos))
            }
            PropertyType::RetainAvailable => {
                let available = BoolData::decode(ba)?;
                Ok(Self::RetainAvailable(available))
            }
            PropertyType::AssignedClientIdentifier => {
                let client_id = StringData::decode(ba)?;
                validate_client_id(client_id.as_ref())?;
                Ok(Self::AssignedClientIdentifier(client_id))
            }
            PropertyType::WildcardSubscriptionAvailable => {
                let available = BoolData::decode(ba)?;
                Ok(Self::WildcardSubscriptionAvailable(available))
            }
            PropertyType::SubscriptionIdentifierAvailable => {
                let available = BoolData::decode(ba)?;
                Ok(Self::SubscriptionIdentifierAvailable(available))
            }
            PropertyType::SharedSubscriptionAvailable => {
                let available = BoolData::decode(ba)?;
                Ok(Self::SharedSubscriptionAvailable(available))
            }
            PropertyType::ServerKeepAlive => {
                let keep_alive = U16Data::decode(ba)?;
                Ok(Self::ServerKeepAlive(keep_alive))
            }
            PropertyType::ResponseInformation => {
                let info = StringData::decode(ba)?;
                Ok(Self::ResponseInformation(info))
            }
            PropertyType::ServerReference => {
                let reference = StringData::decode(ba)?;
                Ok(Self::ServerReference(reference))
            }
            PropertyType::TopicAlias => {
                let alias = U16Data::decode(ba)?;
                Ok(Self::TopicAlias(alias))
            }
            PropertyType::SubscriptionIdentifier => {
                let id = VarInt::decode(ba)?;
                if id.value() == 0 {
                    return Err(DecodeError::InvalidPropertyValue);
                }
                Ok(Self::SubscriptionIdentifier(id))
            }
            _ => unimplemented!(),
        }
    }
}

impl EncodePacket for Property {
    #[allow(clippy::match_same_arms)]
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let property_type_byte = self.property_type() as u8;
        buf.push(property_type_byte);
        let value_bytes = match self {
            Self::AssignedClientIdentifier(client_id) => client_id.encode(buf)?,
            Self::AuthenticationData(data) => data.encode(buf)?,
            Self::AuthenticationMethod(method) => method.encode(buf)?,
            Self::ContentType(content_type) => content_type.encode(buf)?,
            Self::CorrelationData(data) => data.encode(buf)?,
            Self::MaximumPacketSize(max) => max.encode(buf)?,
            Self::MaximumQoS(qos) => qos.encode(buf)?,
            Self::MessageExpiryInterval(interval) => interval.encode(buf)?,
            Self::PayloadFormatIndicator(on) => on.encode(buf)?,
            Self::ReasonString(reason) => reason.encode(buf)?,
            Self::ReceiveMaximum(max) => max.encode(buf)?,
            Self::RequestProblemInformation(on) => on.encode(buf)?,
            Self::RequestResponseInformation(on) => on.encode(buf)?,
            Self::ResponseInformation(info) => info.encode(buf)?,
            Self::ResponseTopic(topic) => topic.encode(buf)?,
            Self::RetainAvailable(available) => available.encode(buf)?,
            Self::ServerKeepAlive(keep_alive) => keep_alive.encode(buf)?,
            Self::ServerReference(reference) => reference.encode(buf)?,
            Self::SessionExpiryInterval(interval) => interval.encode(buf)?,
            Self::SharedSubscriptionAvailable(available) => available.encode(buf)?,
            Self::SubscriptionIdentifier(id) => id.encode(buf)?,
            Self::SubscriptionIdentifierAvailable(available) => available.encode(buf)?,
            Self::TopicAlias(alias) => alias.encode(buf)?,
            Self::TopicAliasMaximum(value) => value.encode(buf)?,
            Self::UserProperty(pair) => pair.encode(buf)?,
            Self::WildcardSubscriptionAvailable(available) => available.encode(buf)?,
            Self::WillDelayInterval(interval) => interval.encode(buf)?,
        };
        Ok(PropertyType::bytes() + value_bytes)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Properties(Vec<Property>);

impl AsRef<[Property]> for Properties {
    fn as_ref(&self) -> &[Property] {
        self.0.as_ref()
    }
}

impl Properties {
    /// Create a new property list with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get byte length of property list.
    ///
    /// # Panics
    /// Raise panic if bytes of properties is larger than 256MB.
    #[must_use]
    pub fn bytes(&self) -> usize {
        let len = VarInt::from(self.len()).unwrap();
        len.bytes() + self.0.iter().map(Property::bytes).sum::<usize>()
    }

    /// Get length of property list.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check whether property list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get a reference to property list.
    #[must_use]
    pub fn props(&self) -> &[Property] {
        &self.0
    }

    /// Clear property list.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Removes the last property from list and returns it, or `None` if it is empty.
    pub fn pop(&mut self) -> Option<Property> {
        self.0.pop()
    }

    /// Push a property to the back of the list.
    ///
    /// # Errors
    ///
    /// Returns error if list is already full.
    pub fn push(&mut self, v: Property) -> Result<(), EncodeError> {
        let mut len = VarInt::from(self.len())?;
        len.add(1)?;
        self.0.push(v);
        Ok(())
    }

    /// Insert a property to speicify `index` of list.
    ///
    /// # Errors
    ///
    /// Returns error if list is already full.
    pub fn insert(&mut self, index: usize, prop: Property) -> Result<(), EncodeError> {
        self.0.insert(index, prop);
        Ok(())
    }

    /// Remove and returns the property from list.
    ///
    /// # Errors
    ///
    /// Returns error if list is already empty.
    pub fn remove(&mut self, index: usize) -> Result<Property, EncodeError> {
        Ok(self.0.remove(index))
    }
}

impl DecodePacket for Properties {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        // If Property Length is not present in packet Variable Header, use default value.
        if ba.remaining_bytes() == 0 {
            return Ok(Self::new());
        }

        let remaining_length = VarInt::decode(ba)?;
        let mut remaining_length = remaining_length.value();
        let mut properties = Vec::new();
        while remaining_length > 0 {
            let property = Property::decode(ba)?;
            remaining_length -= property.bytes();
            properties.push(property);
        }

        Ok(Self(properties))
    }
}

impl EncodePacket for Properties {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let len = VarInt::from(self.len())?;
        let mut bytes_written = len.bytes();
        len.encode(buf)?;
        for property in &self.0 {
            bytes_written += property.encode(buf)?;
        }

        Ok(bytes_written)
    }
}
