// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{
    ConnectAckPacket, ConnectPacket, PublishPacket, SubscribeAckPacket, SubscribePacket,
    UnsubscribePacket,
};
use tokio::sync::oneshot;

use crate::types::{ListenerId, SessionId, SessionInfo};

#[derive(Debug, Clone)]
pub enum ListenerToAuthCmd {
    /// listener-id, session-id, username, password
    RequestAuth(ListenerId, SessionId, String, Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum AuthToListenerCmd {
    /// session-id, access-granted
    ResponseAuth(SessionId, bool),
}

#[derive(Debug, Clone)]
pub enum AclToListenerCmd {}

#[derive(Debug, Clone)]
pub enum ListenerToAclCmd {}

#[derive(Debug, Clone)]
pub enum ListenerToSessionCmd {
    /// Accepted or not.
    ConnectAck(ConnectAckPacket),

    Publish(PublishPacket),

    SubscribeAck(SubscribeAckPacket),
}

#[derive(Debug, Clone)]
pub enum SessionToListenerCmd {
    Connect(SessionId, ConnectPacket),
    Publish(PublishPacket),
    Subscribe(SessionId, SubscribePacket),
    Unsubscribe(SessionId, UnsubscribePacket),
    Disconnect(SessionId),
}

#[derive(Debug, Clone)]
pub enum DispatcherToListenerCmd {
    Publish(PublishPacket),
}

#[derive(Debug, Clone)]
pub enum ListenerToDispatcherCmd {
    Publish(PublishPacket),

    SessionAdded(ListenerId),
    SessionRemoved(ListenerId),

    SubscriptionsAdded(ListenerId),
    SubscriptionsRemoved(ListenerId),
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
    Publish(PublishPacket),
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

#[derive(Debug, Clone)]
pub enum ServerContextRequestCmd {}

#[derive(Debug)]
pub enum ServerContexToMetricsCmd {
    MetricsGetUptime,
}

#[derive(Debug, Clone)]
pub enum ServerContextResponseCmd {}

#[derive(Debug)]
pub enum DashboardToServerContexCmd {
    MetricsGetUptime(oneshot::Sender<u32>),
}

#[derive(Debug, Clone)]
pub enum ServerContextToDashboardCmd {}
