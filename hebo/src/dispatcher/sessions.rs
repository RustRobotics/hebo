// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

use crate::session::CachedSession;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct CachedSessions {
    map: HashMap<String, CachedSession>,
}

impl CachedSessions {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn pop(&mut self, client_id: &str) -> Option<CachedSession> {
        self.map.remove(client_id)
    }
}
