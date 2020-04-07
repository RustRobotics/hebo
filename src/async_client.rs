// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::{
    base::*, connect_options::*, connect_packet::ConnectPacket, publish_packet::PublishPacket,
};
use std::cell::RefCell;
use std::fmt::Debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub trait AsyncDelegate: Debug {
    fn on_connect(&self, client: &AsyncClient);
    fn on_message(&self, buf: &[u8]);
}

#[derive(Debug)]
pub struct AsyncClient {
    connect_options: ConnectOptions,
    socket: RefCell<TcpStream>,
    delegate: Option<Box<dyn AsyncDelegate>>,
}

impl AsyncClient {
    pub async fn new(
        connect_options: ConnectOptions,
        delegate: Option<Box<dyn AsyncDelegate>>,
    ) -> AsyncClient {
        let socket = TcpStream::connect(connect_options.address()).await.unwrap();
        let mut client = AsyncClient {
            connect_options,
            socket: RefCell::new(socket),
            delegate,
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
            let n_recv = self.socket.borrow_mut().read_buf(&mut buf).await.unwrap();
            log::info!("n_recv: {:?}", n_recv);
            if n_recv == 0 {
                continue;
            }

            self.recv_router(&mut buf);

            buf.clear();
        }
    }

    fn recv_router(&mut self, buf: &mut [u8]) {
        match FixedHeader::from_net(&buf) {
            Ok(fixed_header) => {
                log::info!("fixed header: {:?}", fixed_header);
                match fixed_header.packet_type {
                    PacketType::ConnectAck => {
                        self.on_connect();
                    }
                    _ => (),
                }
            }
            Err(err) => log::warn!("err: {:?}", err),
        }
    }

    async fn send<P: ToNetPacket>(&self, packet: P) {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        log::info!("buf: {:?}", buf);
        self.socket.borrow_mut().write_all(&buf).await.unwrap();
    }

    pub async fn publish(&self, topic: &str, qos: QoSLevel, data: &[u8]) {
        let mut msg_packet = PublishPacket::new(topic.as_bytes());
        msg_packet.set_message(data).unwrap();
        log::info!("Send publish packet");
        self.send(msg_packet).await;
    }

    pub async fn disconnect(&mut self) {}

    pub fn on_connect(&mut self) {
        match self.delegate {
            Some(ref d) => d.on_connect(self),
            _ => (),
        }
    }

    pub fn on_disconnect(&mut self) {}

    pub fn on_message(&mut self) {}
}
