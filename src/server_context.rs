// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::commands::{ConnectionCommand, ConnectionId, ServerCommand};
use crate::connection_context::ConnectionContext;
use ruo::publish_packet::PublishPacket;
use ruo::subscribe_packet::SubscribePacket;
use std::io;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
pub struct ServerContext {
    pipelines: Vec<Pipeline>,
    address: String,
    connection_rx: Receiver<ConnectionCommand>,
    connection_tx: Sender<ConnectionCommand>,
    current_connection_id: ConnectionId,
}

impl ServerContext {
    pub fn new(address: &str) -> ServerContext {
        let (connection_tx, connection_rx) = mpsc::channel(10);
        ServerContext {
            pipelines: Vec::new(),
            address: address.to_string(),
            connection_rx,
            connection_tx,
            current_connection_id: 0,
        }
    }

    pub async fn run_loop(&mut self) -> io::Result<()> {
        let mut listener = TcpListener::bind(&self.address).await?;
        loop {
            tokio::select! {
                Ok((socket, address)) = listener.accept() => {
                    log::info!("accept()");
                    log::info!("remote address: {:?}", address);
                    self.new_connection(socket, address).await;
                },
                Some(cmd) = self.connection_rx.recv() => {
                    self.route_cmd(cmd).await;
                },
            }
        }
    }

    async fn new_connection(&mut self, socket: TcpStream, addr: SocketAddr) {
        let (server_tx, server_rx) = mpsc::channel(10);
        let connection_id = self.next_connection_id();
        let pipeline = Pipeline::new(server_tx, connection_id);
        self.pipelines.push(pipeline);
        let connection = ConnectionContext::new(
            socket,
            addr,
            connection_id,
            self.connection_tx.clone(),
            server_rx,
        );
        tokio::spawn(connection.run_loop());
    }

    async fn route_cmd(&mut self, cmd: ConnectionCommand) {
        log::info!("new cmd received: {:?}", cmd);
        match cmd {
            ConnectionCommand::Publish(packet) => self.on_publish(packet).await,
            ConnectionCommand::Subscribe(connection_id, packet) => {
                self.on_subscribe(connection_id, packet);
            }
            ConnectionCommand::Unsubscribe => log::info!("TODO: unsubscribe!"),
        }
    }

    fn next_connection_id(&mut self) -> ConnectionId {
        self.current_connection_id += 1;
        self.current_connection_id
    }

    fn on_subscribe(&mut self, connection_id: ConnectionId, packet: SubscribePacket) {
        // TODO(Shaohua): Consider adding qos

        for pipeline in self.pipelines.iter_mut() {
            if pipeline.connection_id == connection_id {
                pipeline.topics.push(packet.topic().to_string());
                break;
            }
        }
    }

    async fn on_publish(&mut self, packet: PublishPacket) {
        let cmd = ServerCommand::Publish(packet.clone());
        for pipeline in self.pipelines.iter_mut() {
            if topic_match(&pipeline.topics, packet.topic()) {
                log::info!("server_context will send publish packet");
                if let Err(err) = pipeline.server_tx.send(cmd.clone()).await {
                    log::warn!("Failed to send publish packet to connection: {}", err);
                }
            }
        }
    }
}

fn topic_match(topics: &[String], topic: &str) -> bool {
    // TODO(Shaohua): Create a topic parsing tree
    topics.iter().any(|t| t == topic)
}

#[derive(Debug)]
pub struct Pipeline {
    server_tx: Sender<ServerCommand>,
    topics: Vec<String>,
    connection_id: ConnectionId,
}

impl Pipeline {
    pub fn new(server_tx: Sender<ServerCommand>, connection_id: ConnectionId) -> Pipeline {
        Pipeline {
            server_tx,
            topics: Vec::new(),
            connection_id,
        }
    }
}
