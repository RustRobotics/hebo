// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::base::ToNetPacket;
use std::io::{self, Write};
use std::net::{TcpStream, SocketAddr};

#[derive(Debug)]
pub struct SyncStream {
    socket: TcpStream,
}

impl SyncStream {
    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        let socket = TcpStream::connect(addr)?;
        Ok(SyncStream {
            socket,
        })
    }

    pub fn send<P: ToNetPacket>(&mut self, packet: P) {
        let mut buf = Vec::new();
        packet.to_net(&mut buf).unwrap();
        let n_recv = self.socket.write(&buf).unwrap();
        log::info!("n_recv: {:?}", n_recv);
    }
}
