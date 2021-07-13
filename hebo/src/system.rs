// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{PublishPacket, QoS};
use std::time::{self, Duration};
use tokio::sync::mpsc;
use tokio::time::interval;

use crate::commands::SystemToDispatcherCmd;
use crate::error::Error;

const UPTIME: &str = "$SYS/uptime";

/// Produce $SYS message.
#[derive(Debug)]
pub struct System {
    startup: time::SystemTime,
    uptime: u64,
    interval: u32,
    sender: mpsc::Sender<SystemToDispatcherCmd>,
}

impl System {
    pub fn new(interval: u32, sender: mpsc::Sender<SystemToDispatcherCmd>) -> Self {
        System {
            startup: time::SystemTime::now(),
            uptime: 0,
            interval,
            sender,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        let mut timer = interval(Duration::from_secs(self.interval.into()));
        loop {
            timer.tick().await;
            self.update_time();

            if let Err(err) = self.send_uptime().await {
                log::error!(
                    "Failed to send publish packet from system to dispatcher: {:?}",
                    err
                );
            }
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

    async fn send_uptime(&mut self) -> Result<(), Error> {
        let msg = format!("{}", self.uptime).into_bytes();
        let packet = PublishPacket::new(UPTIME, QoS::AtMostOnce, &msg)?;
        self.sender
            .send(SystemToDispatcherCmd::Publish(packet))
            .await
            .map(drop)?;
        Ok(())
    }
}
