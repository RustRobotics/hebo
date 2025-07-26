// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{DispatcherToGatewayCmd, GatewayToDispatcherCmd, ServerContextToGatewayCmd};

mod dispatcher;
mod server;

#[allow(clippy::module_name_repetitions)]
pub struct GatewayApp {
    #[allow(dead_code)]
    dispatcher_sender: Sender<GatewayToDispatcherCmd>,
    dispatcher_receiver: Receiver<DispatcherToGatewayCmd>,

    server_ctx_receiver: Receiver<ServerContextToGatewayCmd>,
}

impl GatewayApp {
    #[must_use]
    pub const fn new(
        // dispatcher
        dispatcher_sender: Sender<GatewayToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToGatewayCmd>,
        // server ctx
        server_ctx_receiver: Receiver<ServerContextToGatewayCmd>,
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
                        log::error!("Failed to handle dispatcher cmd: {err:?}");
                    }
                }

                Some(cmd) = self.server_ctx_receiver.recv() => {
                    self.handle_server_ctx_cmd(cmd).await;
                }
            }
        }
    }
}
