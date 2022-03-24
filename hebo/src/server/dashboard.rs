// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! handles dashboard commands.

use tokio::sync::oneshot;

use super::ServerContext;
use crate::commands::{DashboardToServerContexCmd, ServerContextToMetricsCmd};
use crate::error::{Error, ErrorKind};
use crate::types::Uptime;

impl ServerContext {
    pub(crate) async fn handle_dashboard_cmd(
        &mut self,
        cmd: DashboardToServerContexCmd,
    ) -> Result<(), Error> {
        match cmd {
            DashboardToServerContexCmd::MetricsGetUptime(resp_tx) => {
                self.handle_metrics_uptime(resp_tx).await
            }
        }
    }

    async fn handle_metrics_uptime(
        &mut self,
        resp_tx: oneshot::Sender<Uptime>,
    ) -> Result<(), Error> {
        let (resp2_tx, resp2_rx) = oneshot::channel();

        self.metrics_sender
            .send(ServerContextToMetricsCmd::MetricsGetUptime(resp2_tx))
            .await?;
        let ret = resp2_rx.await?;
        resp_tx.send(ret).map_err(|_| {
            Error::from_string(
                ErrorKind::ChannelError,
                format!("Failed to send metrics uptime to dashboard"),
            )
        })
    }
}
