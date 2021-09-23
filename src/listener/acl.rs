// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Acl cmd handler.

use codec::{PublishPacket, SubscribeAck, SubscribeAckPacket, SubscribePacket};

use super::Listener;
use crate::commands::{AclToListenerCmd, ListenerToDispatcherCmd, ListenerToSessionCmd};
use crate::error::Error;
use crate::types::{SessionGid, SessionId};

impl Listener {
    pub(super) async fn handle_acl_cmd(&mut self, cmd: AclToListenerCmd) -> Result<(), Error> {
        match cmd {
            AclToListenerCmd::PublishAck(session_id, packet, accepted) => {
                self.on_acl_publish_ack(session_id, packet, accepted).await
            }
            AclToListenerCmd::SubscribeAck(session_id, packet, acks, accepted) => {
                self.on_acl_subscribe_ack(session_id, packet, acks, accepted)
                    .await
            }
        }
    }

    async fn on_acl_publish_ack(
        &mut self,
        session_id: SessionId,
        packet: PublishPacket,
        accepted: bool,
    ) -> Result<(), Error> {
        if let Some(session_sender) = self.session_senders.get(&session_id) {
            let cmd = ListenerToSessionCmd::PublishAck(packet.packet_id(), packet.qos(), accepted);
            if let Err(err) = session_sender.send(cmd).await {
                log::error!(
                    "listener: Failed to send publish ack to session: {:?}, err: {:?}",
                    session_id,
                    err
                );
            }
        } else {
            log::error!(
                "listener: Failed to find session sender with id: {}",
                session_id
            );
        }

        // If ACL passed, send publish packet to dispatcher layer.
        if accepted {
            let cmd = ListenerToDispatcherCmd::Publish(packet.clone());
            self.dispatcher_sender.send(cmd).await?;
        }
        Ok(())
    }

    async fn on_acl_subscribe_ack(
        &mut self,
        session_id: SessionId,
        packet: SubscribePacket,
        acks: Vec<SubscribeAck>,
        accepted: bool,
    ) -> Result<(), Error> {
        // If ACL passed, send publish packet to dispatcher layer.
        if accepted {
            // Can accept part of subscribe packet.
            let id = SessionGid::new(self.id, session_id);
            self.dispatcher_sender
                .send(ListenerToDispatcherCmd::Subscribe(id, packet))
                .await
                .map_err(Into::into)
        } else {
            // All of topic filters are rejected.
            let ack_packet = SubscribeAckPacket::with_vec(packet.packet_id(), acks);
            self.send_session_publish_ack(session_id, ack_packet).await;
            Ok(())
        }
    }
}
