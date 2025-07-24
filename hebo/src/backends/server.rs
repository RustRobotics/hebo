// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Server context handler

use super::BackendsApp;
use crate::commands::ServerContextToBackendsCmd;

impl BackendsApp {
    #[allow(clippy::unused_async)]
    pub(super) async fn handle_server_ctx_cmd(&mut self, cmd: ServerContextToBackendsCmd) {
        log::info!("cmd: {:?}", cmd);
    }
}
