// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{DispatcherToGatewayCmd, GatewayToDispatcherCmd};
use crate::error::Error;

#[derive(Debug)]
pub struct GatewayApp {
    dispatcher_sender: Sender<GatewayToDispatcherCmd>,
    dispatcher_receiver: Receiver<DispatcherToGatewayCmd>,
}

impl GatewayApp {
    pub fn new(
        dispatcher_sender: Sender<GatewayToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToGatewayCmd>,
    ) -> Self {
        Self {
            dispatcher_sender,
            dispatcher_receiver,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.dispatcher_receiver.recv() => {
                    if let Err(err) = self.handle_dispatcher_cmd(cmd).await {
                        log::error!("Failed to handle dispatcher cmd: {:?}", err);
                    }
                }
            }
        }
    }

    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToGatewayCmd) -> Result<(), Error> {
        log::info!("cmd: {:?}", cmd);
        Ok(())
    }
}
