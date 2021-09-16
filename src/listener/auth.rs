// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{ConnectAckPacket, ConnectReturnCode};

use super::Listener;
use crate::commands::{AuthToListenerCmd, ListenerToSessionCmd};
use crate::error::Error;
use crate::types::SessionId;

impl Listener {
    pub(super) async fn handle_auth_cmd(&mut self, cmd: AuthToListenerCmd) -> Result<(), Error> {
        match cmd {
            AuthToListenerCmd::ResponseAuth(session_id, access_granted) => {
                self.on_auth_response(session_id, access_granted).await
            }
        }
    }

    async fn on_auth_response(
        &mut self,
        session_id: SessionId,
        access_granted: bool,
    ) -> Result<(), Error> {
        let ack_packet = if access_granted {
            ConnectAckPacket::new(true, ConnectReturnCode::Accepted)
        } else {
            ConnectAckPacket::new(false, ConnectReturnCode::Unauthorized)
        };
        let cmd = ListenerToSessionCmd::ConnectAck(ack_packet);

        if access_granted {
            // Add client id to cache.
            if let Some(client_id) = self.session_ids.get(&session_id) {
                self.client_ids.insert(client_id.to_string(), session_id);
            } else {
                log::error!(
                    "listener: Failed to find client id with session: {}",
                    session_id
                );
            }
        }

        if let Some(session_sender) = self.session_senders.get(&session_id) {
            session_sender.send(cmd).await.map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }
}
