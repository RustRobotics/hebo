// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::oneshot;
use warp::http::StatusCode;

use super::types::DashboardSender;
use crate::commands::DashboardToServerContexCmd;

/// metrics api
pub async fn get_uptime(sender: DashboardSender) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("sender: {:?}", sender);
    let (resp_tx, resp_rx) = oneshot::channel();
    if let Err(err) = sender
        .send(DashboardToServerContexCmd::MetricsGetUptime(resp_tx))
        .await
    {
        log::error!("Failed to send cmd to server ctx, err: {:?}", err);
    } else {
        let ret = resp_rx.await;
        log::info!("ret: {:?}", ret);
    }
    Ok(warp::reply::with_status("Uptime", StatusCode::CREATED))
}
