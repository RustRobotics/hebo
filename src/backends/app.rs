// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{BackendsToDispatcherCmd, DispatcherToBackendsCmd};
use crate::error::Error;
use crate::types::{ListenerId, SessionId, SessionInfo};

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
            DispatcherToBackendsCmd::SessionAdded(session) => {
                self.handle_session_added(session).await
            }
            DispatcherToBackendsCmd::SessionRemoved(listener_id, session_id) => {
                self.handle_session_removed(listener_id, session_id).await
            }
        }
    }

    async fn handle_session_added(&mut self, session: SessionInfo) -> Result<(), Error> {
        log::info!("session added: {}", session.session_id);
        Ok(())
    }

    async fn handle_session_removed(
        &mut self,
        listener_id: ListenerId,
        session_id: SessionId,
    ) -> Result<(), Error> {
        log::info!("session removed: {}, {}", listener_id, session_id);
        Ok(())
    }
}
