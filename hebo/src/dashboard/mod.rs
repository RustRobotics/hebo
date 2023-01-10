// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! REST API server for dashboard.
//!
//! Web ui part is located in `/dashboard`.

use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;
use warp::Filter;

use crate::commands::DashboardToServerContexCmd;
use crate::config;
use crate::error::Error;

mod error_code;
mod metrics;
mod types;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct DashboardApp {
    addr: SocketAddr,

    server_ctx_sender: Sender<DashboardToServerContexCmd>,
}

impl DashboardApp {
    /// Create a new dashboard app.
    ///
    /// # Errors
    ///
    /// Returns error if `config` has invalid socket address.
    pub fn new(
        config: &config::Dashboard,
        server_ctx_sender: Sender<DashboardToServerContexCmd>,
    ) -> Result<Self, Error> {
        let addr = config.address().parse()?;
        Ok(Self {
            addr,
            server_ctx_sender,
        })
    }

    pub async fn run_loop(&mut self) {
        let sender = self.server_ctx_sender.clone();
        let sender_filter = warp::any().map(move || sender.clone());

        let routes = warp::get()
            .and(warp::path("api"))
            .and(warp::path("v1"))
            .and(warp::path("metrics"))
            .and(warp::path("uptime"))
            .and(warp::path::end())
            .and(sender_filter)
            .and_then(metrics::get_uptime);

        warp::serve(routes).run(self.addr).await;
    }
}
