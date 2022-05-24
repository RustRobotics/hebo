// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Handles client packets

use codec::{
    utils::random_client_id, v3, v5, ByteArray, DecodeError, DecodePacket, FixedHeader, PacketType,
    ProtocolLevel, QoS,
};

use super::{Session, Status};
use crate::commands::SessionToListenerCmd;
use crate::error::{Error, ErrorKind};

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

        // TODO(Shaohua): Check packet oversize.

        match fixed_header.packet_type() {
            PacketType::Connect => self.on_client_connect(buf).await,
            PacketType::PingRequest => self.on_client_ping(buf).await,
            PacketType::Publish { .. } => self.on_client_publish(buf).await,
            PacketType::PublishRelease { .. } => self.on_client_publish_release(buf).await,
            PacketType::Subscribe => self.on_client_subscribe(buf).await,
            PacketType::Unsubscribe => self.on_client_unsubscribe(buf).await,
            PacketType::Disconnect => self.on_client_disconnect(buf).await,
            t => {
                log::warn!("Unhandled msg: {:?}", t);
                Ok(())
            }
        }
    }

    async fn reject_client_id(&mut self) -> Result<(), Error> {
        if self.protocol_level == ProtocolLevel::V5 {
            let ack_packet =
                v5::ConnectAckPacket::new(false, v5::ReasonCode::ClientIdentifierNotValid);
            self.send(ack_packet).await?;
        } else {
            // If a server sends a CONNACK packet containing a non-zero return code
            // it MUST set Session Present to 0 [MQTT-3.2.2-4].
            let ack_packet =
                v3::ConnectAckPacket::new(false, v3::ConnectReturnCode::IdentifierRejected);
            self.send(ack_packet).await?;
        }
        self.status = Status::Disconnected;
        Ok(())
    }

    async fn on_client_connect(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let protocol_level = match ProtocolLevel::decode(&mut ba) {
            Ok(protocol_level) => protocol_level,
            Err(err) => match err {
                // From [MQTT-3.1.2-2].
                //
                // The Server MUST respond to the CONNECT Packet with a CONNACK return code
                // 0x01 (unacceptable protocol level) and then disconnect
                // the Client if the Protocol Level is not supported by the Server
                //
                // If a server sends a CONNACK packet containing a non-zero return code
                // it MUST set Session Present to 0 [MQTT-3.2.2-4].
                //
                // If a server sends a CONNACK packet containing a non-zero return code it MUST
                // then close the Network Connection. [MQTT-3.2.2-5]
                DecodeError::InvalidProtocolName | DecodeError::InvalidProtocolLevel => {
                    let ack_packet =
                        v3::ConnectAckPacket::new(false, v3::ConnectReturnCode::UnacceptedProtocol);
                    self.send(ack_packet).await?;
                    self.status = Status::Disconnected;
                    // TODO(Shaohua): Close socket stream by handle.
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
                    // TODO(Shaohua): Close socket stream by handle.
                    return Err(err.into());
                }
            },
        };

        self.protocol_level = protocol_level;
        if protocol_level == ProtocolLevel::V5 {
            self.on_client_connect_v5(buf).await
        } else {
            self.on_client_connect_v3(buf).await
        }
    }

    async fn on_client_connect_v3(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);

        let mut packet = match v3::ConnectPacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => match err {
                DecodeError::InvalidClientId => {
                    self.reject_client_id().await?;
                    // TODO(Shaohua): disconnect socket stream
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
                    // TODO(Shaohua): disconnect socket stream
                    return Err(err.into());
                }
            },
        };

        // Check connection status first.
        //
        // If this client is already connected, send disconnect packet.
        //
        // The Server MUST process a second CONNECT Packet sent from a Client as
        // a protocol violation and disconnect the Client. [MQTT-3.1.0-2]
        //
        // If the Server rejects the CONNECT, it MUST NOT process any data sent by the
        // Client after the CONNECT Packet. [MQTT-3.1.4-5]
        if self.status == Status::Connecting || self.status == Status::Connected {
            self.status = Status::Disconnected;
            // TODO(Shaohua): disconnect socket stream
            return Err(Error::new(
                ErrorKind::StatusError,
                "sesion: Invalid status, got a second CONNECT packet!",
            ));
        }

        // A Server MAY allow a Client to supply a ClientId that has a length of zero bytes,
        // however if it does so the Server MUST treat this as a special case and
        // assign a unique ClientId to that Client. It MUST then process the CONNECT packet
        // as if the Client had provided that unique ClientId [MQTT-3.1.3-6].
        if packet.client_id().is_empty() {
            if self.config.allow_empty_client_id() {
                let new_client_id = random_client_id();
                // No need to catch errors as client id is always valid.
                let _ret = packet.set_client_id(&new_client_id);
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
        if packet.keep_alive() > 0 {
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_sign_loss)]
            let keep_alive = (f64::from(packet.keep_alive()) * 1.5) as u64;
            self.config.set_keep_alive(keep_alive);
        }

        // From [MQTT-3.1.3-8].
        //
        // If the Client supplies a zero-byte ClientId with CleanSession set to 0,
        // the Server MUST respond to the CONNECT Packet with a CONNACK return code
        // 0x02 (Identifier rejected) and then close the Network Connection
        if !packet.connect_flags().clean_session() && packet.client_id().is_empty() {
            let ack_packet =
                v3::ConnectAckPacket::new(false, v3::ConnectReturnCode::IdentifierRejected);
            self.send(ack_packet).await?;
            return self.send_disconnect().await;
        }

        self.clean_session = packet.connect_flags().clean_session();
        // TODO(Shaohua): Handle other connection flags.

        // Send the connect packet to listener.
        self.status = Status::Connecting;
        self.sender
            .send(SessionToListenerCmd::Connect(self.id, packet))
            .await
            .map(drop)?;
        Ok(())
    }

    async fn on_client_connect_v5(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let mut packet = match v5::ConnectPacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => match err {
                DecodeError::InvalidClientId => {
                    self.reject_client_id().await?;
                    // TODO(Shaohua): disconnect socket stream
                    return Err(err.into());
                }
                _ => {
                    // Got malformed packet, disconnect client.
                    self.status = Status::Disconnected;
                    // TODO(Shaohua): disconnect socket stream.
                    // TODO(Shaohua): Return reason-code to client.
                    return Err(err.into());
                }
            },
        };

        // Check connection status first.
        if self.status == Status::Connecting || self.status == Status::Connected {
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
                return self.reject_client_id().await;
            }
        }
        self.client_id = packet.client_id().to_string();

        // Update keep_alive timer.
        if packet.keep_alive() > 0 {
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_sign_loss)]
            let keep_alive = (f64::from(packet.keep_alive()) * 1.5) as u64;
            self.config.set_keep_alive(keep_alive);
        }

        if !packet.connect_flags().clean_session() && packet.client_id().is_empty() {
            let ack_packet =
                v5::ConnectAckPacket::new(false, v5::ReasonCode::ClientIdentifierNotValid);
            self.send(ack_packet).await?;
            return self.send_disconnect().await;
        }

        self.clean_session = packet.connect_flags().clean_session();
        // TODO(Shaohua): Handle other connection flags.

        // Send the connect packet to listener.
        self.status = Status::Connecting;
        self.sender
            .send(SessionToListenerCmd::ConnectV5(self.id, packet))
            .await
            .map(drop)?;
        Ok(())
    }

    async fn on_client_ping(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let _packet = v3::PingRequestPacket::decode(&mut ba)?;

        // Send ping resp packet to client.
        let ping_resp_packet = v3::PingResponsePacket::new();
        self.send(ping_resp_packet).await
    }

    async fn on_client_publish(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("on_client_publish()");
        let mut ba = ByteArray::new(buf);
        let packet = v3::PublishPacket::decode(&mut ba)?;

        // Check dup flag for QoS2.
        if packet.qos() == QoS::ExactOnce && packet.dup() {
            // If this packet_id is already handled, send PublishReceivedPacket again.
            if self.pub_recv_packets.contains(&packet.packet_id()) {
                let ack_packet = v3::PublishReceivedPacket::new(packet.packet_id());
                // TODO(Shaohua): Catch errors
                return self.send(ack_packet).await;
            }
        }

        // Send the publish packet to listener.
        self.sender
            .send(SessionToListenerCmd::Publish(self.id, packet))
            .await
            .map(drop)?;
        Ok(())
    }

    async fn on_client_publish_release(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = match v3::PublishReleasePacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => match err {
                DecodeError::InvalidPacketFlags => {
                    // Bits 3,2,1 and 0 of the fixed header in the PUBREL Control Packet are reserved
                    // and MUST be set to 0,0,1 and 0 respectively. The Server MUST treat
                    // any other value as malformed and close the Network Connection [MQTT-3.6.1-1].
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
            let ack_packet = v3::PublishCompletePacket::new(packet.packet_id());
            self.send(ack_packet).await
        } else {
            log::error!(
                "session: Failed to remove {} from pub_recv_packets",
                packet.packet_id()
            );
            Ok(())
        }
    }

    async fn on_client_subscribe(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = match v3::SubscribePacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => match err {
                DecodeError::InvalidPacketFlags => {
                    // Bits 3,2,1 and 0 of the fixed header of the SUBSCRIBE Control Packet are reserved
                    // and MUST be set to 0,0,1 and 0 respectively. The Server MUST treat
                    // any other value as malformed and close the Network Connection [MQTT-3.8.1-1].
                    log::error!("session: Invalid bit flags for subscribe packet, do disconnect!");
                    return self.send_disconnect().await;
                }
                DecodeError::EmptyTopicFilter => {
                    // The payload of a SUBSCRIBE packet MUST contain at least one Topic Filter / QoS pair.
                    // A SUBSCRIBE packet with no payload is a protocol violation [MQTT-3.8.3-3].
                    //
                    // Unless stated otherwise, if either the Server or Client encounters a protocol violation,
                    // it MUST close the Network Connection on which it received that Control Packet
                    // which caused the protocol violation [MQTT-4.8.0-1].
                    log::error!("session: Empty topic filter in subscribe packet, do disconnect!");
                    return self.send_disconnect().await;
                }
                DecodeError::InvalidQoS => {
                    // The upper 6 bits of the Requested QoS byte are not used in the current version of the protocol.
                    // They are reserved for future use. The Server MUST treat a SUBSCRIBE packet as malformed
                    // and close the Network Connection if any of Reserved bits in the payload are non-zero,
                    // or QoS is not 0,1 or 2 [MQTT-3-8.3-4].
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
            .send(SessionToListenerCmd::Subscribe(self.id, packet))
            .await
        {
            // Send subscribe ack (failed) to client.
            log::error!("Failed to send subscribe command to server: {:?}", err);
            let ack = v3::SubscribeAck::Failed;

            let subscribe_ack_packet = v3::SubscribeAckPacket::new(packet_id, ack);
            self.send(subscribe_ack_packet).await
        } else {
            Ok(())
        }
    }

    async fn on_client_unsubscribe(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let packet = match v3::UnsubscribePacket::decode(&mut ba) {
            Ok(packet) => packet,
            Err(err) => match err {
                DecodeError::InvalidPacketFlags => {
                    // The Server MUST validate that reserved bits are set to zero and disconnect the Client
                    // if they are not zero [MQTT-3.14.1-1].
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
            .send(SessionToListenerCmd::Unsubscribe(self.id, packet))
            .await
        {
            log::warn!("Failed to send unsubscribe command to server: {:?}", err);
        }

        let unsubscribe_ack_packet = v3::UnsubscribeAckPacket::new(packet_id);
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
