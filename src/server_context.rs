// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::commands::{ConnectionCommand, ServerCommand};
use super::connection_context::ConnectionContext;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
pub struct ServerContext {
    pipelines: Vec<Pipeline>,
}

impl ServerContext {
    pub fn new() -> ServerContext {
        ServerContext {
            pipelines: Vec::new(),
        }
    }

    pub async fn new_connection(&mut self, socket: TcpStream, addr: SocketAddr) {
        let (server_tx, server_rx) = mpsc::channel(10);
        let (connection_tx, connection_rx) = mpsc::channel(10);
        let pipeline = Pipeline::new(server_tx, connection_rx);
        self.pipelines.push(pipeline);
        let connection = ConnectionContext::new(socket, addr, connection_tx, server_rx);
        tokio::spawn(connection.run_loop());
    }
}

#[derive(Debug)]
pub struct Pipeline {
    sender: Sender<ServerCommand>,
    receiver: Receiver<ConnectionCommand>,
    topics: Vec<String>,
}

impl Pipeline {
    pub fn new(sender: Sender<ServerCommand>, receiver: Receiver<ConnectionCommand>) -> Pipeline {
        Pipeline {
            sender,
            receiver,
            topics: Vec::new(),
        }
    }
}
