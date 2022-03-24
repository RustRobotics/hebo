// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Handles commands from dispatcher.

use super::BackendsApp;
use crate::commands::DispatcherToBackendsCmd;
use crate::error::Error;
use crate::types::{ListenerId, SessionId, SessionInfo};

impl BackendsApp {
    pub(super) async fn handle_dispatcher_cmd(
        &mut self,
        cmd: DispatcherToBackendsCmd,
    ) -> Result<(), Error> {
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
