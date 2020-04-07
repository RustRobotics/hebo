// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::{
    base::*, connect_options::*, connect_packet::ConnectPacket, publish_packet::PublishPacket,
};
use std::fmt::Debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub trait AsyncDelegate: Debug {
    fn on_connect(&self);
    fn on_message(&self, buf: &[u8]);
}

#[derive(Debug)]
pub struct AsyncClient {
    connect_options: ConnectOptions,
    socket: TcpStream,
    delegate: Option<Box<dyn AsyncDelegate>>,
}

impl AsyncClient {
    pub async fn new(
        connect_options: ConnectOptions,
        delegate: Option<Box<dyn AsyncDelegate>>,
    ) -> AsyncClient {
        let socket = TcpStream::connect(connect_options.address()).await.unwrap();
        AsyncClient {
            connect_options,
            socket,
            delegate,
        }
    }

    pub async fn start(&mut self) {
        log::info!("client.start()");

        let conn_packet = ConnectPacket::new();
        self.send(conn_packet).await;
        log::info!("send conn packet");

        let (reader, _) = self.socket.split();
        // Start read loop.
        let mut buf_reader = BufReader::new(reader);
        let mut buf: Vec<u8> = Vec::with_capacity(1024);
        log::info!("reader loop");

        loop {
            let n_recv = buf_reader.read(&mut buf).await.unwrap();
            if n_recv == 0 {
                continue;
            }
            log::info!("n_recv: {}", n_recv);
            match FixedHeader::from_net(&buf) {
                Ok(fixed_header) => {
                    log::info!("fixed header: {:?}", fixed_header);
                }
                Err(err) => log::info!("err: {:?}", err),
            }

            buf.clear();
        }
    }

    async fn send<P: ToNetPacket>(&mut self, packet: P) {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        log::info!("buf: {:?}", buf);
        self.socket.write_all(&buf).await.unwrap();
    }

    pub async fn publish(&mut self, topic: &str, qos: QoSLevel, data: &[u8]) {
        let mut msg_packet = PublishPacket::new(topic.as_bytes());
        msg_packet.set_message(data).unwrap();
        log::info!("Send publish packet");
        self.send(msg_packet).await;
    }

    pub async fn disconnect(&mut self) {}

    pub fn on_connect(&mut self) {
        match self.delegate {
            Some(ref d) => d.on_connect(),
            _ => (),
        }
    }

    pub fn on_disconnect(&mut self) {}

    pub fn on_message(&mut self) {}
}
