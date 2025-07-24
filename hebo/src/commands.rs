// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::{v3, v5, PacketId, ProtocolLevel, QoS};
use tokio::sync::oneshot;

use crate::types::{ListenerId, SessionGid, SessionId, SessionInfo, Uptime};

use crate::session::CachedSession;

#[derive(Debug, Clone)]
pub enum ListenerToAuthCmd {
    /// `(session_gid, connect_packet)` pair.
    RequestAuth(SessionGid, v3::ConnectPacket),
    RequestAuthV5(SessionGid, v5::ConnectPacket),
}

#[derive(Debug, Clone)]
pub enum AuthToListenerCmd {
    /// `(session_id, access_granted, connect_packet)` pair.
    ResponseAuth(SessionId, bool, v3::ConnectPacket),
    ResponseAuthV5(SessionId, bool, v5::ConnectPacket),
}

#[derive(Debug, Clone)]
pub enum AclToListenerCmd {
    /// `(session_id, publish_packet, accepted)` pair.
    PublishAck(SessionId, v3::PublishPacket, bool),
    PublishAckV5(SessionId, v5::PublishPacket, bool),

    SubscribeAck(SessionId, v3::SubscribePacket, Vec<v3::SubscribeAck>, bool),
    SubscribeAckV5(SessionId, v5::SubscribePacket, Vec<v5::ReasonCode>, bool),
}

#[derive(Debug, Clone)]
pub enum ListenerToAclCmd {
    /// Check publish packet.
    Publish(SessionGid, v3::PublishPacket),
    PublishV5(SessionGid, v5::PublishPacket),

    /// Check subscribe packet.
    Subscribe(SessionGid, v3::SubscribePacket),
    SubscribeV5(SessionGid, v5::SubscribePacket),
}

#[derive(Debug, Clone)]
pub enum ListenerToSessionCmd {
    /// Accepted or not.
    ConnectAck(v3::ConnectAckPacket, Option<CachedSession>),
    ConnectAckV5(v5::ConnectAckPacket, Option<CachedSession>),

    /// Response to Publish packet.
    ///
    /// `(packet_id, qos, accepted)` pair.
    PublishAck(PacketId, QoS, bool),
    PublishAckV5(PacketId, QoS, bool),

    Publish(v3::PublishPacket),
    PublishV5(v5::PublishPacket),

    SubscribeAck(v3::SubscribeAckPacket),
    SubscribeAckV5(v5::SubscribeAckPacket),

    /// Disconnect client connection.
    Disconnect,
    DisconnectV5,
}

#[derive(Debug, Clone)]
pub enum SessionToListenerCmd {
    Connect(SessionId, v3::ConnectPacket),
    ConnectV5(SessionId, v5::ConnectPacket),

    Publish(SessionId, v3::PublishPacket),
    PublishV5(SessionId, v5::PublishPacket),

    Subscribe(SessionId, v3::SubscribePacket),
    SubscribeV5(SessionId, v5::SubscribePacket),

    Unsubscribe(SessionId, v3::UnsubscribePacket),
    UnsubscribeV5(SessionId, v5::UnsubscribePacket),

    Disconnect(SessionId),
    DisconnectV5(SessionId),
}

#[derive(Debug, Clone)]
pub enum DispatcherToListenerCmd {
    CheckCachedSessionResp(SessionId, ProtocolLevel, Option<CachedSession>),

    Publish(SessionId, v3::PublishPacket),
    PublishV5(SessionId, v5::PublishPacket),

    SubscribeAck(SessionId, v3::SubscribeAckPacket),
    SubscribeAckV5(SessionId, v5::SubscribeAckPacket),
}

#[derive(Debug, Clone)]
pub enum ListenerToDispatcherCmd {
    // `(session_gid, client_id, protocol_level)` pair.
    CheckCachedSession(SessionGid, String, ProtocolLevel),

    Publish(v3::PublishPacket),
    PublishV5(v5::PublishPacket),

    Subscribe(SessionGid, v3::SubscribePacket),
    SubscribeV5(SessionGid, v5::SubscribePacket),

    Unsubscribe(SessionGid, v3::UnsubscribePacket),
    UnsubscribeV5(SessionGid, v5::UnsubscribePacket),

    SessionAdded(ListenerId),
    SessionRemoved(ListenerId),
}

#[derive(Debug, Clone)]
pub enum DispatcherToMetricsCmd {
    /// listener id, listener address
    ListenerAdded(ListenerId, String),
    /// listener id
    ListenerRemoved(ListenerId),

    /// listener id, count
    SessionAdded(ListenerId, usize),
    /// listener id, count
    SessionRemoved(ListenerId, usize),

    /// listener id, count
    SubscriptionsAdded(ListenerId, usize),
    /// listener id, count
    SubscriptionsRemoved(ListenerId, usize),

    /// listener id, count, bytes
    RetainedMessageAdded(ListenerId, usize, usize),
    /// listener id, count, bytes
    RetainedMessageRemoved(ListenerId, usize, usize),

    /// listener id, count, bytes
    PublishPacketSent(ListenerId, usize, usize),
    /// listener id, count, bytes
    PublishPacketReceived(ListenerId, usize, usize),
    /// count, bytes
    PublishPacketDropped(usize, usize),

    /// listener id, count, bytes
    PacketSent(ListenerId, usize, usize),
    /// listener id, count, bytes
    PacketReceived(ListenerId, usize, usize),
}

#[derive(Debug, Clone)]
pub enum MetricsToDispatcherCmd {
    Publish(v3::PublishPacket),
    PublishV5(v5::PublishPacket),
}

#[derive(Debug, Clone)]
pub enum DispatcherToBackendsCmd {
    /// session info
    SessionAdded(SessionInfo),

    /// listener id, session id
    SessionRemoved(ListenerId, SessionId),
}

#[derive(Debug, Clone)]
pub enum BackendsToDispatcherCmd {}

#[derive(Debug, Clone)]
pub enum DispatcherToBridgeCmd {}

#[derive(Debug, Clone)]
pub enum BridgeToDispatcherCmd {}

#[derive(Debug, Clone)]
pub enum DispatcherToGatewayCmd {}

#[derive(Debug, Clone)]
pub enum GatewayToDispatcherCmd {}

#[derive(Debug, Clone)]
pub enum DispatcherToRuleEngineCmd {}

#[derive(Debug, Clone)]
pub enum RuleEngineToDispatcherCmd {}

// Server context

#[derive(Debug)]
pub enum ServerContextToAclCmd {}

#[derive(Debug)]
pub enum ServerContextToAuthCmd {}

#[derive(Debug)]
pub enum ServerContextToBackendsCmd {}

#[derive(Debug)]
pub enum ServerContextToBridgeCmd {}

#[derive(Debug)]
pub enum ServerContextToGatewayCmd {}

#[derive(Debug)]
pub enum ServerContextToMetricsCmd {
    MetricsGetUptime(oneshot::Sender<Uptime>),
}

#[derive(Debug)]
pub enum ServerContextToRuleEngineCmd {}

#[derive(Debug)]
pub enum DashboardToServerContexCmd {
    MetricsGetUptime(oneshot::Sender<Uptime>),
}
