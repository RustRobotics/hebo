// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::commands::{ConnectionCommand, ServerCommand};
use ruo::base::{FixedHeader, FromNetPacket, PacketType, QoS};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

#[derive(Debug)]
enum Status {
    Invalid,
    Connected,
    Disconnecting,
    Disconnected,
}

#[derive(Debug)]
pub struct ConnectionContext {
    socket: TcpStream,
    remote_address: SocketAddr,
    sender: Sender<ConnectionCommand>,
    receiver: Receiver<ServerCommand>,
    status: Status,
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
            status: Status::Invalid,
        }
    }

    pub async fn run_loop(mut self) {
        let mut buf = Vec::new();
        // TODO(Shaohua): Handle timeout
        let mut timer = interval(Duration::from_secs(10));
        loop {
            tokio::select! {
                Ok(n_recv) = self.socket.read_buf(&mut buf) => {
                    if n_recv > 0 {
                        log::info!("n_recv: {}", n_recv);
                        self.recv_router(&buf).await;
                        buf.clear();
                    }
                }
                _ = timer.tick() => {
                    log::info!("tick()");
                },
            }
        }
    }

    async fn recv_router(&mut self, buf: &[u8]) {
        match FixedHeader::from_net(&buf) {
            Ok(fixed_header) => {
                //log::info!("fixed header: {:?}", fixed_header);
                match fixed_header.packet_type {
                    PacketType::Connect => self.connect(&buf).await,
                    t => log::warn!("Unhandled msg: {:?}", t),
                }
            }
            Err(err) => log::warn!("err: {:?}", err),
        }
    }

    async fn connect(&mut self, buf: &[u8]) {
        log::info!("connect()");
    }
}
