// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone)]
pub struct CachedSession {
    client_id: String,
}

impl CachedSession {
    pub fn new(client_id: String) -> Self {
        Self { client_id }
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }
}
