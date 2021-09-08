// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::convert::Infallible;
use warp::Filter;

use super::metrics;
use super::types::DashboardSender;

pub fn init(sender: DashboardSender) -> dyn Filter {
    let sender_filter = warp::any().map(move || sender.clone());

    let get_metrics_uptime = warp::get()
        .and(warp::path("api"))
        .and(warp::path("v1"))
        .and(warp::path("metrics"))
        .and(warp::path("uptime"))
        .and(warp::path::end())
        .and(sender_filter.clone())
        .and_then(metrics::get_uptime);

    get_metrics_uptime
}
