// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{PublishPacket, QoS};
use std::time::{self, Duration};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

use crate::commands::{CacheToSystemCmd, SystemToCacheCmd, SystemToDispatcherCmd};
use crate::error::Error;

const UPTIME: &str = "$SYS/uptime";

/// Produce $SYS message.
#[derive(Debug)]
pub struct System {
    startup: time::SystemTime,
    uptime: u64,
    interval: u32,
    dispatcher_sender: Sender<SystemToDispatcherCmd>,

    cache_sender: Sender<SystemToCacheCmd>,
    cache_receiver: Receiver<CacheToSystemCmd>,
}

impl System {
    pub fn new(
        interval: u32,
        dispatcher_sender: Sender<SystemToDispatcherCmd>,
        cache_sender: Sender<SystemToCacheCmd>,
        cache_receiver: Receiver<CacheToSystemCmd>,
    ) -> Self {
        System {
            startup: time::SystemTime::now(),
            uptime: 0,
            interval,
            dispatcher_sender,

            cache_sender,
            cache_receiver,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        let mut timer = interval(Duration::from_secs(self.interval.into()));
        loop {
            tokio::select! {
                Some(cmd) = self.cache_receiver.recv() => {
                    self.handle_cache_cmd(cmd).await;
                },
                _ = timer.tick() => {
                    self.handle_timeout().await;
                }
            }
        }
    }

    async fn handle_cache_cmd(&mut self, cmd: CacheToSystemCmd) {
        match cmd {
            CacheToSystemCmd::All(system_cache, listeners_cache) => {
                log::info!("system cache: {:?}", system_cache);
                log::info!("listeners cache: {:?}", listeners_cache);
            }
            CacheToSystemCmd::System(system_cache) => {
                log::info!("system cache: {:?}", system_cache);
            }
            CacheToSystemCmd::Listeners(listeners_cache) => {
                log::info!("listeners cache: {:?}", listeners_cache);
            }
        }
    }

    async fn handle_timeout(&mut self) {
        self.update_time();

        if let Err(err) = self.cache_sender.send(SystemToCacheCmd::GetAllCache).await {
            log::error!("Failed to send get all cache cmd: {:?}", err);
        }

        if let Err(err) = self.send_uptime().await {
            log::error!(
                "Failed to send publish packet from system to dispatcher: {:?}",
                err
            );
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
        self.dispatcher_sender
            .send(SystemToDispatcherCmd::Publish(packet))
            .await
            .map(drop)?;
        Ok(())
    }
}
