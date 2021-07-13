// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Key-value store.
#[derive(Debug, Default)]
pub struct Cache {
    pub sys_message: SysMessageCache,
}

#[derive(Debug, Default)]
pub struct SysMessageCache {
    connections: usize,
    messages_sent: u64,
    messages_recv: u64,
    messages_queued: u64,
    message_bytes_queued: u64,
}
