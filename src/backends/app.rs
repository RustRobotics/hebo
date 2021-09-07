// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{BackendsToDispatcherCmd, DispatcherToBackendsCmd, ListenerId, SessionId};
use crate::error::Error;

#[derive(Debug)]
pub struct BackendsApp {
    dispatcher_sender: Sender<BackendsToDispatcherCmd>,
    dispatcher_receiver: Receiver<DispatcherToBackendsCmd>,
}

impl BackendsApp {
    pub fn new(
        dispatcher_sender: Sender<BackendsToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToBackendsCmd>,
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

    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToBackendsCmd) -> Result<(), Error> {
        log::info!("cmd: {:?}", cmd);
        match cmd {
            DispatcherToBackendsCmd::SessionAdded(listener_id, session_id, client_id) => {
                self.handle_session_added(listener_id, session_id, client_id)
                    .await
            }
            DispatcherToBackendsCmd::SessionRemoved(listener_id, session_id) => {
                self.handle_session_removed(listener_id, session_id).await
            }
        }
    }

    async fn handle_session_added(
        &mut self,
        listener_id: ListenerId,
        session_id: SessionId,
        client_id: String,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn handle_session_removed(
        &mut self,
        listener_id: ListenerId,
        session_id: SessionId,
    ) -> Result<(), Error> {
        Ok(())
    }
}
