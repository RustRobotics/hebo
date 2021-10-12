// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Dispatcher cmd handlers.

use codec::v3::{ConnectReturnCode, PublishPacket, SubscribeAckPacket};

use super::Listener;
use crate::commands::{DispatcherToListenerCmd, ListenerToSessionCmd};
use crate::session::CachedSession;
use crate::types::SessionId;

impl Listener {
    pub(super) async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToListenerCmd) {
        match cmd {
            DispatcherToListenerCmd::CheckCachedSessionResp(session_id, cached_session) => {
                self.on_dispatcher_check_cached_session(session_id, cached_session)
                    .await
            }
            DispatcherToListenerCmd::Publish(session_id, packet) => {
                self.on_dispatcher_publish(session_id, packet).await
            }
            DispatcherToListenerCmd::SubscribeAck(session_id, packet) => {
                self.on_dispatcher_subscribe_ack(session_id, packet).await
            }
        }
    }

    async fn on_dispatcher_check_cached_session(
        &mut self,
        session_id: SessionId,
        cached_session: Option<CachedSession>,
    ) {
        if let Some(cached_session) = cached_session {
        } else {
            // No cached session object found, just send auth cmd.
            if let Err(err) = self
                .session_send_connect_ack(session_id, ConnectReturnCode::Accepted)
                .await
            {
                log::warn!(
                    "Failed to send connect ack packet from listener to session: {:?}",
                    err
                );
            }
        }
    }

    async fn on_dispatcher_publish(&mut self, session_id: SessionId, packet: PublishPacket) {
        if let Some(session_sender) = self.session_senders.get(&session_id) {
            let cmd = ListenerToSessionCmd::Publish(packet);
            if let Err(err) = session_sender.send(cmd).await {
                log::warn!(
                    "Failed to send publish packet from listener to session: {:?}",
                    err
                );
            }
        } else {
            log::error!(
                "listener: Failed to find session_sender with id: {}",
                session_id
            );
        }
    }

    async fn on_dispatcher_subscribe_ack(
        &mut self,
        session_id: SessionId,
        packet: SubscribeAckPacket,
    ) {
        self.send_session_publish_ack(session_id, packet).await
    }
}
