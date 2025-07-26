// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Gateway app handler

use super::Dispatcher;
use crate::commands::GatewayToDispatcherCmd;

impl Dispatcher {
    #[allow(clippy::unused_async)]
    pub(super) async fn handle_gateway_cmd(&self, cmd: GatewayToDispatcherCmd) {
        log::info!("cmd: {cmd:?}");
    }
}
