// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::*;
use super::connect_options::*;
use super::connect_packet::ConnectPacket;
use super::ping_packet::PingPacket;
use super::publish_packet::PublishPacket;
use super::subscribe_packet::SubscribePacket;
use std::cell::RefCell;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::interval;

#[derive(Clone, Debug)]
pub struct AsyncClient {
    connect_options: ConnectOptions,
    socket: Arc<Mutex<RefCell<TcpStream>>>,
}

impl AsyncClient {
    pub async fn new(connect_options: ConnectOptions) -> AsyncClient {
        let socket = TcpStream::connect(connect_options.address()).await.unwrap();
        let client = AsyncClient {
            connect_options,
            socket: Arc::new(Mutex::new(RefCell::new(socket))),
        };

        let conn_packet = ConnectPacket::new();
        client.send(conn_packet).await;
        log::info!("send conn packet");

        client
    }

    pub async fn start(&mut self) {
        log::info!("client.start()");

        let mut buf: Vec<u8> = Vec::with_capacity(1024);
        log::info!("reader loop");
        loop {
            let n_recv = self
                .socket
                .lock()
                .unwrap()
                .borrow_mut()
                .read_buf(&mut buf)
                .await
                .unwrap();
            if n_recv == 0 {
                continue;
            }
            //log::info!("n_recv: {:?}", n_recv);
            self.recv_router(&mut buf).await;
            buf.clear();
        }
    }

    async fn recv_router(&mut self, buf: &mut Vec<u8>) {
        match FixedHeader::from_net(&buf) {
            Ok(fixed_header) => {
                //log::info!("fixed header: {:?}", fixed_header);
                match fixed_header.packet_type {
                    PacketType::ConnectAck => self.on_connect().await,
                    PacketType::Publish => self.on_message(&buf).await,
                    PacketType::PubAck => log::info!("PubAck: {:x?}", &buf),
                    PacketType::SubAck => log::info!("SubAck: {:x?}", &buf),
                    t => log::info!("Unhandled msg: {:?}", t),
                }
            }
            Err(err) => log::warn!("err: {:?}", err),
        }
    }

    async fn send<P: ToNetPacket>(&self, packet: P) {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        if let Ok(socket_cell) = self.socket.lock() {
            let mut socket = socket_cell.borrow_mut();
            socket.write_all(&buf).await.unwrap();
        }
    }

    pub async fn publish(&self, topic: &str, qos: QoSLevel, data: &[u8]) {
        log::info!("Send publish packet");
        let packet = PublishPacket::new(topic, qos, data);
        self.send(packet).await;
    }

    pub async fn subscribe(&self, topic: &str, qos: QoSLevel) {
        log::info!("subscribe to: {}", topic);
        let packet = SubscribePacket::new(topic, qos);
        self.send(packet).await;
    }

    pub async fn disconnect(&mut self) {}

    async fn on_connect(&self) {
        log::info!("On connect()");
        self.subscribe("hello", QoSLevel::QoS0).await;
        self.publish("hello", QoSLevel::QoS0, b"Hello, world").await;

        self.start_timer();
    }

    fn start_timer(&self) {
        let client = self.clone();
        tokio::spawn(async move {
            log::info!("client: {:?}", client);
            let mut timer = interval(Duration::from_secs(3));
            loop {
                log::info!("tick()");
                timer.tick().await;
                client.ping().await;
            }
        });
    }

    async fn ping(&self) {
        let packet = PingPacket::new();
        self.send(packet).await;
    }

    //fn on_disconnect(&mut self) {}

    async fn on_message(&self, buf: &Vec<u8>) {
        match PublishPacket::from_net(buf) {
            Ok(packet) => {
                log::info!("packet: {:?}", packet);
                log::info!("message: {:?}", std::str::from_utf8(packet.message()));
            }
            Err(err) => log::warn!("Failed to parse publish msg: {:?}", err),
        }
    }
}
