// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// A Reason Code is a one byte unsigned value that indicates the result of an operation.
///
/// Reason Codes less than 0x80 indicate successful completion of an operation.
/// The normal Reason Code for success is 0. Reason Code values of 0x80 or greater indicate failure.
///
/// The CONNACK, PUBACK, PUBREC, PUBREL, PUBCOMP, DISCONNECT and AUTH Control Packets
/// have a single Reason Code as part of the Variable Header. The SUBACK and UNSUBACK packets
/// contain a list of one or more Reason Codes in the Payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReasonCode {
    /// - Success: CONNACK, PUBACK, PUBREC, PUBREL, PUBCOMP, UNSUBACK, AUTH
    /// - Normal disconnection: DISCONNECT
    /// - Granted QoS 0: SUBACK
    Success,

    /// Granted QoS 1: SUBACK
    GrantedQos1,

    /// Granted QoS 2: SUBACK
    GrantedQos2,

    /// Disconnect with Will Message: DISCONNECT
    DisconnectWithWillMessage,

    /// No matching subscribers: PUBACK, PUBREC
    NoMatchingSubscribers,

    /// No subscription existed: UNSUBACK
    NoSubscriptionExisted,

    /// Continue authentication: AUTH
    ContinueAuthentication,

    /// Re-authenticate: AUTH
    ReAuthenticate,

    /// Unspecified error: CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
    UnspecifiedError,

    /// Malformed Packet: CONNACK, DISCONNECT
    MalformedPacket,

    /// Protocol Error: CONNACK, DISCONNECT
    ProtocolError,

    /// Implementation specific error: CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
    ImplementationSpecificError,

    /// Unsupported Protocol Version: CONNACK
    UnsupportedProtocolVersion,

    /// Client Identifier not valid: CONNACK
    ClientIdentifierNotValid,

    /// Bad User Name or Password: CONNACK
    BadUserNameOrPassword,

    /// Not authorized: CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
    NotAuthorized,

    /// Server unavailable: CONNACK
    ServerUnavailable,

    /// Server busy: CONNACK, DISCONNECT
    ServerBusy,

    /// Banned: CONNACK
    Banned,

    /// Server shutting down: DISCONNECT
    ServerShuttingDown,

    /// Bad authentication method: CONNACK, DISCONNECT
    BadAuthenticationMethod,

    /// Keep Alive timeout: DISCONNECT
    KeepAliveTimeout,

    /// Session taken over: DISCONNECT
    SessionTakenOver,

    /// Topic Filter invalid: SUBACK, UNSUBACK, DISCONNECT
    TopicFilterInvalid,

    /// Topic Name invalid: CONNACK, PUBACK, PUBREC, DISCONNECT
    TopicNameInvalid,

    /// Packet Identifier in use: PUBACK, PUBREC, SUBACK, UNSUBACK
    PacketIdentifierInUse,

    /// Packet Identifier not found: PUBREL, PUBCOMP
    PacketIdentifierNotFound,

    /// Receive Maximum exceeded: DISCONNECT
    ReceiveMaximumExceeded,

    /// Topic Alias invalid: DISCONNECT
    TopicAliasInvalid,

    /// Packet too large: CONNACK, DISCONNECT
    PacketTooLarge,

    /// Message rate too high: DISCONNECT
    MessageRateTooHigh,

    /// Quota exceeded: CONNACK, PUBACK, PUBREC, SUBACK, DISCONNECT
    QuotaExceeded,

    /// Administrative action: DISCONNECT
    AdministrativeAction,

    /// Payload format invalid: CONNACK, PUBACK, PUBREC, DISCONNECT
    PayloadFormatInvalid,

    /// Retain not supported: CONNACK, DISCONNECT
    RetainNotSupported,

    /// QoS not supported: CONNACK, DISCONNECT
    QosNotSupported,

    /// Use another server: CONNACK, DISCONNECT
    UseAnotherServer,

    /// Server moved: CONNACK, DISCONNECT
    ServerMoved,

    /// Shared Subscriptions not supported: SUBACK, DISCONNECT
    SharedSubscriptionNotSupported,

    /// Connection rate exceeded: CONNACK, DISCONNECT
    ConnectionRateExceeded,

    /// Maximum connect time: DISCONNECT
    MaximumConnectTime,

    /// Subscription Identifiers not supported: SUBACK, DISCONNECT
    SubscriptionIdentifiersNotSupported,

    /// Wildcard Subscriptions not supported: SUBACK, DISCONNECT
    WildcardSubscriptionsNotSupported,
}

impl Into<u8> for ReasonCode {
    fn into(self) -> u8 {
        match self {
            Self::Success => 0x00,
            Self::GrantedQos1 => 0x01,
            Self::GrantedQos2 => 0x02,
            Self::DisconnectWithWillMessage => 0x04,
            Self::NoMatchingSubscribers => 0x10,
            Self::NoSubscriptionExisted => 0x11,
            Self::ContinueAuthentication => 0x18,
            Self::ReAuthenticate => 0x19,

            Self::UnspecifiedError => 0x80,
            Self::MalformedPacket => 0x81,
            Self::ProtocolError => 0x82,
            Self::ImplementationSpecificError => 0x83,
            Self::UnsupportedProtocolVersion => 0x84,
            Self::ClientIdentifierNotValid => 0x85,
            Self::BadUserNameOrPassword => 0x86,
            Self::NotAuthorized => 0x87,
            Self::ServerUnavailable => 0x88,
            Self::ServerBusy => 0x89,
            Self::Banned => 0x8a,
            Self::ServerShuttingDown => 0x8b,
            Self::BadAuthenticationMethod => 0x8c,
            Self::KeepAliveTimeout => 0x8d,
            Self::SessionTakenOver => 0x8e,
            Self::TopicFilterInvalid => 0x8f,
            Self::TopicNameInvalid => 0x90,
            Self::PacketIdentifierInUse => 0x91,
            Self::PacketIdentifierNotFound => 0x92,
            Self::ReceiveMaximumExceeded => 0x93,
            Self::TopicAliasInvalid => 0x94,
            Self::PacketTooLarge => 0x95,
            Self::MessageRateTooHigh => 0x96,
            Self::QuotaExceeded => 0x97,
            Self::AdministrativeAction => 0x98,
            Self::PayloadFormatInvalid => 0x99,
            Self::RetainNotSupported => 0x9a,
            Self::QosNotSupported => 0x9b,
            Self::UseAnotherServer => 0x9c,
            Self::ServerMoved => 0x9d,
            Self::SharedSubscriptionNotSupported => 0x9e,
            Self::ConnectionRateExceeded => 0x9f,
            Self::MaximumConnectTime => 0xa0,
            Self::SubscriptionIdentifiersNotSupported => 0xa1,
            Self::WildcardSubscriptionsNotSupported => 0xa2,
        }
    }
}
