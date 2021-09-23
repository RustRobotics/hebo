// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Handles commands from listener.

use codec::{ConnectReturnCode, PacketId, PublishAckPacket, PublishReceivedPacket, QoS};

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
            ListenerToSessionCmd::PublishAck(packet_id, qos, accepted) => {
                self.on_listener_publish_ack(packet_id, qos, accepted).await
            }
            ListenerToSessionCmd::Publish(packet) => self.send(packet).await,
            ListenerToSessionCmd::SubscribeAck(packet) => self.send(packet).await,
            ListenerToSessionCmd::Disconnect => self.send_disconnect().await,
        }
    }

    /// Send ack to client.
    async fn on_listener_publish_ack(
        &mut self,
        packet_id: PacketId,
        qos: QoS,
        accepted: bool,
    ) -> Result<(), Error> {
        // If a Server implementation does not authorize a PUBLISH to be performed by a Client;
        // it has no way of informing that Client. It MUST either make a positive acknowledgement,
        // according to the normal QoS rules, or close the Network Connection [MQTT-3.3.5-2].
        if !accepted {
            return self.send_disconnect().await;
        }

        // Check qos and send publish ack packet to client.
        if qos == QoS::AtLeastOnce {
            let ack_packet = PublishAckPacket::new(packet_id);
            // TODO(Shaohua): Catch errors
            self.send(ack_packet).await?;
        } else if qos == QoS::ExactOnce {
            // Check inflight messages overflow.
            if self.pub_recv_packets.len() > self.config.max_inflight_messages() {
                log::error!("session: Too many unacknowledged qos=2 messages, disconnect client!");
                return self.send_disconnect().await;
            }

            // Send PublishReceived.
            self.pub_recv_packets.insert(packet_id);
            let ack_packet = PublishReceivedPacket::new(packet_id);
            // TODO(Shaohua): Catch errors
            self.send(ack_packet).await?;
        }
        Ok(())
    }
}
