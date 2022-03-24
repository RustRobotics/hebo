// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! RuleEngine app handler

use super::Dispatcher;
use crate::dispatcher::RuleEngineToDispatcherCmd;

impl Dispatcher {
    pub(super) async fn handle_rule_engine_cmd(&mut self, cmd: RuleEngineToDispatcherCmd) {
        log::info!("cmd: {:?}", cmd);
    }
}
