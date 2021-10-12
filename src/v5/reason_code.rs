// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::convert::TryFrom;

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

/// A Reason Code is a one byte unsigned value that indicates the result of an operation.
///
/// Reason Codes less than 0x80 indicate successful completion of an operation.
/// The normal Reason Code for success is 0. Reason Code values of 0x80 or greater indicate failure.
///
/// The CONNACK, PUBACK, PUBREC, PUBREL, PUBCOMP, DISCONNECT and AUTH Control Packets
/// have a single Reason Code as part of the Variable Header. The SUBACK and UNSUBACK packets
/// contain a list of one or more Reason Codes in the Payload.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReasonCode {
    /// - Success: CONNACK, PUBACK, PUBREC, PUBREL, PUBCOMP, UNSUBACK, AUTH
    /// - Normal disconnection: DISCONNECT
    /// - Granted QoS 0: SUBACK
    Success = 0x00,

    /// Granted QoS 1: SUBACK
    GrantedQos1 = 0x01,

    /// Granted QoS 2: SUBACK
    GrantedQos2 = 0x02,

    /// Disconnect with Will Message: DISCONNECT
    DisconnectWithWillMessage = 0x04,

    /// No matching subscribers: PUBACK, PUBREC
    NoMatchingSubscribers = 0x10,

    /// No subscription existed: UNSUBACK
    NoSubscriptionExisted = 0x11,

    /// Continue authentication: AUTH
    ContinueAuthentication = 0x18,

    /// Re-authenticate: AUTH
    ReAuthenticate = 0x19,

    /// Unspecified error: CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
    UnspecifiedError = 0x80,

    /// Malformed Packet: CONNACK, DISCONNECT
    MalformedPacket = 0x81,

    /// Protocol Error: CONNACK, DISCONNECT
    ProtocolError = 0x82,

    /// Implementation specific error: CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
    ImplementationSpecificError = 0x83,

    /// Unsupported Protocol Version: CONNACK
    UnsupportedProtocolVersion = 0x84,

    /// Client Identifier not valid: CONNACK
    ClientIdentifierNotValid = 0x85,

    /// Bad User Name or Password: CONNACK
    BadUserNameOrPassword = 0x86,

    /// Not authorized: CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
    NotAuthorized = 0x87,

    /// Server unavailable: CONNACK
    ServerUnavailable = 0x88,

    /// Server busy: CONNACK, DISCONNECT
    ServerBusy = 0x89,

    /// Banned: CONNACK
    Banned = 0x8a,

    /// Server shutting down: DISCONNECT
    ServerShuttingDown = 0x8b,

    /// Bad authentication method: CONNACK, DISCONNECT
    BadAuthenticationMethod = 0x8c,

    /// Keep Alive timeout: DISCONNECT
    KeepAliveTimeout = 0x8d,

    /// Session taken over: DISCONNECT
    SessionTakenOver = 0x8e,

    /// Topic Filter invalid: SUBACK, UNSUBACK, DISCONNECT
    TopicFilterInvalid = 0x8f,

    /// Topic Name invalid: CONNACK, PUBACK, PUBREC, DISCONNECT
    TopicNameInvalid = 0x90,

    /// Packet Identifier in use: PUBACK, PUBREC, SUBACK, UNSUBACK
    PacketIdentifierInUse = 0x91,

    /// Packet Identifier not found: PUBREL, PUBCOMP
    PacketIdentifierNotFound = 0x92,

    /// Receive Maximum exceeded: DISCONNECT
    ReceiveMaximumExceeded = 0x93,

    /// Topic Alias invalid: DISCONNECT
    TopicAliasInvalid = 0x94,

    /// Packet too large: CONNACK, DISCONNECT
    PacketTooLarge = 0x95,

    /// Message rate too high: DISCONNECT
    MessageRateTooHigh = 0x96,

    /// Quota exceeded: CONNACK, PUBACK, PUBREC, SUBACK, DISCONNECT
    QuotaExceeded = 0x97,

    /// Administrative action: DISCONNECT
    AdministrativeAction = 0x98,

    /// Payload format invalid: CONNACK, PUBACK, PUBREC, DISCONNECT
    PayloadFormatInvalid = 0x99,

    /// Retain not supported: CONNACK, DISCONNECT
    RetainNotSupported = 0x9a,

    /// QoS not supported: CONNACK, DISCONNECT
    QoSNotSupported = 0x9b,

    /// Use another server: CONNACK, DISCONNECT
    UseAnotherServer = 0x9c,

    /// Server moved: CONNACK, DISCONNECT
    ServerMoved = 0x9d,

    /// Shared Subscriptions not supported: SUBACK, DISCONNECT
    SharedSubscriptionNotSupported = 0x9e,

    /// Connection rate exceeded: CONNACK, DISCONNECT
    ConnectionRateExceeded = 0x9f,

    /// Maximum connect time: DISCONNECT
    MaximumConnectTime = 0xa0,

    /// Subscription Identifiers not supported: SUBACK, DISCONNECT
    SubscriptionIdentifiersNotSupported = 0xa1,

    /// Wildcard Subscriptions not supported: SUBACK, DISCONNECT
    WildcardSubscriptionsNotSupported = 0xa2,
}

impl Default for ReasonCode {
    fn default() -> Self {
        Self::Success
    }
}

impl TryFrom<u8> for ReasonCode {
    type Error = DecodeError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(Self::Success),
            0x01 => Ok(Self::GrantedQos1),
            0x02 => Ok(Self::GrantedQos2),
            0x04 => Ok(Self::DisconnectWithWillMessage),
            0x10 => Ok(Self::NoMatchingSubscribers),
            0x11 => Ok(Self::NoSubscriptionExisted),
            0x18 => Ok(Self::ContinueAuthentication),
            0x19 => Ok(Self::ReAuthenticate),
            0x80 => Ok(Self::UnspecifiedError),
            0x81 => Ok(Self::MalformedPacket),
            0x82 => Ok(Self::ProtocolError),
            0x83 => Ok(Self::ImplementationSpecificError),
            0x84 => Ok(Self::UnsupportedProtocolVersion),
            0x85 => Ok(Self::ClientIdentifierNotValid),
            0x86 => Ok(Self::BadUserNameOrPassword),
            0x87 => Ok(Self::NotAuthorized),
            0x88 => Ok(Self::ServerUnavailable),
            0x89 => Ok(Self::ServerBusy),
            0x8a => Ok(Self::Banned),
            0x8b => Ok(Self::ServerShuttingDown),
            0x8c => Ok(Self::BadAuthenticationMethod),
            0x8d => Ok(Self::KeepAliveTimeout),
            0x8e => Ok(Self::SessionTakenOver),
            0x8f => Ok(Self::TopicFilterInvalid),
            0x90 => Ok(Self::TopicNameInvalid),
            0x91 => Ok(Self::PacketIdentifierInUse),
            0x92 => Ok(Self::PacketIdentifierNotFound),
            0x93 => Ok(Self::ReceiveMaximumExceeded),
            0x94 => Ok(Self::TopicAliasInvalid),
            0x95 => Ok(Self::PacketTooLarge),
            0x96 => Ok(Self::MessageRateTooHigh),
            0x97 => Ok(Self::QuotaExceeded),
            0x98 => Ok(Self::AdministrativeAction),
            0x99 => Ok(Self::PayloadFormatInvalid),
            0x9a => Ok(Self::RetainNotSupported),
            0x9b => Ok(Self::QoSNotSupported),
            0x9c => Ok(Self::UseAnotherServer),
            0x9d => Ok(Self::ServerMoved),
            0x9e => Ok(Self::SharedSubscriptionNotSupported),
            0x9f => Ok(Self::ConnectionRateExceeded),
            0xa0 => Ok(Self::MaximumConnectTime),
            0xa1 => Ok(Self::SubscriptionIdentifiersNotSupported),
            0xa2 => Ok(Self::WildcardSubscriptionsNotSupported),
            _ => Err(DecodeError::OtherErrors),
        }
    }
}

impl ReasonCode {
    pub fn bytes(&self) -> usize {
        1
    }
    pub fn const_bytes() -> usize {
        1
    }
}

impl DecodePacket for ReasonCode {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let byte = ba.read_byte()?;
        let flag = Self::try_from(byte)?;
        Ok(flag)
    }
}

impl EncodePacket for ReasonCode {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.push(*self as u8);
        Ok(self.bytes())
    }
}
