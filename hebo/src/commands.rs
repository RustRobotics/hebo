// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{PublishPacket, SubscribePacket, UnsubscribePacket};
use std::sync::Arc;

pub type ListenerId = u32;
pub type SessionId = u64;

#[derive(Clone, Debug)]
pub enum ListenerToSessionCmd {
    Publish(PublishPacket),
}

#[derive(Debug)]
pub enum SessionToListenerCmd {
    Publish(PublishPacket),
    Subscribe(SessionId, SubscribePacket),
    Unsubscribe(SessionId, UnsubscribePacket),
    Disconnect(SessionId),
}

#[derive(Debug)]
pub enum DispatcherToListenerCmd {
    Publish(PublishPacket),
}

#[derive(Debug)]
pub enum ListenerToDispatcherCmd {
    Publish(PublishPacket),
    NewSession(ListenerId, SessionId),
}

#[derive(Debug)]
pub enum DispatcherToSystemCmd {}

#[derive(Debug)]
pub enum SystemToDispatcherCmd {
    Publish(PublishPacket),
}

#[derive(Debug)]
pub enum DispatcherToCacheCmd {
    /// listener id, listener address
    ListenerAdded(ListenerId, Arc<String>),
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

#[derive(Debug)]
pub enum CacheToDispatcherCmd {
    ListenersCount(usize),
    SessionsCount(usize),
}
