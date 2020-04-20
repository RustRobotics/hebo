// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::publish_packet::PublishPacket;
use codec::subscribe_packet::SubscribePacket;

pub type ConnectionId = u64;

#[derive(Clone, Debug)]
pub enum ServerCommand {
    Publish(PublishPacket),
}

#[derive(Debug)]
pub enum ConnectionCommand {
    Publish(PublishPacket),
    Subscribe(ConnectionId, SubscribePacket),
    Unsubscribe,
}
