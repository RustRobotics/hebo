// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct ListenerMetrics {
    pub id: u32,
    pub address: String,

    pub sessions: i64,

    pub subscriptions: i64,

    pub retained_messages: i64,
    pub retained_bytes: i64,

    pub messages_sent: i64,
    pub messages_received: i64,

    pub bytes_sent: i64,
    pub bytes_received: i64,

    pub publish_messages_sent: i64,
    pub publish_messages_received: i64,

    pub publish_bytes_sent: i64,
    pub publish_bytes_received: i64,
}

impl ListenerMetrics {
    #[must_use]
    pub fn new(id: u32, address: String) -> Self {
        Self {
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
    pub sessions: i64,
    pub subscriptions: i64,

    pub retained_messages: i64,
    pub retained_bytes: i64,

    pub messages_sent: i64,
    pub messages_received: i64,

    pub bytes_sent: i64,
    pub bytes_received: i64,

    pub publish_messages_dropped: i64,
    pub publish_messages_sent: i64,
    pub publish_messages_received: i64,

    pub publish_bytes_dropped: i64,
    pub publish_bytes_sent: i64,
    pub publish_bytes_received: i64,
}
