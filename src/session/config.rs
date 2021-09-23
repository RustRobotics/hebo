// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Default, Clone)]
pub struct SessionConfig {
    keep_alive: u64,
    connect_timeout: u64,
    allow_empty_client_id: bool,
    max_inflight_messages: usize,
}

impl SessionConfig {
    pub fn new(
        keep_alive: u64,
        connect_timeout: u64,
        allow_empty_client_id: bool,
        max_inflight_messages: usize,
    ) -> Self {
        Self {
            keep_alive,
            connect_timeout,
            allow_empty_client_id,
            max_inflight_messages,
        }
    }

    pub fn set_keep_alive(&mut self, keep_alive: u64) {
        self.keep_alive = keep_alive;
    }

    #[inline]
    pub fn keep_alive(&self) -> u64 {
        self.keep_alive
    }

    #[inline]
    pub fn connect_timeout(&self) -> u64 {
        self.connect_timeout
    }

    #[inline]
    pub fn allow_empty_client_id(&self) -> bool {
        self.allow_empty_client_id
    }

    #[inline]
    pub fn max_inflight_messages(&self) -> usize {
        self.max_inflight_messages
    }
}
