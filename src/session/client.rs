// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Handles client packets

use codec::{
    utils::random_client_id, ByteArray, ConnectAckPacket, ConnectPacket, ConnectReturnCode,
    DecodeError, DecodePacket, FixedHeader, PacketType, PingRequestPacket, PingResponsePacket,
    PublishCompletePacket, PublishPacket, PublishReleasePacket, SubscribeAck, SubscribeAckPacket,
    SubscribePacket, UnsubscribeAckPacket, UnsubscribePacket,
};

use super::{Session, Status};
use crate::commands::SessionToListenerCmd;
use crate::error::Error;

impl Session {
    pub(super) async fn handle_client_packet(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let fixed_header = match FixedHeader::decode(&mut ba) {
            Ok(fixed_header) => fixed_header,
            Err(err) => {
                // Disconnect the network if Connect Packet is invalid.
                log::error!("session: Invalid packet: {:?}, content: {:?}", err, buf);
                return self.send_disconnect().await;
            }
        };

        // The Keep Alive is a time interval measured in seconds. Expressed as a 16-bit word,
        // it is the maximum time interval that is permitted to elapse between the point
        // at which the Client finishes transmitting one Control Packet and the point
        // it starts sending the next. It is the responsibility of the Client to ensure that
        // the interval between Control Packets being sent does not exceed the Keep Alive value.
        // In the absence of sending any other Control Packets, the Client MUST send
        // a PINGREQ Packet [MQTT-3.1.2-23].
        self.reset_instant();

        match fixed_header.packet_type() {
            PacketType::Connect => self.on_client_connect(&buf).await,
            PacketType::PingRequest => self.on_client_ping(&buf).await,
            PacketType::Publish { .. } => self.on_client_publish(&buf).await,
            PacketType::PublishRelease { .. } => self.on_client_publish_release(&buf).await,
            PacketType::Subscribe => self.on_client_subscribe(&buf).await,
            PacketType::Unsubscribe => self.on_client_unsubscribe(&buf).await,
            PacketType::Disconnect => self.on_client_disconnect(&buf).await,
            t => {
                log::warn!("Unhandled msg: {:?}", t);
                Ok(())
            }
        }
    }

    async fn reject_client_id(&mut self) -> Result<(), Error> {
        // If a server sends a CONNACK packet containing a non-zero return code
        // it MUST set Session Present to 0 [MQTT-3.2.2-4].
        let ack_packet = ConnectAckPacket::new(false, ConnectReturnCode::IdentifierRejected);
        self.send(ack_packet).await?;
        self.send_disconnect().await
    }

    async fn on_client_connect(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let mut packet = match ConnectPacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => match err {
                // From [MQTT-3.1.2-2].
                //
                // The Server MUST respond to the CONNECT Packet with a CONNACK return code
                // 0x01 (unacceptable protocol level) and then disconnect
                // the Client if the Protocol Level is not supported by the Server
                //
                // If a server sends a CONNACK packet containing a non-zero return code
                // it MUST set Session Present to 0 [MQTT-3.2.2-4].
                DecodeError::InvalidProtocolName | DecodeError::InvalidProtocolLevel => {
                    let ack_packet =
                        ConnectAckPacket::new(false, ConnectReturnCode::UnacceptedProtocol);
                    self.send(ack_packet).await?;
                    self.send_disconnect().await?;
                    return Err(err.into());
                }
                DecodeError::InvalidClientId => {
                    self.reject_client_id().await?;
                    return Err(err.into());
                }
                _ => {
                    // Got malformed packet, disconnect client.
                    //
                    // The Server MUST validate that the CONNECT Packet conforms to section 3.1 and close the
                    // Network Connection without sending a CONNACK if it does not conform [MQTT-3.1.4-1].
                    //
                    // We do not send any packets, just disconnect the stream.
                    self.status = Status::Disconnected;
                    return Err(err.into());
                }
            },
        };

        // A Server MAY allow a Client to supply a ClientId that has a length of zero bytes,
        // however if it does so the Server MUST treat this as a special case and
        // assign a unique ClientId to that Client. It MUST then process the CONNECT packet
        // as if the Client had provided that unique ClientId [MQTT-3.1.3-6].
        if packet.client_id().is_empty() {
            if self.config.allow_empty_client_id() {
                if let Ok(new_client_id) = random_client_id() {
                    // No need to catch errors as client id is always valid.
                    let _ = packet.set_client_id(&new_client_id);
                } else {
                    // Almost never happens.
                    return self.reject_client_id().await;
                }
            } else {
                return self.reject_client_id().await;
            }
        }
        self.client_id = packet.client_id().to_string();

        // Update keep_alive timer.
        //
        // If the Keep Alive value is non-zero and the Server does not receive a Control Packet
        // from the Client within one and a half times the Keep Alive time period,
        // it MUST disconnect the Network Connection to the Client as if the network
        // had failed [MQTT-3.1.2-24].
        if packet.keep_alive > 0 {
            self.config
                .set_keep_alive((packet.keep_alive as f64 * 1.5) as u64);
        }

        // From [MQTT-3.1.3-8].
        //
        // If the Client supplies a zero-byte ClientId with CleanSession set to 0,
        // the Server MUST respond to the CONNECT Packet with a CONNACK return code
        // 0x02 (Identifier rejected) and then close the Network Connection
        if !packet.connect_flags().clean_session() && packet.client_id().is_empty() {
            let ack_packet = ConnectAckPacket::new(false, ConnectReturnCode::IdentifierRejected);
            self.send(ack_packet).await?;
            return self.send_disconnect().await;
        }

        self.clean_session = packet.connect_flags().clean_session();
        // TODO(Shaohua): Handle other connection flags.

        // Check connection status first.
        // If this client is already connected, send disconnect packet.
        if self.status == Status::Connected || self.status == Status::Connecting {
            return self.send_disconnect().await;
        }

        // Send the connect packet to listener.
        self.status = Status::Connecting;
        self.sender
            .send(SessionToListenerCmd::Connect(self.id, packet))
            .await
            .map(drop)?;
        Ok(())
    }

    async fn on_client_ping(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let _packet = PingRequestPacket::decode(&mut ba)?;

        // Send ping resp packet to client.
        let ping_resp_packet = PingResponsePacket::new();
        self.send(ping_resp_packet).await
    }

    async fn on_client_publish(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("on_client_publish()");
        let mut ba = ByteArray::new(buf);
        let packet = PublishPacket::decode(&mut ba)?;

        // Send the publish packet to listener.
        self.sender
            .send(SessionToListenerCmd::Publish(self.id, packet))
            .await
            .map(drop)?;
        Ok(())
    }

    async fn on_client_publish_release(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = PublishReleasePacket::decode(&mut ba)?;
        if !self.pub_recv_packets.contains(&packet.packet_id()) {
            log::error!(
                "session: Failed to remove {} from pub_recv_packets",
                packet.packet_id()
            );
        } else {
            let ack_packet = PublishCompletePacket::new(packet.packet_id());
            self.send(ack_packet).await?;
            self.pub_recv_packets.remove(&packet.packet_id());
        }
        Ok(())
    }

    async fn on_client_subscribe(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = SubscribePacket::decode(&mut ba)?;

        // Send subscribe packet to listener, which will check auth.
        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::Subscribe(self.id, packet.clone()))
            .await
        {
            // Send subscribe ack (failed) to client.
            log::error!("Failed to send subscribe command to server: {:?}", err);
            let ack = SubscribeAck::Failed;

            let subscribe_ack_packet = SubscribeAckPacket::new(packet.packet_id(), ack);
            self.send(subscribe_ack_packet).await
        } else {
            Ok(())
        }
    }

    async fn on_client_unsubscribe(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = UnsubscribePacket::decode(&mut ba)?;
        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::Unsubscribe(self.id, packet.clone()))
            .await
        {
            log::warn!("Failed to send unsubscribe command to server: {:?}", err);
        }

        let unsubscribe_ack_packet = UnsubscribeAckPacket::new(packet.packet_id());
        self.send(unsubscribe_ack_packet).await
    }

    /// Handle disconnect request from client.
    async fn on_client_disconnect(&mut self, _buf: &[u8]) -> Result<(), Error> {
        self.status = Status::Disconnected;
        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::Disconnect(self.id))
            .await
        {
            log::warn!("Failed to send disconnect command to server: {:?}", err);
        }
        Ok(())
    }
}
