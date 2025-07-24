// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Dispatcher cmd handlers.

use codec::{v3, v5, ProtocolLevel};

use super::Listener;
use crate::commands::{DispatcherToListenerCmd, ListenerToSessionCmd};
use crate::error::Error;
use crate::session::CachedSession;
use crate::types::SessionId;

impl Listener {
    pub(super) async fn handle_dispatcher_cmd(
        &mut self,
        cmd: DispatcherToListenerCmd,
    ) -> Result<(), Error> {
        match cmd {
            DispatcherToListenerCmd::CheckCachedSessionResp(
                session_id,
                protocol_level,
                cached_session,
            ) => {
                self.on_dispatcher_check_cached_session(session_id, protocol_level, cached_session)
                    .await
            }
            DispatcherToListenerCmd::Publish(session_id, packet) => {
                self.on_dispatcher_publish(session_id, packet).await
            }
            DispatcherToListenerCmd::PublishV5(session_id, packet) => {
                self.on_dispatcher_publish_v5(session_id, packet).await
            }
            DispatcherToListenerCmd::SubscribeAck(session_id, packet) => {
                self.on_dispatcher_subscribe_ack(session_id, packet).await
            }
            DispatcherToListenerCmd::SubscribeAckV5(session_id, packet) => {
                self.on_dispatcher_subscribe_ack_v5(session_id, packet)
                    .await
            }
        }
    }

    async fn on_dispatcher_check_cached_session(
        &mut self,
        session_id: SessionId,
        protocol_level: ProtocolLevel,
        cached_session: Option<CachedSession>,
    ) -> Result<(), Error> {
        if protocol_level == ProtocolLevel::V5 {
            self.session_send_connect_ack_v5(session_id, v5::ReasonCode::Success, cached_session)
                .await
        } else {
            self.session_send_connect_ack(
                session_id,
                v3::ConnectReturnCode::Accepted,
                cached_session,
            )
            .await
        }
    }

    async fn on_dispatcher_publish(
        &mut self,
        session_id: SessionId,
        packet: v3::PublishPacket,
    ) -> Result<(), Error> {
        if let Some(session_sender) = self.session_senders.get(&session_id) {
            let cmd = ListenerToSessionCmd::Publish(packet);
            session_sender.send(cmd).await.map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }

    async fn on_dispatcher_publish_v5(
        &mut self,
        session_id: SessionId,
        packet: v5::PublishPacket,
    ) -> Result<(), Error> {
        if let Some(session_sender) = self.session_senders.get(&session_id) {
            let cmd = ListenerToSessionCmd::PublishV5(packet);
            session_sender.send(cmd).await.map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }

    async fn on_dispatcher_subscribe_ack(
        &mut self,
        session_id: SessionId,
        packet: v3::SubscribeAckPacket,
    ) -> Result<(), Error> {
        self.session_send_publish_ack(session_id, packet).await
    }

    async fn on_dispatcher_subscribe_ack_v5(
        &mut self,
        session_id: SessionId,
        packet: v5::SubscribeAckPacket,
    ) -> Result<(), Error> {
        self.session_send_publish_ack_v5(session_id, packet).await
    }
}
