// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{
    v3::{DisconnectPacket, Packet, PacketType},
    EncodePacket, PacketId,
};
use std::collections::HashSet;
use std::time::Instant;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{ListenerToSessionCmd, SessionToListenerCmd};
use crate::error::{Error, ErrorKind};
use crate::stream::Stream;
use crate::types::SessionId;

mod cache;
mod client;
mod config;
mod listener;

pub use cache::CachedSession;
pub use config::SessionConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Invalid,
    Connecting,
    Connected,
    Disconnecting,
    Disconnected,
}

/// ConnectionContext represents a client connection.
///
/// All the status of this client is maintained in this struct.
// TODO(Shaohua): Handle Will Message
#[derive(Debug)]
pub struct Session {
    id: SessionId,
    config: SessionConfig,
    stream: Stream,

    status: Status,
    client_id: String,
    // TODO(Shaohua): Add session flag
    instant: Instant,
    clean_session: bool,

    pub_recv_packets: HashSet<PacketId>,

    sender: Sender<SessionToListenerCmd>,
    receiver: Receiver<ListenerToSessionCmd>,
}

impl Session {
    pub fn new(
        id: SessionId,
        config: SessionConfig,
        stream: Stream,
        sender: Sender<SessionToListenerCmd>,
        receiver: Receiver<ListenerToSessionCmd>,
    ) -> Session {
        Session {
            id,
            config,
            stream,

            status: Status::Invalid,
            client_id: String::new(),
            instant: Instant::now(),
            clean_session: true,

            pub_recv_packets: HashSet::new(),

            sender,
            receiver,
        }
    }

    pub async fn run_loop(mut self) {
        // TODO(Shaohua): Set buffer cap based on settings
        let mut buf = Vec::with_capacity(1024);

        let connect_timeout = Instant::now();

        loop {
            // If the Server does not receive a CONNECT Packet within a reasonable amount of time after the
            // Network Connection is established, the Server SHOULD close the connection.
            if self.status == Status::Invalid
                && self.config.connect_timeout() > 0
                && connect_timeout.elapsed().as_secs() > self.config.connect_timeout()
            {
                break;
            }

            if self.status == Status::Disconnected {
                break;
            }

            tokio::select! {
                Ok(n_recv) = self.stream.read_buf(&mut buf) => {
                    log::info!("n_recv: {}", n_recv);
                    if n_recv > 0 {
                        if let Err(err) = self.handle_client_packet(&buf).await {
                            log::error!("handle_client_packet() failed: {:?}", err);
                            break;
                        }
                        buf.clear();

                    } else {
                        log::info!("session: Empty packet received, disconnect client, {}", self.id);
                        if let Err(err) = self.send_disconnect().await {
                            log::error!("session: Failed to send disconnect packet: {:?}", err);
                        }
                        break;
                    }
                }
                Some(cmd) = self.receiver.recv() => {
                    if let Err(err) = self.handle_listener_packet(cmd).await {
                        log::error!("Failed to handle server packet: {:?}", err);
                    }
                },
            }

            // From [MQTT-3.1.2-24]
            //
            // If the Keep Alive value is non-zero and the Server does not receive a Control Packet
            // from the Client within one and a half times the Keep Alive time period,
            // it MUST disconnect the Network Connection to the Client as if the network had
            // failed.
            //
            // A Keep Alive value of zero (0) has the effect of turning off the keep alive mechanism.
            // This means that, in this case, the Server is not required to disconnect the Client
            // on the grounds of inactivity.
            //
            // Note that a Server is permitted to disconnect a Client that it determines to be inactive
            // or non-responsive at any time, regardless of the Keep Alive value provided by that Client.
            if self.config.keep_alive() > 0
                && self.instant.elapsed().as_secs() > self.config.keep_alive()
            {
                log::warn!("sessoin: keep_alive time reached, disconnect client!");
                if let Err(err) = self.send_disconnect().await {
                    log::error!("session: Failed to send disconnect packet: {:?}", err);
                }
                break;
            }
        }

        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::Disconnect(self.id))
            .await
        {
            log::error!(
                "Failed to send disconnect cmd to server, id: {}, err: {:?}",
                self.id,
                err
            );
        }
    }

    /// Reset instant if packet is send to or receive from client.
    fn reset_instant(&mut self) {
        self.instant = Instant::now();
    }

    async fn send<P: EncodePacket + Packet>(&mut self, packet: P) -> Result<(), Error> {
        // The CONNACK Packet is the packet sent by the Server in response to a CONNECT Packet
        // received from a Client. The first packet sent from the Server to the Client MUST be
        // a CONNACK Packet [MQTT-3.2.0-1].
        if self.status == Status::Connecting && packet.packet_type() != PacketType::ConnectAck {
            log::error!(
                "ConnectAck is not the first packet to send: {:?}",
                packet.packet_type()
            );
        }

        // After sending a DISCONNECT Packet the Client:
        // - MUST close the Network Connection [MQTT-3.14.4-1].
        // - MUST NOT send any more Control Packets on that Network Connection [MQTT-3.14.4-2].
        if self.status == Status::Disconnected {
            return Err(Error::from_string(
                ErrorKind::SendError,
                format!(
                    "session: Cannot send packet when stream has been disconnected: {:?}",
                    packet.packet_type()
                ),
            ));
        }

        let mut buf = Vec::new();
        packet.encode(&mut buf)?;
        self.stream.write(&buf).await.map(drop)?;
        self.reset_instant();
        Ok(())
    }

    /// Send disconnect packet to client and update status.
    async fn send_disconnect(&mut self) -> Result<(), Error> {
        self.status = Status::Disconnecting;
        let packet = DisconnectPacket::new();
        if let Err(err) = self.send(packet).await.map(drop) {
            log::error!(
                "session: Failed to send disconnect packet, {}, err: {:?}",
                self.id,
                err
            );
            return Err(err);
        }
        self.status = Status::Disconnected;
        Ok(())
    }
}