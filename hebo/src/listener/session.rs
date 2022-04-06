// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Session cmd handlers.

use codec::v3::{
    ConnectAckPacket, ConnectPacket, ConnectReturnCode, PublishPacket, SubscribeAckPacket,
    SubscribePacket, UnsubscribePacket,
};

use super::Listener;
use crate::listener::{
    ListenerToAclCmd, ListenerToAuthCmd, ListenerToDispatcherCmd, ListenerToSessionCmd,
    SessionToListenerCmd,
};
use crate::session::CachedSession;
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
        self.connecting_sessions.insert(session_id);

        // Send request to auth app.
        self.auth_sender
            .send(ListenerToAuthCmd::RequestAuth(
                SessionGid::new(self.id, session_id),
                packet,
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
        // Check ACL.
        let cmd = ListenerToAclCmd::Subscribe(SessionGid::new(self.id, session_id), packet);
        self.acl_sender.send(cmd).await.map_err(Into::into)
    }

    async fn on_session_unsubscribe(
        &mut self,
        session_id: SessionId,
        packet: UnsubscribePacket,
    ) -> Result<(), Error> {
        // No need to check ACL.
        // Remove topic from sub tree.
        self.dispatcher_sender
            .send(ListenerToDispatcherCmd::Unsubscribe(
                SessionGid::new(self.id, session_id),
                packet,
            ))
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

    pub(crate) async fn session_send_connect_ack(
        &mut self,
        session_id: SessionId,
        reason: ConnectReturnCode,
        cached_session: Option<CachedSession>,
    ) -> Result<(), Error> {
        let ack_packet = ConnectAckPacket::new(false, reason);
        let cmd = ListenerToSessionCmd::ConnectAck(ack_packet, cached_session);

        if let Some(session_sender) = self.session_senders.get(&session_id) {
            session_sender.send(cmd).await.map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }

    pub(super) async fn session_send_publish_ack(
        &mut self,
        session_id: SessionId,
        packet: SubscribeAckPacket,
    ) -> Result<(), Error> {
        if let Some(session_sender) = self.session_senders.get(&session_id) {
            let cmd = ListenerToSessionCmd::SubscribeAck(packet);
            session_sender.send(cmd).await.map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }
}