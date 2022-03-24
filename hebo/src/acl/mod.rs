// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{AclToListenerCmd, ListenerToAclCmd, ServerContextToAclCmd};
use crate::types::ListenerId;

mod listener;
mod server;

#[derive(Debug)]
pub struct AclApp {
    listener_senders: HashMap<ListenerId, Sender<AclToListenerCmd>>,
    listener_receiver: Receiver<ListenerToAclCmd>,

    server_ctx_receiver: Receiver<ServerContextToAclCmd>,
}

impl AclApp {
    pub fn new(
        // listeners
        listener_senders: Vec<(ListenerId, Sender<AclToListenerCmd>)>,
        listener_receiver: Receiver<ListenerToAclCmd>,
        // server ctx
        server_ctx_receiver: Receiver<ServerContextToAclCmd>,
    ) -> Self {
        Self {
            listener_senders: listener_senders.into_iter().collect(),
            listener_receiver,

            server_ctx_receiver,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.listener_receiver.recv() => {
                    if let Err(err) = self.handle_listener_cmd(cmd).await {
                        log::error!("Failed to handle listener cmd: {:?}", err);
                    }
                },
                Some(cmd) = self.server_ctx_receiver.recv() => {
                    self.handle_server_ctx_cmd(cmd).await;
                }
            }
        }
    }
}
