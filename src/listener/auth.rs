// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::v3::{ConnectPacket, ConnectReturnCode};

use super::Listener;
use crate::commands::{AuthToListenerCmd, ListenerToDispatcherCmd};
use crate::error::Error;
use crate::types::{SessionGid, SessionId};

impl Listener {
    pub(super) async fn handle_auth_cmd(&mut self, cmd: AuthToListenerCmd) -> Result<(), Error> {
        match cmd {
            AuthToListenerCmd::ResponseAuth(session_id, access_granted, packet) => {
                self.on_auth_response(session_id, access_granted, packet)
                    .await
            }
        }
    }

    async fn on_auth_response(
        &mut self,
        session_id: SessionId,
        access_granted: bool,
        packet: ConnectPacket,
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

        self.connecting_sessions.remove(&session_id);

        // If not granted, reject this session here.
        if !access_granted {
            return self
                .session_send_connect_ack(session_id, ConnectReturnCode::Unauthorized, None)
                .await;
        }

        // Clean session flag is on.
        if packet.connect_flags().clean_session() {
            return self
                .session_send_connect_ack(session_id, ConnectReturnCode::Accepted, None)
                .await;
        }

        self.client_ids
            .insert(packet.client_id().to_string(), session_id);

        // Check cached session store and update session_present flag.
        let cmd = ListenerToDispatcherCmd::CheckCachedSession(
            SessionGid::new(self.id, session_id),
            packet.client_id().to_string(),
        );
        self.dispatcher_sender.send(cmd).await.map_err(Into::into)
    }
}
