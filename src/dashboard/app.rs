// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! REST API server for dashboard.
//!
//! Web ui part is located in `/dashboard`.

use std::net::SocketAddr;
use tokio::sync::mpsc::{Receiver, Sender};

use super::routes;
use crate::commands::{DashboardToServerContexCmd, ServerContextToDashboardCmd};
use crate::config;
use crate::error::Error;

#[derive(Debug)]
pub struct DashboardApp {
    addr: SocketAddr,

    server_ctx_sender: Sender<DashboardToServerContexCmd>,
}

impl DashboardApp {
    pub fn new(
        config: &config::Dashboard,
        server_ctx_sender: Sender<DashboardToServerContexCmd>,
    ) -> Result<Self, Error> {
        let addr = config.address.parse()?;
        Ok(Self {
            addr,
            server_ctx_sender,
        })
    }

    pub async fn run_loop(&mut self) {
        let sender = self.server_ctx_sender.clone();
        let routes = routes::init(sender);
        warp::serve(routes).run(self.addr).await
    }
}
