// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use warp::http::StatusCode;

use super::types::DashboardSender;
use crate::commands::DashboardToServerContexCmd;

/// metrics api
pub async fn get_uptime(sender: DashboardSender) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("sender: {:?}", sender);
    if let Err(err) = sender
        .send(DashboardToServerContexCmd::MetrixGetUptime)
        .await
    {
        log::error!("err: {:?}", err);
    }
    Ok(warp::reply::with_status("Uptime", StatusCode::CREATED))
}
