// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::v3::ConnectPacket;

use super::AuthApp;
use crate::commands::{AuthToListenerCmd, ListenerToAuthCmd};
use crate::error::{Error, ErrorKind};
use crate::types::SessionGid;

impl AuthApp {
    pub(super) async fn handle_listener_cmd(
        &mut self,
        cmd: ListenerToAuthCmd,
    ) -> Result<(), Error> {
        log::info!("AuthApp::handle_listener_cmd(), cmd: {:?}", cmd);
        match cmd {
            ListenerToAuthCmd::RequestAuth(session_gid, packet) => {
                self.on_listener_request_auth(session_gid, packet).await
            }
        }
    }

    async fn on_listener_request_auth(
        &mut self,
        session_gid: SessionGid,
        packet: ConnectPacket,
    ) -> Result<(), Error> {
        let username = packet.username();
        let password = packet.password();
        let access_granted = if username.is_empty() {
            self.allow_anonymous
        } else if let Some(file_auth) = &self.file_auth {
            file_auth.is_match(username, password)?
        } else {
            false
        };
        if !access_granted {
            log::error!("AuthApp: Check auth failed, {}:{:?}", username, password);
        }
        for (sender_listener_id, sender) in &self.listener_senders {
            if *sender_listener_id == session_gid.listener_id() {
                let cmd = AuthToListenerCmd::ResponseAuth(
                    session_gid.session_id(),
                    access_granted,
                    packet,
                );
                sender.send(cmd).await?;
                return Ok(());
            }
        }

        Err(Error::from_string(
            ErrorKind::ChannelError,
            format!(
                "AuthApp: Failed to find listener_senders with id: {}",
                session_gid.listener_id()
            ),
        ))
    }
}
