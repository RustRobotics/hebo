// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{DispatcherToRuleEngineCmd, RuleEngineToDispatcherCmd};
use crate::error::Error;

#[derive(Debug)]
pub struct RuleEngineApp {
    dispatcher_sender: Sender<RuleEngineToDispatcherCmd>,
    dispatcher_receiver: Receiver<DispatcherToRuleEngineCmd>,
}

impl RuleEngineApp {
    pub fn new(
        dispatcher_sender: Sender<RuleEngineToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToRuleEngineCmd>,
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

    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToRuleEngineCmd) -> Result<(), Error> {
        log::info!("cmd: {:?}", cmd);
        Ok(())
    }
}
