// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Server context handler

use super::RuleEngineApp;
use crate::commands::ServerContextToRuleEngineCmd;

impl RuleEngineApp {
    pub(super) async fn handle_server_ctx_cmd(&mut self, cmd: ServerContextToRuleEngineCmd) {
        log::info!("cmd: {:?}", cmd);
    }
}
