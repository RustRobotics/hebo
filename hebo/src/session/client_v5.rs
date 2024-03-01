// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use codec::{utils::random_client_id, v5, ByteArray, DecodeError, DecodePacket, QoS};

use super::{Session, Status};
use crate::commands::SessionToListenerCmd;
use crate::error::{Error, ErrorKind};

impl Session {
    pub(super) async fn reject_client_id_v5(&mut self) -> Result<(), Error> {
        log::info!("Session::reject_client_id_v5()");
        let ack_packet = v5::ConnectAckPacket::new(false, v5::ReasonCode::ClientIdentifierNotValid);
        self.send(ack_packet).await?;
        self.status = Status::Disconnected;
        Ok(())
    }

    pub(super) async fn on_client_connect_v5(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let mut packet = match v5::ConnectPacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => {
                if matches!(err, DecodeError::InvalidClientId) {
                    self.reject_client_id_v5().await?;
                    // TODO(Shaohua): disconnect socket stream
                } else {
                    log::error!("on_client_connect_v5() Uncaught error: {:?}", err);
                    // Got malformed packet, disconnect client.
                    self.status = Status::Disconnected;
                    // TODO(Shaohua): disconnect socket stream.
                    // TODO(Shaohua): Return reason-code to client.
                }
                return Err(err.into());
            }
        };

        // Check connection status first.
        if self.status != Status::Invalid {
            self.status = Status::Disconnected;
            // TODO(Shaohua): disconnect socket stream
            return Err(Error::new(
                ErrorKind::StatusError,
                "sesion: Invalid status, got a second CONNECT packet!",
            ));
        }

        // TODO(Shaohua): Check client-id rules in V5 spec.
        if packet.client_id().is_empty() {
            if self.config.allow_empty_client_id() {
                let new_client_id = random_client_id();
                // No need to catch errors as client id is always valid.
                let _ret = packet.set_client_id(&new_client_id);
            } else {
                return self.reject_client_id_v5().await;
            }
        }
        self.client_id = packet.client_id().to_string();

        if packet.keep_alive() > 0 {
            self.config.set_keep_alive(packet.keep_alive());
        }

        if !packet.connect_flags().clean_session() && packet.client_id().is_empty() {
            let ack_packet =
                v5::ConnectAckPacket::new(false, v5::ReasonCode::ClientIdentifierNotValid);
            self.send(ack_packet).await?;
            return self.send_disconnect().await;
        }

        self.clean_session = packet.connect_flags().clean_session();
        // TODO(Shaohua): Handle other connection flags.
        // TODO(Shaohua): Check will and will_qos is valid.

        self.process_connect_properties(&packet);

        // TODO(Shaohua): Read auth-method and auth-data in properties.

        // Send the connect packet to listener.
        self.status = Status::Connecting;
        self.sender
            .send(SessionToListenerCmd::ConnectV5(self.id, packet))
            .await
            .map(drop)?;
        Ok(())
    }

    pub(super) async fn on_client_ping_v5(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let _packet = v5::PingRequestPacket::decode(&mut ba)?;

        // Send ping resp packet to client.
        let ping_resp_packet = v5::PingResponsePacket::new();
        self.send(ping_resp_packet).await
    }

    pub(super) async fn on_client_publish_v5(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("Session::on_client_publish_v5()");
        let mut ba = ByteArray::new(buf);
        let packet = v5::PublishPacket::decode(&mut ba)?;

        // Check dup flag for QoS2.
        if packet.qos() == QoS::ExactOnce && packet.dup() {
            // If this packet_id is already handled, send PublishReceivedPacket again.
            if self.pub_recv_packets.contains(&packet.packet_id()) {
                let ack_packet = v5::PublishReceivedPacket::new(packet.packet_id());
                // TODO(Shaohua): Catch errors
                return self.send(ack_packet).await;
            }
        }

        // Send the publish packet to listener.
        self.sender
            .send(SessionToListenerCmd::PublishV5(self.id, packet))
            .await
            .map(drop)?;
        Ok(())
    }

    pub(super) async fn on_client_publish_release_v5(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = match v5::PublishReleasePacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => match err {
                DecodeError::InvalidPacketFlags => {
                    // TODO(Shaohua): Add commentds
                    log::error!(
                        "session: Invalid bit flags for publish release packet, do disconnect!"
                    );
                    return self.send_disconnect().await;
                }
                _ => return Err(err.into()),
            },
        };

        if self.pub_recv_packets.contains(&packet.packet_id()) {
            // Remove packet_id from cache then send complete packet.
            self.pub_recv_packets.remove(&packet.packet_id());
            let ack_packet = v5::PublishCompletePacket::new(packet.packet_id());
            self.send(ack_packet).await
        } else {
            log::error!(
                "session: Failed to remove {} from pub_recv_packets",
                packet.packet_id()
            );
            Ok(())
        }
    }

    pub(super) async fn on_client_subscribe_v5(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = match v5::SubscribePacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => match err {
                DecodeError::InvalidPacketFlags => {
                    // TODO(Shaohua): Add comments
                    log::error!("session: Invalid bit flags for subscribe packet, do disconnect!");
                    return self.send_disconnect().await;
                }
                DecodeError::EmptyTopicFilter => {
                    // TODO(Shaohua): Add comments
                    log::error!("session: Empty topic filter in subscribe packet, do disconnect!");
                    return self.send_disconnect().await;
                }
                DecodeError::InvalidQoS => {
                    // TODO(Shaohua): Add comments
                    log::error!("session: Invalid QoS flag in subscribe packet, do disconnect!");
                    return self.send_disconnect().await;
                }
                _ => {
                    // TODO(Shaohua): Send disconnect when got error.
                    return Err(err.into());
                }
            },
        };

        // Send subscribe packet to listener, which will check ACL.
        let packet_id = packet.packet_id();
        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::SubscribeV5(self.id, packet))
            .await
        {
            // Send subscribe ack (failed) to client.
            log::error!("Failed to send subscribe command to server: {:?}", err);
            let reason = v5::ReasonCode::UnspecifiedError;

            let subscribe_ack_packet = v5::SubscribeAckPacket::new(packet_id, reason);
            self.send(subscribe_ack_packet).await
        } else {
            Ok(())
        }
    }

    pub(super) async fn on_client_unsubscribe_v5(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = match v5::UnsubscribePacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => match err {
                DecodeError::InvalidPacketFlags => {
                    // TODO(Shaohua): Add comments
                    log::error!(
                        "session: Invalid bit flags for unsubscribe packet, do disconnect!"
                    );
                    return self.send_disconnect().await;
                }
                _ => {
                    // TODO(Shaohua): Send disconnect when got error.
                    return Err(err.into());
                }
            },
        };
        let packet_id = packet.packet_id();
        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::UnsubscribeV5(self.id, packet))
            .await
        {
            log::warn!("Failed to send unsubscribe command to server: {:?}", err);
        }

        let unsubscribe_ack_packet =
            v5::UnsubscribeAckPacket::new(packet_id, v5::ReasonCode::Success);
        self.send(unsubscribe_ack_packet).await
    }

    pub(super) async fn on_client_disconnect_v5(&mut self, _: &[u8]) -> Result<(), Error> {
        self.status = Status::Disconnected;
        let cmd = SessionToListenerCmd::DisconnectV5(self.id);
        if let Err(err) = self.sender.send(cmd).await {
            log::warn!("Failed to send disconnect command to server: {:?}", err);
        }
        Ok(())
    }
}
