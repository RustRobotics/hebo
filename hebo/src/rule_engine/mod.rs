// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    DispatcherToRuleEngineCmd, RuleEngineToDispatcherCmd, ServerContextToRuleEngineCmd,
};

mod dispatcher;
mod server;

pub struct RuleEngineApp {
    _dispatcher_sender: Sender<RuleEngineToDispatcherCmd>,
    dispatcher_receiver: Receiver<DispatcherToRuleEngineCmd>,

    server_ctx_receiver: Receiver<ServerContextToRuleEngineCmd>,
}

impl RuleEngineApp {
    pub fn new(
        // dispatcher
        _dispatcher_sender: Sender<RuleEngineToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToRuleEngineCmd>,
        // server ctx
        server_ctx_receiver: Receiver<ServerContextToRuleEngineCmd>,
    ) -> Self {
        Self {
            _dispatcher_sender,
            dispatcher_receiver,
            server_ctx_receiver,
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

                Some(cmd) = self.server_ctx_receiver.recv() => {
                    self.handle_server_ctx_cmd(cmd).await;
                }
            }
        }
    }
}
