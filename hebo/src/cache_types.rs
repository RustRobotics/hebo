// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct ListenerMetrics {
    pub id: u32,
    pub address: String,

    pub sessions: u64,

    pub subscriptions: u64,

    pub retained_messages: u64,
    pub retained_bytes: u64,

    pub messages_sent: u64,
    pub messages_received: u64,

    pub bytes_sent: u64,
    pub bytes_received: u64,

    pub publish_messages_sent: u64,
    pub publish_messages_received: u64,

    pub publish_bytes_sent: u64,
    pub publish_bytes_received: u64,
}

impl ListenerMetrics {
    pub fn new(id: u32, address: String) -> Self {
        ListenerMetrics {
            id,
            address,
            ..Self::default()
        }
    }
}

pub type ListenersMapMetrics = HashMap<u32, ListenerMetrics>;
pub type ListenersVectorMetrics = Vec<ListenerMetrics>;

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemMetrics {
    pub listener_count: usize,
    pub sessions: u64,
    pub subscriptions: u64,

    pub retained_messages: u64,
    pub retained_bytes: u64,

    pub messages_sent: u64,
    pub messages_received: u64,

    pub bytes_sent: u64,
    pub bytes_received: u64,

    pub publish_messages_dropped: u64,
    pub publish_messages_sent: u64,
    pub publish_messages_received: u64,

    pub publish_bytes_dropped: u64,
    pub publish_bytes_sent: u64,
    pub publish_bytes_received: u64,
}
