// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Handles commands from listener.

use codec::{ConnectReturnCode, PublishPacket};

use super::{Session, Status};
use crate::commands::ListenerToSessionCmd;
use crate::error::Error;

impl Session {
    pub(super) async fn handle_listener_packet(
        &mut self,
        cmd: ListenerToSessionCmd,
    ) -> Result<(), Error> {
        match cmd {
            ListenerToSessionCmd::ConnectAck(packet) => {
                // Send connect ack first, then update status.
                let return_code = packet.return_code();
                self.send(packet).await?;

                self.status = match return_code {
                    ConnectReturnCode::Accepted => Status::Connected,
                    _ => Status::Disconnected,
                };
                Ok(())
            }
            ListenerToSessionCmd::PublishAck(packet, accepted) => {
                self.on_listener_publish_ack(packet, accepted).await
            }
            ListenerToSessionCmd::Publish(packet) => self.send(packet).await,
            ListenerToSessionCmd::SubscribeAck(packet) => self.send(packet).await,
            ListenerToSessionCmd::Disconnect => self.send_disconnect().await,
        }
    }

    async fn on_listener_publish_ack(
        &mut self,
        _packet: PublishPacket,
        _accepted: bool,
    ) -> Result<(), Error> {
        // TODO(Shaohua): send ack to client.
        Ok(())
    }
}
