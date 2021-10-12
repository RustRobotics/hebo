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
    ///
    /// CONNACK: Connection accepted.
    ///
    /// DISCONNECT: Close the connection normally.  Do not send the Will Message.
    /// Sent by client or server.
    ///
    /// UNSUBACK: The subscription is deleted.
    ///
    /// SUBACK: The subscription is accepted and the maximum QoS sent will be QoS 0.
    /// This might be a lower QoS than was requested.
    ///
    /// AUTH: Authentication is successful. Sent by server.
    ///
    /// PUBACK: The message is accepted. Publication of the QoS 1 message proceeds.
    ///
    /// PUBCOMP: Message released.
    Success = 0x00,

    /// Granted QoS 1: SUBACK
    ///
    /// SUBACK: The subscription is accepted and the maximum QoS sent will be QoS 1.
    /// This might be a lower QoS than was requested.
    GrantedQoS1 = 0x01,

    /// Granted QoS 2: SUBACK
    ///
    /// SUBACK: The subscription is accepted and any received QoS will be sent to this subscription.
    GrantedQoS2 = 0x02,

    /// Disconnect with Will Message: DISCONNECT
    ///
    /// DISCONNECT: The Client wishes to disconnect but requires that the Server
    /// also publishes its Will Message. Sent by client.
    DisconnectWithWillMessage = 0x04,

    /// No matching subscribers: PUBACK, PUBREC
    ///
    /// PUBACK: The message is accepted but there are no subscribers. This is sent only by the Server.
    /// If the Server knows that there are no matching subscribers, it MAY use
    /// this Reason Code instead of 0x00 (Success).
    NoMatchingSubscribers = 0x10,

    /// No subscription existed: UNSUBACK
    ///
    /// UNSUBACK: No matching Topic Filter is being used by the Client.
    NoSubscriptionExisted = 0x11,

    /// Continue authentication: AUTH
    ///
    /// AUTH: Continue the authentication with another step. Sent by client or server.
    ContinueAuthentication = 0x18,

    /// Re-authenticate: AUTH
    ///
    /// AUTH: Initiate a re-authentication. Sent by client.
    ReAuthenticate = 0x19,

    /// Unspecified error: CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
    ///
    /// CONNACK: The Server does not wish to reveal the reason for the failure, or
    /// none of the other Reason Codes apply.
    ///
    /// DISCONNECT: The Connection is closed but the sender either does not wish to reveal the reason,
    /// or none of the other Reason Codes apply. Sent by client or server.
    ///
    /// UNSUBACK: The unsubscribe could not be completed and the Server either does not
    /// wish to reveal the reason or none of the other Reason Codes apply.
    ///
    /// SUBACK: The subscription is not accepted and the Server either does not wish to reveal
    /// the reason or none of the other Reason Codes apply.
    ///
    /// PUBACK: The receiver does not accept the publish but either does not want to reveal the reason,
    /// or it does not match one of the other values.
    UnspecifiedError = 0x80,

    /// Malformed Packet: CONNACK, DISCONNECT
    ///
    /// CONNACK: Data within the CONNECT packet could not be correctly parsed.
    ///
    /// DISCONNECT: The received packet does not conform to this specification.
    /// Sent by client or server.
    MalformedPacket = 0x81,

    /// Protocol Error: CONNACK, DISCONNECT
    ///
    /// CONNACK: Data in the CONNECT packet does not conform to this specification.
    ///
    /// DISCONNECT: An unexpected or out of order packet was received. Sent by client or server.
    ProtocolError = 0x82,

    /// Implementation specific error: CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
    ///
    /// CONNACK: The CONNECT is valid but is not accepted by this Server.
    ///
    /// DISCONNECT: The packet received is valid but cannot be processed by this implementation.
    /// Sent by client or server.
    ///
    /// UNSUBACK: The UNSUBSCRIBE is valid but the Server does not accept it.
    ///
    /// SUBACK: The SUBSCRIBE is valid but the Server does not accept it.
    ///
    /// PUBACK: The PUBLISH is valid but the receiver is not willing to accept it.
    ImplementationSpecificError = 0x83,

    /// Unsupported Protocol Version: CONNACK
    ///
    /// CONNACK: The Server does not support the version of the MQTT protocol requested by the Client.
    UnsupportedProtocolVersion = 0x84,

    /// Client Identifier not valid: CONNACK
    ///
    /// CONNACK: The Client Identifier is a valid string but is not allowed by the Server.
    ClientIdentifierNotValid = 0x85,

    /// Bad User Name or Password: CONNACK
    ///
    /// CONNACK: The Server does not accept the User Name or Password specified by the Client
    BadUserNameOrPassword = 0x86,

    /// Not authorized: CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
    ///
    /// CONNACK: The Client is not authorized to connect.
    ///
    /// DISCONNECT: The request is not authorized. Sent by server.
    ///
    /// UNSUBACK: The Client is not authorized to unsubscribe.
    ///
    /// SUBACK: The Client is not authorized to make this subscription.
    ///
    /// PUBACK: The PUBLISH is not authorized.
    NotAuthorized = 0x87,

    /// Server unavailable: CONNACK
    ///
    /// CONNACK: The MQTT Server is not available.
    ServerUnavailable = 0x88,

    /// Server busy: CONNACK, DISCONNECT
    ///
    /// CONNACK: The Server is busy. Try again later.
    ///
    /// DISCONNECT: The Server is busy and cannot continue processing requests from this Client.
    /// Sent by server.
    ServerBusy = 0x89,

    /// Banned: CONNACK
    ///
    /// CONNACK: This Client has been banned by administrative action. Contact the server administrator.
    Banned = 0x8a,

    /// Server shutting down: DISCONNECT
    ///
    /// DISCONNECT: The Server is shutting down. Sent by server.
    ServerShuttingDown = 0x8b,

    /// Bad authentication method: CONNACK, DISCONNECT
    ///
    /// CONNACK: The authentication method is not supported or does not match
    /// the authentication method currently in use.
    BadAuthenticationMethod = 0x8c,

    /// Keep Alive timeout: DISCONNECT
    ///
    /// DISCONNECT: The Connection is closed because no packet has been received for 1.5 times the Keepalive time.
    /// Sent by server.
    KeepAliveTimeout = 0x8d,

    /// Session taken over: DISCONNECT
    ///
    /// DISCONNECT: Another Connection using the same ClientID has connected causing this Connection to be closed.
    /// Sent by server.
    SessionTakenOver = 0x8e,

    /// Topic Filter invalid: SUBACK, UNSUBACK, DISCONNECT
    ///
    /// DISCONNECT: The Topic Filter is correctly formed, but is not accepted by this Sever.
    /// Sent by server.
    ///
    /// UNSUBACK: The Topic Filter is correctly formed but is not allowed for this Client.
    ///
    /// SUBACK: The Topic Filter is correctly formed but is not allowed for this Client.
    TopicFilterInvalid = 0x8f,

    /// Topic Name invalid: CONNACK, PUBACK, PUBREC, DISCONNECT
    ///
    /// CONNACK: The Will Topic Name is not malformed, but is not accepted by this Server.
    ///
    /// DISCONNECT: The Topic Name is correctly formed, but is not accepted by this Client or Server.
    /// Sent by client or server.
    ///
    /// PUBACK: The Topic Name is not malformed, but is not accepted by this Client or Server.
    TopicNameInvalid = 0x90,

    /// Packet Identifier in use: PUBACK, PUBREC, SUBACK, UNSUBACK
    ///
    /// UNSUBACK: The specified Packet Identifier is already in use.
    ///
    /// SUBACK: The specified Packet Identifier is already in use.
    ///
    /// PUBACK: The Packet Identifier is already in use. This might indicate a mismatch
    /// in the Session State between the Client and Server.
    PacketIdentifierInUse = 0x91,

    /// Packet Identifier not found: PUBREL, PUBCOMP
    ///
    /// PUBCOMP: The Packet Identifier is not known.
    /// This is not an error during recovery, but at other times indicates a mismatch
    /// between the Session State on the Client and Server.
    PacketIdentifierNotFound = 0x92,

    /// Receive Maximum exceeded: DISCONNECT
    ///
    /// DISCONNECT: The Client or Server has received more than Receive Maximum publication
    /// for which it has not sent PUBACK or PUBCOMP. Sent by client or server.
    ReceiveMaximumExceeded = 0x93,

    /// Topic Alias invalid: DISCONNECT
    ///
    /// DISCONNECT: The Client or Server has received a PUBLISH packet containing a Topic Alias
    /// which is greater than the Maximum Topic Alias it sent in the CONNECT or CONNACK packet.
    /// Sent by client or server.
    TopicAliasInvalid = 0x94,

    /// Packet too large: CONNACK, DISCONNECT
    ///
    /// CONNACK: The CONNECT packet exceeded the maximum permissible size.
    PacketTooLarge = 0x95,

    /// Message rate too high: DISCONNECT
    MessageRateTooHigh = 0x96,

    /// Quota exceeded: CONNACK, PUBACK, PUBREC, SUBACK, DISCONNECT
    ///
    /// CONNACK: An implementation or administrative imposed limit has been exceeded.
    ///
    /// SUBACK: An implementation or administrative imposed limit has been exceeded.
    ///
    /// PUBACK: An implementation or administrative imposed limit has been exceeded.
    QuotaExceeded = 0x97,

    /// Administrative action: DISCONNECT
    AdministrativeAction = 0x98,

    /// Payload format invalid: CONNACK, PUBACK, PUBREC, DISCONNECT
    ///
    /// CONNACK: The Will Payload does not match the specified Payload Format Indicator.
    ///
    /// PUBACK: The payload format does not match the specified Payload Format Indicator.
    PayloadFormatInvalid = 0x99,

    /// Retain not supported: CONNACK, DISCONNECT
    RetainNotSupported = 0x9a,

    /// QoS not supported: CONNACK, DISCONNECT
    ///
    /// CONNACK: The Server does not support the QoS set in Will QoS.
    QoSNotSupported = 0x9b,

    /// Use another server: CONNACK, DISCONNECT
    ///
    /// CONNACK: The Client should temporarily use another server.
    UseAnotherServer = 0x9c,

    /// Server moved: CONNACK, DISCONNECT
    ///
    /// CONNACK: The Client should permanently use another server.
    ServerMoved = 0x9d,

    /// Shared Subscriptions not supported: SUBACK, DISCONNECT
    ///
    /// SUBACK: The Server does not support Shared Subscriptions for this Client.
    SharedSubscriptionNotSupported = 0x9e,

    /// Connection rate exceeded: CONNACK, DISCONNECT
    ///
    /// CONNACK: The connection rate limit has been exceeded.
    ConnectionRateExceeded = 0x9f,

    /// Maximum connect time: DISCONNECT
    MaximumConnectTime = 0xa0,

    /// Subscription Identifiers not supported: SUBACK, DISCONNECT
    ///
    /// SUBACK: The Server does not support Subscription Identifiers; the subscription is not accepted.
    SubscriptionIdentifiersNotSupported = 0xa1,

    /// Wildcard Subscriptions not supported: SUBACK, DISCONNECT
    ///
    /// SUBACK: The Server does not support Wildcard Subscriptions; the subscription is not accepted.
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
            0x01 => Ok(Self::GrantedQoS1),
            0x02 => Ok(Self::GrantedQoS2),
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
