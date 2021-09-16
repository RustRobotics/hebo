// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Session cmd handlers.

use codec::{
    ConnectAckPacket, ConnectPacket, ConnectReturnCode, PublishPacket, SubscribeAck,
    SubscribeAckPacket, SubscribePacket, UnsubscribePacket,
};

use super::Listener;
use crate::listener::{
    ListenerToAuthCmd, ListenerToDispatcherCmd, ListenerToSessionCmd, SessionToListenerCmd,
};
use crate::types::SessionId;
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
            SessionToListenerCmd::Publish(packet) => self.on_session_publish(packet).await,
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
        // If client id already exists, notify session to send disconnect packet.
        if self.client_ids.get(packet.client_id()).is_some() {
            let ack_packet = ConnectAckPacket::new(false, ConnectReturnCode::IdentifierRejected);
            let cmd = ListenerToSessionCmd::ConnectAck(ack_packet);
            if let Some(session_sender) = self.session_senders.get(&session_id) {
                return session_sender.send(cmd).await.map_err(Into::into);
            } else {
                return Err(Error::session_error(session_id));
            }
        }

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

        // TODO(Shaohua): Check acl.

        // TODO(Shaohua): Send notify to dispatcher.

        Ok(())
    }

    async fn on_session_unsubscribe(
        &mut self,
        session_id: SessionId,
        packet: UnsubscribePacket,
    ) -> Result<(), Error> {
        // Remove topic from sub tree.
        /*
        for (_, pipeline) in self.pipelines.iter_mut() {
            if pipeline.session_id == session_id {
                pipeline
                    .topics
                    .retain(|ref topic| !packet.topics().any(|t| t == topic.topic().topic()));
            }
            break;
        }
        */

        // Send subRemoved to dispatcher.
        self.dispatcher_sender
            .send(ListenerToDispatcherCmd::SubscriptionsRemoved(self.id))
            .await
            .map_err(Into::into)
    }

    async fn on_session_publish(&mut self, packet: PublishPacket) -> Result<(), Error> {
        let cmd = ListenerToDispatcherCmd::Publish(packet.clone());
        self.dispatcher_sender.send(cmd).await.map_err(Into::into)
    }

    /// Send subscribe ack to session.
    async fn send_subscribe_ack(
        &mut self,
        session_id: SessionId,
        packet: SubscribePacket,
    ) -> Result<(), Error> {
        if let Some(session_sender) = self.session_senders.get_mut(&session_id) {
            let ack_vec = packet
                .topics()
                .iter()
                .map(|topic| SubscribeAck::QoS(topic.qos()))
                .collect();

            let ack_packet = SubscribeAckPacket::with_vec(ack_vec, packet.packet_id());
            session_sender
                .send(ListenerToSessionCmd::SubscribeAck(ack_packet))
                .await
                .map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }
}
