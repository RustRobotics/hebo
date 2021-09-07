// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! REST API server for dashboard.
//!
//! Web ui part is located in `/dashboard`.

use std::net::SocketAddr;
use tokio::sync::mpsc::{Receiver, Sender};
use warp::Filter;

use crate::commands::{DashboardToServerContexCmd, ServerContextToDashboardCmd};

#[derive(Debug)]
pub struct DashboardApp {
    addr: SocketAddr,

    // TODO(Shaohua): Replace with oneshot.
    server_ctx_sender: Sender<DashboardToServerContexCmd>,
    server_ctx_receiver: Receiver<ServerContextToDashboardCmd>,
}

impl DashboardApp {
    pub fn new<A: Into<SocketAddr>>(
        addr: A,
        server_ctx_sender: Sender<DashboardToServerContexCmd>,
        server_ctx_receiver: Receiver<ServerContextToDashboardCmd>,
    ) -> Self {
        Self {
            addr: addr.into(),
            server_ctx_sender,
            server_ctx_receiver,
        }
    }

    pub async fn run_loop(&mut self) {
        let routes = warp::any().map(|| "Hello, World!");
        warp::serve(routes).run(self.addr).await
    }
}
