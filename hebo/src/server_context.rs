// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io;
use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, Receiver, Sender};

use codec::publish_packet::PublishPacket;
use codec::subscribe_packet::{SubscribePacket};

use crate::commands::{ConnectionCommand, ConnectionId, ServerCommand};
use crate::config::Config;
use crate::connection_context::ConnectionContext;
use codec::base::QoS;
use codec::topic::Topic;

#[derive(Debug)]
pub struct ServerContext {
    config: Config,

    pipelines: Vec<Pipeline>,
    connection_rx: Receiver<ConnectionCommand>,
    connection_tx: Sender<ConnectionCommand>,
    current_connection_id: ConnectionId,
}

#[derive(Debug)]
struct TopicAndQoS {
    topic: Topic,
    qos: QoS,
}

impl ServerContext {
    pub fn new(config: Config) -> ServerContext {
        let (connection_tx, connection_rx) = mpsc::channel(10);
        ServerContext {
            config,
            pipelines: Vec::new(),
            connection_rx,
            connection_tx,
            current_connection_id: 0,
        }
    }

    pub async fn run_loop(&mut self) -> io::Result<()> {
        log::info!("Listening at: {}", self.config.connections.mqtt);
        let listener = TcpListener::bind(&self.config.connections.mqtt).await?;
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
        for pipeline in self.pipelines.iter_mut() {
            if pipeline.connection_id == connection_id {
                for sub_topic in packet.topics() {
                    let topic = Topic::parse(&sub_topic.topic);
                    if topic.is_err() {
                        continue;
                    }
                    pipeline.topics.push(TopicAndQoS {
                        topic: topic.unwrap(),
                        qos: sub_topic.qos,
                    });
                }
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

fn topic_match(topics: &[TopicAndQoS], topic: &str) -> bool {
    topics.iter().any(|t| t.topic.is_match(topic))
}

#[derive(Debug)]
pub struct Pipeline {
    server_tx: Sender<ServerCommand>,
    topics: Vec<TopicAndQoS>,
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
