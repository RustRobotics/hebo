// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::time;

const UPTIME: &str = "$SYS/uptime";

#[derive(Debug)]
pub struct SysMessage {
    startup: time::SystemTime,
    connections: usize,
    messages_sent: u64,
    messages_recv: u64,
    messages_queued: u64,
    message_bytes_queued: u64,
}

impl SysMessage {
    pub fn new() -> Self {
        SysMessage {
            startup: time::SystemTime::now(),
            connections: 0,
            messages_sent: 0,
            messages_recv: 0,
            messages_queued: 0,
            message_bytes_queued: 0,
        }
    }

    pub fn uptime(&self) -> Result<time::Duration, time::SystemTimeError> {
        time::SystemTime::now().duration_since(self.startup)
    }
}
