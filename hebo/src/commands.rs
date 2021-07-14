// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{PublishPacket, SubscribePacket, UnsubscribePacket};

pub type ConnectionId = u64;

#[derive(Clone, Debug)]
pub enum ListenerToSessionCmd {
    Publish(PublishPacket),
}

#[derive(Debug)]
pub enum SessionToListenerCmd {
    Publish(PublishPacket),
    Subscribe(ConnectionId, SubscribePacket),
    Unsubscribe(ConnectionId, UnsubscribePacket),
    Disconnect(ConnectionId),
}

#[derive(Debug)]
pub enum DispatcherToListenerCmd {
    Publish(PublishPacket),
}

#[derive(Debug)]
pub enum ListenerToDispatcherCmd {
    Publish(PublishPacket),
}

#[derive(Debug)]
pub enum DispatcherToSystemCmd {}

#[derive(Debug)]
pub enum SystemToDispatcherCmd {
    Publish(PublishPacket),
}

#[derive(Debug)]
pub enum DispatcherToCacheCmd {
    // listener id
    ListenerAdded(u32, String),
    ListenerRemoved(u32),

    // listener id, count
    SessionAdded(u32, usize),
    SessionRemoved(u32, usize),

    // listener id, count
    SubscriptionsAdded(u32, usize),
    SubscriptionsRemoved(u32, usize),

    // count, bytes
    RetainedMessageAdded(usize, usize),
    RetainedMessageRemoved(usize, usize),

    // count, bytes
    PublishPacketSent(usize, usize),
    PublishPacketReceived(usize, usize),
    PublishPacketDropped(usize, usize),

    // listener id, count, bytes
    PacketReceived(u32, usize, usize),
    PackdetSent(u32, usize, usize),
}

#[derive(Debug)]
pub enum CacheToDispatcherCmd {
    ConneectionsInfo(u32),
}
