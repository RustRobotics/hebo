// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::v3::{
    ConnectAckPacket, ConnectPacket, ConnectReturnCode, DisconnectPacket, FixedHeader, PacketType,
    PingRequestPacket, PublishAckPacket, PublishPacket, SubscribeAckPacket, SubscribePacket,
    UnsubscribeAckPacket, UnsubscribePacket,
};
use codec::{ByteArray, DecodePacket, EncodePacket, PacketId, QoS};
use std::collections::HashMap;
use std::fmt;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use super::stream::Stream;
use crate::connect_options::*;
use crate::error::Error;

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum ClientStatus {
    Initialized,
    Connecting,
    Connected,
    ConnectFailed,
    Disconnecting,
    Disconnected,
}

type ConnectCallback = fn(&mut Client);
type MessageCallback = fn(&mut Client, &PublishPacket);

#[derive(Debug, PartialEq)]
enum ClientToInnerCmd {
    Connect,
    Publish(PublishPacket),
    Subscribe(SubscribePacket),
    Unsubscribe(UnsubscribePacket),
    Disconnect,
}

#[derive(Debug, PartialEq)]
enum InnerToClientCmd {
    OnConnect,
    OnMessage(PublishPacket),
    OnDisconnect,
}

pub struct Client {
    connect_options: ConnectOptions,
    status: ClientStatus,
    on_connect_cb: Option<ConnectCallback>,
    on_message_cb: Option<MessageCallback>,

    client_sender: Sender<ClientToInnerCmd>,
    client_receiver: Option<Receiver<ClientToInnerCmd>>,
    inner_sender: Option<Sender<InnerToClientCmd>>,
    inner_receiver: Receiver<InnerToClientCmd>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("connect_options", &self.connect_options)
            .field("status", &self.status)
            .finish()
    }
}

impl Client {
    pub fn new(
        connect_options: ConnectOptions,
        on_connect_cb: Option<ConnectCallback>,
        on_message_cb: Option<MessageCallback>,
    ) -> Self {
        let (client_sender, client_receiver) = mpsc::channel();
        let (inner_sender, inner_receiver) = mpsc::channel();
        Self {
            connect_options,
            status: ClientStatus::Initialized,
            on_connect_cb,
            on_message_cb,
            client_sender: client_sender,
            client_receiver: Some(client_receiver),
            inner_sender: Some(inner_sender),
            inner_receiver: inner_receiver,
        }
    }

    pub fn connect_option(&self) -> &ConnectOptions {
        &self.connect_options
    }

    pub fn status(&self) -> ClientStatus {
        self.status
    }

    pub fn init(&mut self) -> Result<(), Error> {
        let client_receiver = self.client_receiver.take().unwrap();
        let inner_sender = self.inner_sender.take().unwrap();
        let mut inner = ClientInner::new(&self.connect_options, inner_sender, client_receiver)?;
        thread::spawn(move || {
            inner.run_loop().unwrap();
        });

        Ok(())
    }

    pub fn process_events(&mut self) {
        if let Ok(cmd) = self.inner_receiver.try_recv() {
            match cmd {
                InnerToClientCmd::OnConnect => {
                    self.status = ClientStatus::Connected;
                    self.on_connect()
                }
                InnerToClientCmd::OnMessage(packet) => self.on_message(packet),
                InnerToClientCmd::OnDisconnect => {
                    self.status = ClientStatus::Disconnected;
                }
            }
        }
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        self.status = ClientStatus::Connecting;
        self.client_sender.send(ClientToInnerCmd::Connect).unwrap();
        Ok(())
    }

    pub fn publish(&mut self, topic: &str, qos: QoS, data: &[u8]) -> Result<(), Error> {
        log::info!("client publish()");
        let packet = PublishPacket::new(topic, qos, data)?;
        self.client_sender
            .send(ClientToInnerCmd::Publish(packet))
            .unwrap();
        Ok(())
    }

    pub fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Error> {
        let packet = SubscribePacket::new(topic, qos, PacketId::new(0))?;
        self.client_sender
            .send(ClientToInnerCmd::Subscribe(packet))
            .unwrap();
        Ok(())
    }

    pub fn unsubscribe(&mut self, topic: &str) -> Result<(), Error> {
        let packet = UnsubscribePacket::new(topic, PacketId::new(0))?;
        self.client_sender
            .send(ClientToInnerCmd::Unsubscribe(packet))
            .unwrap();
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        log::info!("client disconnect()");
        self.status = ClientStatus::Disconnecting;
        self.client_sender
            .send(ClientToInnerCmd::Disconnect)
            .unwrap();
        Ok(())
    }

    fn on_connect(&mut self) {
        if let Some(cb) = &self.on_connect_cb {
            cb(self);
        }
    }

    fn on_message(&mut self, packet: PublishPacket) {
        if let Some(cb) = &self.on_message_cb {
            cb(self, &packet);
        }
    }
}

struct ClientInner {
    client_id: String,
    stream: Stream,
    status: ClientStatus,
    topics: HashMap<String, PacketId>,
    packet_id: PacketId,
    subscribing_packets: HashMap<PacketId, SubscribePacket>,
    unsubscribing_packets: HashMap<PacketId, UnsubscribePacket>,
    publishing_qos1_packets: HashMap<PacketId, PublishPacket>,
    publishing_qos2_packets: HashMap<PacketId, PublishPacket>,

    inner_sender: Sender<InnerToClientCmd>,
    client_receiver: Receiver<ClientToInnerCmd>,
}

impl ClientInner {
    fn new(
        connect_options: &ConnectOptions,
        inner_sender: Sender<InnerToClientCmd>,
        client_receiver: Receiver<ClientToInnerCmd>,
    ) -> Result<Self, Error> {
        let stream = Stream::new(connect_options.connect_type())?;
        Ok(ClientInner {
            client_id: connect_options.client_id().to_string(),
            stream,
            status: ClientStatus::Initialized,
            topics: HashMap::new(),
            packet_id: PacketId::new(1),
            subscribing_packets: HashMap::new(),
            unsubscribing_packets: HashMap::new(),
            publishing_qos1_packets: HashMap::new(),
            publishing_qos2_packets: HashMap::new(),
            inner_sender,
            client_receiver,
        })
    }

    fn run_loop(&mut self) -> Result<(), Error> {
        let mut buf = Vec::with_capacity(1024);
        let timeout = Duration::from_millis(1);

        loop {
            if self.status == ClientStatus::Disconnected {
                break;
            }
            if let Ok(cmd) = self.client_receiver.recv_timeout(timeout) {
                self.handle_client_cmd(cmd);
            }
            buf.resize(buf.capacity(), 0);
            if let Ok(n_recv) = self.stream.read_buf(&mut buf) {
                if n_recv > 0 {
                    if let Err(err) = self.handle_session_packet(&mut buf) {
                        log::error!("err: {:?}", err);
                    }
                    buf.clear();
                } else if n_recv == 0 {
                    log::warn!("n_recv is 0");
                    break;
                }
            }
        }

        Ok(())
    }

    fn send<P: EncodePacket>(&mut self, packet: P) -> Result<(), Error> {
        let mut buf = Vec::new();
        packet.encode(&mut buf)?;
        self.stream.write_all(&buf).map_err(|err| err.into())
    }

    fn handle_client_cmd(&mut self, cmd: ClientToInnerCmd) -> Result<(), Error> {
        // TODO(Shaohua): Check client status first.
        match cmd {
            ClientToInnerCmd::Connect => self.do_connect(),
            ClientToInnerCmd::Publish(packet) => self.do_publish(packet),
            ClientToInnerCmd::Subscribe(packet) => self.do_subscribe(packet),
            ClientToInnerCmd::Unsubscribe(packet) => self.do_unsubscribe(packet),
            ClientToInnerCmd::Disconnect => self.do_disconnect(),
        }
    }

    fn handle_session_packet(&mut self, buf: &mut Vec<u8>) -> Result<(), Error> {
        let mut ba = ByteArray::new(buf);
        let fixed_header = FixedHeader::decode(&mut ba)?;
        log::info!("fixed header: {:?}", fixed_header);
        match fixed_header.packet_type() {
            PacketType::ConnectAck => self.on_connect_ack(&buf),
            PacketType::Publish { .. } => self.on_message(&buf),
            PacketType::PublishAck => self.on_publish_ack(&buf),
            PacketType::SubscribeAck => self.on_subscribe_ack(&buf),
            PacketType::UnsubscribeAck => self.on_unsubscribe_ack(&buf),
            PacketType::PingResponse => self.on_ping_resp(),
            t => {
                log::info!("Unhandled msg: {:?}", t);
                Ok(())
            }
        }
    }

    fn do_connect(&mut self) -> Result<(), Error> {
        log::info!(" inner do_connect() client id: {}", &self.client_id);
        let conn_packet = ConnectPacket::new(&self.client_id)?;
        self.send(conn_packet)
    }

    fn do_publish(&mut self, mut packet: PublishPacket) -> Result<(), Error> {
        log::info!("inner do_publish: {:?}", packet.topic());
        match packet.qos() {
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
        self.send(packet)
    }

    fn do_subscribe(&mut self, mut packet: SubscribePacket) -> Result<(), Error> {
        log::info!("inner do_subscribe: {:?}", packet.topics());
        let packet_id = self.next_packet_id();
        // TODO(Shaohua): Support multiple topics.
        //self.topics.insert(packet.topic().to_string(), packet_id);
        packet.set_packet_id(packet_id);
        self.subscribing_packets.insert(packet_id, packet.clone());
        self.send(packet)
    }

    fn do_unsubscribe(&mut self, mut packet: UnsubscribePacket) -> Result<(), Error> {
        log::info!("inner do_unsubscribe: {:?}", packet);
        let packet_id = self.next_packet_id();
        packet.set_packet_id(packet_id);
        self.unsubscribing_packets.insert(packet_id, packet.clone());
        self.send(packet)
    }

    fn do_disconnect(&mut self) -> Result<(), Error> {
        log::info!("inner do_disconnect()");
        // Send disconnect packet to broker.
        if self.status == ClientStatus::Connected {
            self.status = ClientStatus::Disconnecting;
            let packet = DisconnectPacket::new();
            self.send(packet)?;
        } else {
            // TODO(Shaohua): Return errors
            return Ok(());
        }

        self.status = ClientStatus::Disconnected;
        // Send disconnect packet to client.
        self.inner_sender
            .send(InnerToClientCmd::OnDisconnect)
            .unwrap();

        // Network connection will be closed soon.

        Ok(())
    }

    fn on_connect(&mut self) {
        self.inner_sender.send(InnerToClientCmd::OnConnect).unwrap();
    }

    fn ping(&mut self) -> Result<(), Error> {
        log::info!("ping()");
        if self.status == ClientStatus::Connected {
            log::info!("Send ping packet");
            let packet = PingRequestPacket::new();
            self.send(packet)
        } else {
            // TODO(Shaohua): Return Error
            Ok(())
        }
    }

    fn on_message(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("on_message()");
        let mut ba = ByteArray::new(buf);
        let packet = PublishPacket::decode(&mut ba)?;
        log::info!("on_message() packet: {:?}", packet);
        self.inner_sender
            .send(InnerToClientCmd::OnMessage(packet))
            .unwrap();
        Ok(())
    }

    fn on_ping_resp(&self) -> Result<(), Error> {
        log::info!("on ping resp");
        // TODO(Shaohua): Reset reconnect timer.
        Ok(())
    }

    fn on_connect_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("connect_ack()");
        let mut ba = ByteArray::new(buf);
        let packet = ConnectAckPacket::decode(&mut ba)?;
        match packet.return_code() {
            ConnectReturnCode::Accepted => {
                self.status = ClientStatus::Connected;
                self.on_connect();
            }
            _ => {
                log::warn!("Failed to connect to server, {:?}", packet.return_code());
                self.status = ClientStatus::ConnectFailed;
            }
        }
        Ok(())
    }

    fn on_publish_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
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

    fn on_subscribe_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("subscribe_ack()");
        // Parse packet_id and remove from vector.
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

    fn on_unsubscribe_ack(&mut self, buf: &[u8]) -> Result<(), Error> {
        log::info!("unsubscribe_ack()");
        let mut ba = ByteArray::new(buf);
        let packet = UnsubscribeAckPacket::decode(&mut ba)?;
        let packet_id = packet.packet_id();
        // TODO(Shaohua): Tuning
        if let Some(_p) = self.unsubscribing_packets.get(&packet_id) {
            self.unsubscribing_packets.remove(&packet.packet_id());
        } else {
            log::warn!("Failed to find UnsubscribeAckPacket: {}", packet_id);
        }
        Ok(())
    }

    fn next_packet_id(&mut self) -> PacketId {
        if self.packet_id == u16::MAX {
            self.packet_id = PacketId::new(1);
        } else {
            self.packet_id += 1;
        }
        self.packet_id
    }
}
