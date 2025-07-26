// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Bridge app handlers

use super::Dispatcher;
use crate::commands::BridgeToDispatcherCmd;

impl Dispatcher {
    #[allow(clippy::unused_async)]
    pub(super) async fn handle_bridge_cmd(&self, cmd: BridgeToDispatcherCmd) {
        log::info!("cmd: {cmd:?}");
    }
}
