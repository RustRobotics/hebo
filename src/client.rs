// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::{
    ByteArray, ConnectAckPacket, ConnectPacket, ConnectReturnCode, DecodePacket, DisconnectPacket,
    EncodePacket, FixedHeader, PacketId, PacketType, PingRequestPacket, PublishAckPacket,
    PublishPacket, QoS, SubscribeAckPacket, SubscribePacket, UnsubscribeAckPacket,
    UnsubscribePacket,
};
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use tokio::time::interval;

use crate::connect_options::*;
use crate::error::{Error, ErrorKind};
use crate::stream::Stream;

#[derive(Debug, Hash, PartialEq)]
enum ConnectStatus {
    Connecting,
    Connected,
    ConnectFailed,
    Disconnecting,
    Disconnected,
}

type FutureConnectCb = dyn Fn(&mut Client) -> dyn Future<Output = ()>;

pub struct Client {
    connect_options: ConnectOptions,
    stream: Stream,
    status: ConnectStatus,
    topics: HashMap<String, PacketId>,
    packet_id: PacketId,
    subscribing_packets: HashMap<PacketId, SubscribePacket>,
    unsubscribing_packets: HashMap<PacketId, UnsubscribePacket>,
    publishing_qos1_packets: HashMap<PacketId, PublishPacket>,
    publishing_qos2_packets: HashMap<PacketId, PublishPacket>,
    connect_cb: Option<Box<FutureConnectCb>>,
}

impl Client {
    pub fn new(connect_options: ConnectOptions) -> Client {
        Client {
            connect_options,
            stream: Stream::None,
            status: ConnectStatus::Disconnected,
            topics: HashMap::new(),
            packet_id: 1,
            subscribing_packets: HashMap::new(),
            unsubscribing_packets: HashMap::new(),
            publishing_qos1_packets: HashMap::new(),
            publishing_qos2_packets: HashMap::new(),
            connect_cb: None,
        }
    }

    pub fn set_connect_callback(&mut self, callback: Box<FutureConnectCb>) {
        self.connect_cb = Some(callback);
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        if self.status == ConnectStatus::Connecting {
            return Err(Error::new(
                ErrorKind::InvalidConnectStatus,
                "In connecting ..",
            ));
        }
        if self.status == ConnectStatus::Connected {
            return Err(Error::new(
                ErrorKind::InvalidConnectStatus,
                "Already connected",
            ));
        }

        self.stream = Stream::connect(self.connect_options.connect_type()).await?;
        let conn_packet = ConnectPacket::new(self.connect_options.client_id());
        log::info!("send conn packet");
        self.send(conn_packet).await
    }

    pub async fn run_loop(&mut self) -> ! {
        log::info!("client.start()");

        let mut buf: Vec<u8> = Vec::with_capacity(1024);
        log::info!("reader loop");
        // FIXME(Shaohua): Fix panic when keep_alive is 0
        let mut timer = interval(*self.connect_options.keep_alive());

        loop {
            tokio::select! {
                Ok(n_recv) = self.stream.read_buf(&mut buf) => {
                    if n_recv > 0 {
                        if let Err(err) = self.handle_session_packet(&mut buf).await {
                            log::error!("err: {:?}", err);
                        }
                        buf.clear();
                    }
                }
                _ = timer.tick() => {
                    log::info!("tick()");
                    if let Err(err) = self.ping().await {
                        log::error!("Ping failed: {:?}", err);
                    }
                },
            }
        }
    }

    async fn handle_session_packet(&mut self, buf: &mut Vec<u8>) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let fixed_header = FixedHeader::decode(&mut ba)?;
        match fixed_header.packet_type {
            PacketType::ConnectAck => self.connect_ack(&buf).await,
            PacketType::Publish { .. } => self.on_message(&buf).await,
            PacketType::PublishAck => self.publish_ack(&buf),
            PacketType::SubscribeAck => self.subscribe_ack(&buf),
            PacketType::UnsubscribeAck => self.unsubscribe_ack(&buf),
            PacketType::PingResponse => self.on_ping_resp().await,
            t => {
                log::info!("Unhandled msg: {:?}", t);
                Ok(())
            }
        }
    }

    async fn send<P: EncodePacket>(&mut self, packet: P) -> Result<(), Error> {
        let mut buf = Vec::new();
        packet.encode(&mut buf)?;
        self.stream
            .write(&buf)
            .await
            .map(drop)
            .map_err(|err| err.into())
    }

    pub async fn publish(&mut self, topic: &str, qos: QoS, data: &[u8]) -> Result<(), Error> {
        let mut packet = PublishPacket::new(topic, qos, data)?;
        match qos {
            QoS::AtLeastOnce => {
                let packet_id = self.next_packet_id();
                packet.set_packet_id(packet_id);
                // TODO(Shaohua): Tuning memory usage.
                self.publishing_qos1_packets
                    .insert(packet_id, packet.clone());
            }
            QoS::ExactOnce => {
                let packet_id = self.next_packet_id();
                packet.set_packet_id(packet_id);
                self.publishing_qos2_packets
                    .insert(packet_id, packet.clone());
            }
            _ => (),
        }
        self.send(packet).await
    }

    pub async fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Error> {
        log::info!("subscribe to: {}", topic);
        let packet_id = self.next_packet_id();
        self.topics.insert(topic.to_string(), packet_id);
        let packet = SubscribePacket::new(topic, qos, packet_id)?;
        self.subscribing_packets.insert(packet_id, packet.clone());
        self.send(packet).await
    }

    pub async fn unsubscribe(&mut self, topic: &str) -> Result<(), Error> {
        log::info!("unsubscribe to: {:?}", topic);
        let packet_id = self.next_packet_id();
        let packet = UnsubscribePacket::new(topic, packet_id);
        self.unsubscribing_packets.insert(packet_id, packet.clone());
        self.send(packet).await
    }

    pub async fn disconnect(&mut self) -> Result<(), Error> {
        if self.status == ConnectStatus::Connected {
            self.status = ConnectStatus::Disconnecting;
            let packet = DisconnectPacket::new();
            self.send(packet).await?;
        }
        self.on_disconnect();
        Ok(())
    }

    async fn on_connect(&mut self) -> Result<(), Error> {
        log::info!("on_connect()");
        Ok(())
        //if let Some(ref cb) = self.connect_cb {
        //(*cb)(self).await;
        //}
    }

    async fn ping(&mut self) -> Result<(), Error> {
        log::info!("ping()");
        if self.status == ConnectStatus::Connected {
            log::info!("Send ping packet");
            let packet = PingRequestPacket::new();
            self.send(packet).await
        } else {
            // TODO(Shaohua): Return Error
            Ok(())
        }
    }

    fn on_disconnect(&mut self) {
        self.status = ConnectStatus::Disconnected;
    }

    async fn on_message(&self, buf: &[u8]) -> Result<(), Error> {
        log::info!("on_message()");
        let mut ba = ByteArray::new(buf);
        let packet = PublishPacket::decode(&mut ba)?;
        log::info!("packet: {:?}", packet);
        //if let Some(cb) = &self.on_message_cb {
        //    cb(self, &packet);
        //}
        Ok(())
    }

    async fn on_ping_resp(&self) -> Result<(), Error> {
        log::info!("on ping resp");
        // TODO(Shaohua): Reset reconnect timer.
        Ok(())
    }

    async fn connect_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("connect_ack()");
        let mut ba = ByteArray::new(buf);
        let packet = ConnectAckPacket::decode(&mut ba)?;
        match packet.return_code() {
            ConnectReturnCode::Accepted => {
                self.status = ConnectStatus::Connected;
                self.on_connect().await?;
            }
            _ => {
                log::warn!("Failed to connect to server, {:?}", packet.return_code());
                self.status = ConnectStatus::ConnectFailed;
            }
        }
        Ok(())
    }

    fn publish_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("publish_ack()");
        let mut ba = ByteArray::new(buf);
        let packet = PublishAckPacket::decode(&mut ba)?;
        let packet_id = packet.packet_id();
        if let Some(p) = self.publishing_qos1_packets.get(&packet_id) {
            log::info!("Topic `{}` publish confirmed!", p.topic());
            self.publishing_qos1_packets.remove(&packet.packet_id());
        } else {
            log::warn!("Failed to find PublishAckPacket: {}", packet_id);
        }
        Ok(())
    }

    /// Parse packet_id and remove from vector.
    fn subscribe_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("subscribe_ack()");
        let mut ba = ByteArray::new(buf);
        let packet = SubscribeAckPacket::decode(&mut ba)?;
        let packet_id = packet.packet_id();
        if let Some(p) = self.subscribing_packets.get(&packet_id) {
            log::info!("Subscription {:?} confirmed!", p.topics());
            self.subscribing_packets.remove(&packet.packet_id());
        } else {
            log::warn!("Failed to find SubscribeAckPacket: {}", packet_id);
        }
        Ok(())
    }

    fn unsubscribe_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("unsubscribe_ack()");
        let mut ba = ByteArray::new(buf);
        let packet = UnsubscribeAckPacket::decode(&mut ba)?;
        let packet_id = packet.packet_id();
        if let Some(p) = self.unsubscribing_packets.get(&packet_id) {
            log::info!("Topics {:?} unsubscribe confirmed!", p);
            self.unsubscribing_packets.remove(&packet.packet_id());
        } else {
            log::warn!("Failed to find UnsubscribeAckPacket: {}", packet_id);
        }
        Ok(())
    }

    fn next_packet_id(&mut self) -> PacketId {
        if self.packet_id == u16::MAX {
            self.packet_id = 1;
        } else {
            self.packet_id += 1;
        }
        self.packet_id
    }

    pub fn connect_option(&self) -> &ConnectOptions {
        return &self.connect_options;
    }
}
