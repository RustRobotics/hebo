// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::time::{self, Duration};
use tokio::sync::mpsc;
use tokio::time::interval;

use crate::commands::SystemToDispatcherCmd;

const UPTIME: &str = "$SYS/uptime";

/// Produce $SYS message.
#[derive(Debug)]
pub struct System {
    startup: time::SystemTime,
    uptime: u64,
    sender: mpsc::Sender<SystemToDispatcherCmd>,
}

impl System {
    pub fn new(sender: mpsc::Sender<SystemToDispatcherCmd>) -> Self {
        System {
            startup: time::SystemTime::now(),
            uptime: 0,
            sender,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        // TODO(Shaohua): Read interval from config.
        let mut timer = interval(Duration::from_secs(3));
        loop {
            log::info!("tick()");
            timer.tick().await;
            self.update_time();
            self.send_uptime().await;
        }
    }

    fn update_time(&mut self) {
        match time::SystemTime::now().duration_since(self.startup) {
            Ok(duration) => {
                self.uptime = duration.as_secs();
            }
            Err(err) => {
                log::error!("Failed to update time, got error: {}", err);
            }
        }
    }

    async fn send_uptime(&mut self) {}
}
