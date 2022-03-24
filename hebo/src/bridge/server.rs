// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use super::BridgeApp;
use crate::commands::ServerContextToBridgeCmd;

impl BridgeApp {
    /// Server context handler
    pub(super) async fn handle_server_ctx_cmd(&mut self, cmd: ServerContextToBridgeCmd) {
        log::info!("cmd: {:?}", cmd);
    }
}
