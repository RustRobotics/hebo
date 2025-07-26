// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::Session;

#[derive(Debug, Clone)]
pub struct CachedSession {
    client_id: String,
}

impl CachedSession {
    #[must_use]
    pub const fn new(client_id: String) -> Self {
        Self { client_id }
    }

    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
}

impl Session {
    pub(crate) fn load_cached_session(&self, _cached_session: &CachedSession) {
        // Do nothing currently.
        todo!()
    }
}
