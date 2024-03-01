// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Handles commands from listener.

use codec::{v3, v5, PacketId, QoS};

use super::{Session, Status};
use crate::commands::ListenerToSessionCmd;
use crate::error::Error;
use crate::session::CachedSession;

impl Session {
    pub(super) async fn handle_listener_cmd(
        &mut self,
        cmd: ListenerToSessionCmd,
    ) -> Result<(), Error> {
        match cmd {
            ListenerToSessionCmd::ConnectAck(packet, cached_session) => {
                self.on_listener_connect_ack(packet, cached_session).await
            }
            ListenerToSessionCmd::ConnectAckV5(packet, cached_session) => {
                self.on_listener_connect_ack_v5(packet, cached_session)
                    .await
            }
            ListenerToSessionCmd::PublishAck(packet_id, qos, accepted) => {
                self.on_listener_publish_ack(packet_id, qos, accepted).await
            }
            ListenerToSessionCmd::PublishAckV5(packet_id, qos, accepted) => {
                self.on_listener_publish_ack_v5(packet_id, qos, accepted)
                    .await
            }
            ListenerToSessionCmd::Publish(packet) => self.on_listener_publish(packet).await,
            ListenerToSessionCmd::PublishV5(packet) => self.on_listener_publish_v5(packet).await,
            ListenerToSessionCmd::SubscribeAck(packet) => {
                self.on_listener_subscribe_ack(packet).await
            }
            ListenerToSessionCmd::SubscribeAckV5(packet) => {
                self.on_listener_subscribe_ack_v5(packet).await
            }
            ListenerToSessionCmd::Disconnect => self.on_listener_disconnect().await,
            ListenerToSessionCmd::DisconnectV5 => self.on_listener_disconnect().await,
        }
    }

    async fn on_listener_connect_ack(
        &mut self,
        packet: v3::ConnectAckPacket,
        cached_session: Option<CachedSession>,
    ) -> Result<(), Error> {
        // Send connect ack first, then update status.
        let return_code = packet.return_code();
        self.send(packet).await?;

        self.status = match return_code {
            v3::ConnectReturnCode::Accepted => Status::Connected,
            _ => Status::Disconnected,
        };

        if let Some(cached_session) = cached_session {
            self.load_cached_session(&cached_session);
        }

        Ok(())
    }

    async fn on_listener_connect_ack_v5(
        &mut self,
        packet: v5::ConnectAckPacket,
        cached_session: Option<CachedSession>,
    ) -> Result<(), Error> {
        // Send connect ack first, then update status.
        let reason_code = packet.reason_code();
        self.send(packet).await?;

        self.status = match reason_code {
            v5::ReasonCode::Success => Status::Connected,
            _ => Status::Disconnected,
        };

        if let Some(cached_session) = cached_session {
            self.load_cached_session(&cached_session);
        }

        Ok(())
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
            let ack_packet = v3::PublishAckPacket::new(packet_id);
            // TODO(Shaohua): Catch errors
            self.send(ack_packet).await?;
        } else if qos == QoS::ExactOnce {
            // Check inflight messages overflow.
            if self.pub_recv_packets.len() > self.config.maximum_inflight_messages() {
                log::error!("session: Too many unacknowledged qos=2 messages, disconnect client!");
                return self.send_disconnect().await;
            }

            // Send PublishReceived.
            self.pub_recv_packets.insert(packet_id);
            let ack_packet = v3::PublishReceivedPacket::new(packet_id);
            // TODO(Shaohua): Catch errors
            self.send(ack_packet).await?;
        }
        Ok(())
    }

    /// Send ack to client.
    async fn on_listener_publish_ack_v5(
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
            let ack_packet = v5::PublishAckPacket::new(packet_id);
            // TODO(Shaohua): Catch errors
            self.send(ack_packet).await?;
        } else if qos == QoS::ExactOnce {
            // Check inflight messages overflow.
            if self.pub_recv_packets.len() > self.config.maximum_inflight_messages() {
                log::error!("session: Too many unacknowledged qos=2 messages, disconnect client!");
                return self.send_disconnect().await;
            }

            // Send PublishReceived.
            self.pub_recv_packets.insert(packet_id);
            let ack_packet = v5::PublishReceivedPacket::new(packet_id);
            // TODO(Shaohua): Catch errors
            self.send(ack_packet).await?;
        }
        Ok(())
    }

    async fn on_listener_publish(&mut self, packet: v3::PublishPacket) -> Result<(), Error> {
        self.send(packet).await
    }

    async fn on_listener_publish_v5(&mut self, packet: v5::PublishPacket) -> Result<(), Error> {
        self.send(packet).await
    }

    async fn on_listener_subscribe_ack(
        &mut self,
        packet: v3::SubscribeAckPacket,
    ) -> Result<(), Error> {
        // When the Server receives a SUBSCRIBE Packet from a Client, the Server MUST respond with a
        // SUBACK Packet [MQTT-3.8.4-1]. The SUBACK Packet MUST have the same Packet Identifier as the
        // SUBSCRIBE Packet that it is acknowledging [MQTT-3.8.4-2].
        self.send(packet).await
    }

    async fn on_listener_subscribe_ack_v5(
        &mut self,
        packet: v5::SubscribeAckPacket,
    ) -> Result<(), Error> {
        // TODO(Shaohua): Add comments
        self.send(packet).await
    }

    async fn on_listener_disconnect(&mut self) -> Result<(), Error> {
        self.send_disconnect().await
    }
}
