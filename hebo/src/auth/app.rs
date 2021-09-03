// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use crate::config::Security;

#[derive(Debug)]
pub struct AuthApp {
    security: Security,
}

impl AuthApp {
    pub fn new(security: Security) -> Self {
        Self { security }
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {}
    }
}
