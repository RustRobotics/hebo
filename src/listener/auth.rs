// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::v3::{ConnectAckPacket, ConnectReturnCode};

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
        // If the Server accepts a connection with CleanSession set to 1,
        // the Server MUST set Session Present to 0 in the CONNACK packet
        // in addition to setting a zero return code in the CONNACK packet [MQTT-3.2.2- 1].
        //
        // If the Server accepts a connection with CleanSession set to 0, the value
        // set in Session Present depends on whether the Server already has
        // stored Session state for the supplied client ID. If the Server has stored
        // Session state, it MUST set Session Present to 1 in the CONNACK packet [MQTT-3.2.2-2].
        // If the Server does not have stored Session state, it MUST set Session Present
        // to 0 in the CONNACK packet. This is in addition to setting a zero return code
        // in the CONNACK packet [MQTT-3.2.2-3].
        //
        // If a server sends a CONNACK packet containing a non-zero return code
        // it MUST set Session Present to 0 [MQTT-3.2.2-4].

        // TODO(Shaohua): Check cached session store and update session_present flag.
        let ack_packet = if access_granted {
            ConnectAckPacket::new(false, ConnectReturnCode::Accepted)
        } else {
            ConnectAckPacket::new(false, ConnectReturnCode::Unauthorized)
        };
        let cmd = ListenerToSessionCmd::ConnectAck(ack_packet);

        self.connecting_sessions.remove(&session_id);

        if access_granted {
            self.client_ids.insert(client_id.to_string(), session_id);
        }

        if let Some(session_sender) = self.session_senders.get(&session_id) {
            session_sender.send(cmd).await.map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }
}
