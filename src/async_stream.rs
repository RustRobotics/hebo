// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::ToNetPacket;
use tokio::prelude::*;
use tokio::net::TcpStream;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct AsyncStream {
    socket: TcpStream,
}

impl AsyncStream {
    pub async fn connect(addr: SocketAddr) -> Self {
        let socket = TcpStream::connect(addr).await.unwrap();
        AsyncStream {
            socket,
        }
    }

    pub async fn send<P: ToNetPacket>(&mut self, packet: P) {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        log::info!("buf: {:?}", buf);
        self.socket.write_all(&buf).await.unwrap();
    }
}
