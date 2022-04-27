// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::v3::{
    ConnectAckPacket, ConnectPacket, ConnectReturnCode, DisconnectPacket, FixedHeader, PacketType,
    PingRequestPacket, PublishAckPacket, PublishPacket, SubscribeAckPacket, SubscribePacket,
    UnsubscribeAckPacket, UnsubscribePacket,
};
use codec::{ByteArray, DecodePacket, EncodePacket, PacketId, QoS};
use std::fmt;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use super::client_inner::{ClientInner, ClientToInnerCmd, InnerToClientCmd};
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
