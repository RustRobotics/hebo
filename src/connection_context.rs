// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::commands::{ConnectionCommand, ServerCommand};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::interval;

#[derive(Debug)]
pub struct ConnectionContext {
    socket: TcpStream,
    remote_address: SocketAddr,
    sender: Sender<ConnectionCommand>,
    receiver: Receiver<ServerCommand>,
}

impl ConnectionContext {
    pub fn new(
        socket: TcpStream,
        remote_address: SocketAddr,
        sender: Sender<ConnectionCommand>,
        receiver: Receiver<ServerCommand>,
    ) -> ConnectionContext {
        ConnectionContext {
            remote_address,
            socket,
            sender,
            receiver,
        }
    }

    pub async fn run_loop(mut self) {
        let mut buf = Vec::new();
        let mut timer = interval(Duration::from_secs(10));
        loop {
            tokio::select! {
                Ok(n_recv) = self.socket.read_buf(&mut buf) => {
                    if n_recv > 0 {
                        log::info!("n_recv: {}", n_recv);
                        buf.clear();
                    }
                }
                _ = timer.tick() => {
                    log::info!("tick()");
                },
            }
        }
    }
}
