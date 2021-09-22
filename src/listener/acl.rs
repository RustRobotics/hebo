// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Acl cmd handler.

use codec::PublishPacket;

use super::Listener;
use crate::commands::{AclToListenerCmd, ListenerToDispatcherCmd};
use crate::error::Error;
use crate::types::SessionId;

impl Listener {
    pub(super) async fn handle_acl_cmd(&mut self, cmd: AclToListenerCmd) -> Result<(), Error> {
        match cmd {
            AclToListenerCmd::PublishAck(session_id, packet, accepted) => {
                self.on_acl_publish_ack(session_id, packet, accepted).await
            }
        }
    }

    async fn on_acl_publish_ack(
        &mut self,
        session_id: SessionId,
        packet: PublishPacket,
        accepted: bool,
    ) -> Result<(), Error> {
        // TODO(Shaohua): Send ack packet to session.

        // If ACL passed, send publish packet to dispatcher layer.
        if accepted {
            let cmd = ListenerToDispatcherCmd::Publish(packet.clone());
            self.dispatcher_sender.send(cmd).await?;
        }
        Ok(())
    }
}
