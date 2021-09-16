// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Dispatcher cmd handlers.

use codec::{PublishPacket, SubscribeAckPacket};

use super::Listener;
use crate::commands::{DispatcherToListenerCmd, ListenerToSessionCmd};
use crate::types::SessionId;

impl Listener {
    pub(super) async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToListenerCmd) {
        match cmd {
            DispatcherToListenerCmd::Publish(packet) => self.on_dispatcher_publish(packet).await,
            DispatcherToListenerCmd::SubscribeAck(session_id, packet) => {
                self.on_dispatcher_subscribe_ack(session_id, packet).await
            }
        }
    }

    async fn on_dispatcher_publish(&mut self, packet: PublishPacket) {
        let cmd = ListenerToSessionCmd::Publish(packet.clone());
        // TODO(Shaohua): Handle errors
        /*
        for (_, session_sender) in self.session_senders.iter_mut() {
            if topic_match(&pipeline.topics, packet.topic()) {
                if let Err(err) = pipeline.sender.send(cmd.clone()).await {
                    log::warn!(
                        "Failed to send publish packet from listener to session: {:?}",
                        err
                    );
                }
            }
        }
        */
    }

    async fn on_dispatcher_subscribe_ack(
        &mut self,
        session_id: SessionId,
        packet: SubscribeAckPacket,
    ) {
        if let Some(session_sender) = self.session_senders.get(&session_id) {
            let cmd = ListenerToSessionCmd::SubscribeAck(packet);
            if let Err(err) = session_sender.send(cmd).await {
                log::warn!(
                    "listener: Failed to send subscribe ack packet to session {}, err: {:?}",
                    session_id,
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
}
