// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::GatewayApp;
use crate::commands::DispatcherToGatewayCmd;
use crate::error::Error;

impl GatewayApp {
    #[allow(clippy::unused_async)]
    pub(super) async fn handle_dispatcher_cmd(
        &self,
        cmd: DispatcherToGatewayCmd,
    ) -> Result<(), Error> {
        log::info!("cmd: {cmd:?}");
        Ok(())
    }
}
