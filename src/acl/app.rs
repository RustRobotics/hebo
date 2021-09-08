// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::broadcast;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    AclToListenerCmd, ListenerToAclCmd, ServerContextRequestCmd, ServerContextResponseCmd,
};
use crate::error::Error;
use crate::types::ListenerId;

// TODO(Shaohua): Replace vector with map.
#[derive(Debug)]
pub struct AclApp {
    listener_senders: Vec<(ListenerId, Sender<AclToListenerCmd>)>,
    listener_receiver: Receiver<ListenerToAclCmd>,

    server_ctx_sender: Sender<ServerContextResponseCmd>,
    server_ctx_receiver: broadcast::Receiver<ServerContextRequestCmd>,
}

impl AclApp {
    pub fn new(
        // listeners
        listener_senders: Vec<(ListenerId, Sender<AclToListenerCmd>)>,
        listener_receiver: Receiver<ListenerToAclCmd>,
        // server ctx
        server_ctx_sender: Sender<ServerContextResponseCmd>,
        server_ctx_receiver: broadcast::Receiver<ServerContextRequestCmd>,
    ) -> Self {
        Self {
            listener_senders,
            listener_receiver,

            server_ctx_sender,
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
                Ok(cmd) = self.server_ctx_receiver.recv() => {
                    self.handle_server_ctx_cmd(cmd).await;
                }
            }
        }
    }

    async fn handle_listener_cmd(&mut self, cmd: ListenerToAclCmd) -> Result<(), Error> {
        log::info!("AclApp::handle_listener_cmd(), cmd: {:?}", cmd);
        Ok(())
    }

    /// Server context handler
    async fn handle_server_ctx_cmd(&mut self, cmd: ServerContextRequestCmd) {
        log::info!("cmd: {:?}", cmd);
    }
}