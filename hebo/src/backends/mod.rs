// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    BackendsToDispatcherCmd, DispatcherToBackendsCmd, ServerContextToBackendsCmd,
};

mod dispatcher;
pub mod memory;
mod server;

#[allow(dead_code)]
#[allow(clippy::module_name_repetitions)]
pub struct BackendsApp {
    dispatcher_sender: Sender<BackendsToDispatcherCmd>,
    dispatcher_receiver: Receiver<DispatcherToBackendsCmd>,

    server_ctx_receiver: Receiver<ServerContextToBackendsCmd>,
}

impl BackendsApp {
    #[must_use]
    pub const fn new(
        // dispatcher
        dispatcher_sender: Sender<BackendsToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToBackendsCmd>,
        // server ctx
        server_ctx_receiver: Receiver<ServerContextToBackendsCmd>,
    ) -> Self {
        Self {
            dispatcher_sender,
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
