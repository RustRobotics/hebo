// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! `RuleEngine` app handler

use super::Dispatcher;
use crate::dispatcher::RuleEngineToDispatcherCmd;

impl Dispatcher {
    #[allow(clippy::unused_async)]
    pub(super) async fn handle_rule_engine_cmd(&self, cmd: RuleEngineToDispatcherCmd) {
        log::info!("cmd: {cmd:?}");
    }
}
