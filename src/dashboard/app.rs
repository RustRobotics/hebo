// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! REST API server for dashboard.
//!
//! Web ui part is located in `/dashboard`.

use std::net::SocketAddr;
use warp::Filter;

#[derive(Debug)]
pub struct DashboardApp {
    addr: SocketAddr,
}

impl DashboardApp {
    pub fn new<A: Into<SocketAddr>>(addr: A) -> Self {
        Self { addr: addr.into() }
    }

    pub async fn run_loop(&mut self) {
        let routes = warp::any().map(|| "Hello, World!");
        warp::serve(routes).run(self.addr).await
    }
}
