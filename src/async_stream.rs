// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::ToNetPacket;
use std::net::SocketAddr;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::prelude::*;

#[derive(Debug)]
pub struct AsyncStream {
    socket: TcpStream,
}

impl AsyncStream {
    pub async fn connect(addr: SocketAddr) -> Self {
        let socket = TcpStream::connect(addr).await.unwrap();
        AsyncStream { socket }
    }

    pub async fn recv(&mut self) {
        let (reader, _) = self.socket.split();
        // Start read loop.
        let mut buf_reader = BufReader::new(reader);
        let mut buf = [0_u8; 1024];
        log::info!("reader loop");
        loop {
            let n_recv = buf_reader.read(&mut buf).await.unwrap();
            println!("n_recv: {}", n_recv);
            println!("buf: {:?}", &buf[..24]);
        }
    }

    pub async fn send<P: ToNetPacket>(&mut self, packet: P) {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        log::info!("buf: {:?}", buf);
        self.socket.write_all(&buf).await.unwrap();
    }
}
