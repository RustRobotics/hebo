// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SessionConfig {
    keep_alive: Duration,
    connect_timeout: Duration,

    maximum_inflight_messages: usize,
    maximum_packet_size: usize,
    maximum_topic_alias: u16,

    allow_empty_client_id: bool,

    out_packet_count: usize,
    last_packet_id: u16,
    session_expiry_interval: Duration,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionConfig {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            keep_alive: Duration::from_secs(60),
            connect_timeout: Duration::from_secs(30),

            maximum_inflight_messages: 10,
            maximum_packet_size: 10,
            maximum_topic_alias: 10,

            allow_empty_client_id: false,

            out_packet_count: 0,
            last_packet_id: 0,
            session_expiry_interval: Duration::from_secs(180),
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn set_keep_alive(&mut self, keep_alive: u16) -> &mut Self {
        let keep_alive = (f64::from(keep_alive) * 1.5).round() as u64;
        self.keep_alive = Duration::from_secs(keep_alive);
        self
    }

    #[inline]
    #[must_use]
    pub const fn keep_alive(&self) -> Duration {
        self.keep_alive
    }

    pub fn set_connect_timeout(&mut self, connect_timeout: u16) -> &mut Self {
        self.connect_timeout = Duration::from_secs(u64::from(connect_timeout));
        self
    }

    #[inline]
    #[must_use]
    pub const fn connect_timeout(&self) -> Duration {
        self.connect_timeout
    }

    pub fn set_maximum_inflight_messages(&mut self, maximum_inflight_messages: u16) -> &mut Self {
        self.maximum_inflight_messages = maximum_inflight_messages as usize;
        self
    }

    #[inline]
    #[must_use]
    pub const fn maximum_inflight_messages(&self) -> usize {
        self.maximum_inflight_messages
    }

    pub fn set_maximum_packet_size(&mut self, maximum_packet_size: u32) -> &mut Self {
        self.maximum_packet_size = maximum_packet_size as usize;
        self
    }

    #[inline]
    #[must_use]
    pub const fn maximum_packet_size(&self) -> usize {
        self.maximum_packet_size
    }

    pub fn set_maximum_topic_alias(&mut self, maximum_topic_alias: u16) -> &mut Self {
        self.maximum_topic_alias = maximum_topic_alias;
        self
    }

    #[inline]
    #[must_use]
    pub const fn maximum_topic_alias(&self) -> u16 {
        self.maximum_topic_alias
    }

    pub fn set_allow_empty_client_id(&mut self, allow_empty_client_id: bool) -> &mut Self {
        self.allow_empty_client_id = allow_empty_client_id;
        self
    }

    #[inline]
    #[must_use]
    pub const fn allow_empty_client_id(&self) -> bool {
        self.allow_empty_client_id
    }

    pub fn out_packet_count_add_one(&mut self) {
        self.out_packet_count += 1;
    }

    #[inline]
    #[must_use]
    pub const fn out_packet_count(&self) -> usize {
        self.out_packet_count
    }

    pub fn renew_last_packet_id(&mut self) -> u16 {
        let last_packet_id = self.last_packet_id;
        self.last_packet_id += 1;
        last_packet_id
    }

    #[inline]
    #[must_use]
    pub const fn last_packet_id(&self) -> u16 {
        self.last_packet_id
    }

    pub fn set_session_expiry_interval(&mut self, session_expiry_interval: u32) -> &mut Self {
        self.session_expiry_interval = Duration::from_secs(u64::from(session_expiry_interval));
        self
    }

    #[inline]
    #[must_use]
    pub const fn session_expiry_interval(&self) -> Duration {
        self.session_expiry_interval
    }
}
