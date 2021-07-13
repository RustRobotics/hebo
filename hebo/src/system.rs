// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::time::{self, Duration};
use tokio::time::interval;

const UPTIME: &str = "$SYS/uptime";

/// Produce $SYS message.
#[derive(Debug)]
pub struct System {
    startup: time::SystemTime,
    uptime: u64,
}

impl System {
    pub fn new() -> Self {
        System {
            startup: time::SystemTime::now(),
            uptime: 0,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        // TODO(Shaohua): Read interval from config.
        let mut timer = interval(Duration::from_secs(3));
        loop {
            timer.tick().await;
            log::info!("tick()");
        }
    }

    pub fn uptime(&self) -> Result<time::Duration, time::SystemTimeError> {
        time::SystemTime::now().duration_since(self.startup)
    }
}
