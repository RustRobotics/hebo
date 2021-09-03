// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{
    ConnectAckPacket, ConnectPacket, PublishPacket, SubscribeAckPacket, SubscribePacket,
    UnsubscribePacket,
};

use crate::cache_types::{ListenersVectorCache, SystemCache};

pub type ListenerId = u32;
pub type SessionId = u64;

#[derive(Debug, Clone)]
pub enum ListenerToAuthCmd {
    /// listener-id, username, password
    RequestAuth(ListenerId, String, Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum AuthToListenerCmd {
    /// username, access-granted, error-reason
    ResponseAuth(String, bool, String),
}

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
pub enum DispatcherToSystemCmd {}

#[derive(Debug, Clone)]
pub enum SystemToDispatcherCmd {
    Publish(PublishPacket),
}

#[derive(Debug, Clone)]
pub enum DispatcherToCacheCmd {
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
pub enum CacheToDispatcherCmd {
    ListenersCount(usize),
    SessionsCount(usize),
}

#[derive(Debug, Clone)]
pub enum SystemToCacheCmd {
    GetAllCache,
    GetSystemCache,
    GetListenersCache,
}

#[derive(Debug, Clone)]
pub enum CacheToSystemCmd {
    All(SystemCache, ListenersVectorCache),
    System(SystemCache),
    Listeners(ListenersVectorCache),
}
