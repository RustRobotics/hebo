// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Session cmd handlers.

use codec::{
    ConnectAckPacket, ConnectPacket, ConnectReturnCode, PublishPacket, SubscribePacket,
    UnsubscribePacket,
};

use super::Listener;
use crate::listener::{
    ListenerToAclCmd, ListenerToAuthCmd, ListenerToDispatcherCmd, ListenerToSessionCmd,
    SessionToListenerCmd,
};
use crate::types::{SessionGid, SessionId};
use crate::Error;

impl Listener {
    pub(super) fn next_session_id(&mut self) -> SessionId {
        self.current_session_id += 1;
        self.current_session_id
    }

    pub(super) async fn handle_session_cmd(
        &mut self,
        cmd: SessionToListenerCmd,
    ) -> Result<(), Error> {
        log::info!("Listener::handle_session_cmd()");
        match cmd {
            SessionToListenerCmd::Connect(session_id, packet) => {
                self.on_session_connect(session_id, packet).await
            }
            SessionToListenerCmd::Publish(session_id, packet) => {
                self.on_session_publish(session_id, packet).await
            }
            SessionToListenerCmd::Subscribe(session_id, packet) => {
                self.on_session_subscribe(session_id, packet).await
            }
            SessionToListenerCmd::Unsubscribe(session_id, packet) => {
                self.on_session_unsubscribe(session_id, packet).await
            }
            SessionToListenerCmd::Disconnect(session_id) => {
                self.on_session_disconnect(session_id).await
            }
        }
    }

    #[allow(dead_code)]
    async fn reject_client_id(&mut self, session_id: SessionId) -> Result<(), Error> {
        // If a server sends a CONNACK packet containing a non-zero return code
        // it MUST set Session Present to 0 [MQTT-3.2.2-4].
        let ack_packet = ConnectAckPacket::new(false, ConnectReturnCode::IdentifierRejected);
        let cmd = ListenerToSessionCmd::ConnectAck(ack_packet);
        if let Some(session_sender) = self.session_senders.get(&session_id) {
            session_sender.send(cmd).await.map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }

    async fn on_session_connect(
        &mut self,
        session_id: SessionId,
        packet: ConnectPacket,
    ) -> Result<(), Error> {
        log::info!("Listener::on_session_connect()");

        // If the ClientId represents a Client already connected to the Server then the Server MUST
        // disconnect the existing Client [MQTT-3.1.4-2].
        let old_session_id = self.client_ids.get(packet.client_id());
        if let Some(old_session_id) = old_session_id {
            let old_session_id = *old_session_id;
            if let Err(err) = self.disconnect_session(old_session_id).await {
                log::error!(
                    "Failed to send disconnect cmd to {}, err: {:?}",
                    old_session_id,
                    err
                );
            }
        }

        // TODO(Shaohua): Check duplicated ConnectPacket.
        self.connecting_sessions
            .insert(session_id, packet.connect_flags().clean_session());
        self.session_ids
            .insert(session_id, packet.client_id().to_string());

        // Send request to auth app.
        self.auth_sender
            .send(ListenerToAuthCmd::RequestAuth(
                self.id,
                session_id,
                packet.username().to_string(),
                packet.password().to_vec(),
            ))
            .await
            .map_err(Into::into)
    }

    async fn on_session_disconnect(&mut self, session_id: SessionId) -> Result<(), Error> {
        log::info!("Listener::on_session_disconnect()");
        // Delete session info
        if self.session_senders.remove(&session_id).is_none() {
            log::error!("Failed to remove pipeline with session id: {}", session_id);
        }
        if let Some(client_id) = self.session_ids.remove(&session_id) {
            if self.client_ids.remove(&client_id).is_none() {
                log::error!("Failed to remove client id: {}", client_id);
            }
        } else {
            log::error!("Failed to remove session id: {}", session_id);
        }

        self.dispatcher_sender
            .send(ListenerToDispatcherCmd::SessionRemoved(self.id))
            .await
            .map_err(Into::into)
    }

    async fn on_session_subscribe(
        &mut self,
        session_id: SessionId,
        packet: SubscribePacket,
    ) -> Result<(), Error> {
        log::info!("Listener::on_session_subscribe()");

        // TODO(Shaohua): Check ACL.

        // Send notification to dispatcher.
        let id = SessionGid::new(self.id, session_id);
        self.dispatcher_sender
            .send(ListenerToDispatcherCmd::Subscribe(id, packet))
            .await
            .map_err(Into::into)
    }

    async fn on_session_unsubscribe(
        &mut self,
        _session_id: SessionId,
        _packet: UnsubscribePacket,
    ) -> Result<(), Error> {
        // Remove topic from sub tree.
        // Send subRemoved to dispatcher.
        self.dispatcher_sender
            .send(ListenerToDispatcherCmd::SubscriptionsRemoved(self.id))
            .await
            .map_err(Into::into)
    }

    async fn on_session_publish(
        &mut self,
        session_id: SessionId,
        packet: PublishPacket,
    ) -> Result<(), Error> {
        // Check ACL.
        let cmd = ListenerToAclCmd::Publish(SessionGid::new(self.id, session_id), packet);
        self.acl_sender.send(cmd).await.map_err(Into::into)
    }

    /// Send disconnect cmd to session.
    async fn disconnect_session(&mut self, session_id: SessionId) -> Result<(), Error> {
        let cmd = ListenerToSessionCmd::Disconnect;
        if let Some(session_sender) = self.session_senders.get(&session_id) {
            session_sender.send(cmd).await.map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }
}
