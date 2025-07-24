// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::AclApp;
use crate::commands::ServerContextToAclCmd;

impl AclApp {
    /// Server context handler
    pub(super) async fn handle_server_ctx_cmd(&mut self, cmd: ServerContextToAclCmd) {
        log::info!("cmd: {:?}", cmd);
    }
}
