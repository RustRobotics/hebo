// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use futures_util::{SinkExt, StreamExt};
use std::fs::File;
use std::io::BufReader;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::rustls::internal::pemfile;
use tokio_rustls::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;
use tokio_tungstenite::{self, tungstenite::protocol::Message, WebSocketStream};
use tokio::sync::mpsc::{self, Receiver, Sender};
use codec::{PublishPacket, QoS, SubscribePacket, Topic, UnsubscribePacket};

use crate::commands::{ConnectionCommand, ConnectionId, ServerCommand};
use crate::config::{self, Protocol};
use crate::session::Session;
use crate::error::{Error, ErrorKind};
use crate::stream::Stream;

#[derive(Debug)]
pub struct Listener {
    protocol: ListenerProtocol,
    pipelines: Vec<Pipeline>,

    connection_rx: Receiver<ConnectionCommand>,
    connection_tx: Sender<ConnectionCommand>,
    current_connection_id: ConnectionId,
}

fn topic_match(topics: &[SubscribedTopic], topic_str: &str) -> bool {
    for topic in topics {
        if topic.pattern.is_match(topic_str) {
            return true;
        }
    }
    false
}

#[derive(Debug)]
pub struct SubscribedTopic {
    pattern: Topic,
    qos: QoS,
}

#[derive(Debug)]
pub struct Pipeline {
    server_tx: Sender<ServerCommand>,
    topics: Vec<SubscribedTopic>,
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

/// Each Listener binds to a specific port
enum ListenerProtocol {
    Mqtt(TcpListener),
    Mqtts(TcpListener, TlsAcceptor),
    Ws(TcpListener),
    Wss(TcpListener, TlsAcceptor),
}

impl Listener {
    fn new(protocol: ListenerProtocol) -> Self {
        Listener { protocol }
    }

    fn load_certs(path: &String) -> Result<Vec<Certificate>, Error> {
        pemfile::certs(&mut BufReader::new(File::open(path)?)).map_err(|err| {
            Error::with_string(
                ErrorKind::CertError,
                format!("Failed to load cert file at {}, got: {:?}", &path, err),
            )
        })
    }

    fn load_keys(path: &String) -> Result<Vec<PrivateKey>, Error> {
        pemfile::rsa_private_keys(&mut BufReader::new(File::open(path)?)).map_err(|err| {
            Error::with_string(
                ErrorKind::CertError,
                format!("Failed to load key file at {}, got {:?}", &path, err),
            )
        })
    }

    pub async fn bind(listener: &config::Listener) -> Result<Listener, Error> {
        match listener.protocol {
            Protocol::Mqtt => {
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(ListenerProtocol::Mqtt(listener)));
                }
            }
            Protocol::Mqtts => {
                let cert_file = listener
                    .cert_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "cert_file is required"))?;
                let key_file = listener
                    .key_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "key_file is required"))?;

                let certs = Listener::load_certs(cert_file)?;
                let mut keys = Listener::load_keys(key_file)?;
                let mut config = ServerConfig::new(NoClientAuth::new());
                config
                    .set_single_cert(certs, keys.remove(0))
                    .map_err(|err| {
                        Error::with_string(
                            ErrorKind::CertError,
                            format!("Failed to init ServerConfig, got {:?}", err),
                        )
                    })?;

                let acceptor = TlsAcceptor::from(Arc::new(config));

                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(ListenerProtocol::Mqtts(listener, acceptor)));
                }
            }
            Protocol::Ws => {
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(ListenerProtocol::Ws(listener)));
                }
            }
            Protocol::Wss => {
                let cert_file = listener
                    .cert_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "cert_file is required"))?;
                let key_file = listener
                    .key_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "key_file is required"))?;

                let certs = Listener::load_certs(cert_file)?;
                let mut keys = Listener::load_keys(key_file)?;
                let mut config = ServerConfig::new(NoClientAuth::new());
                config
                    .set_single_cert(certs, keys.remove(0))
                    .map_err(|err| {
                        Error::with_string(
                            ErrorKind::CertError,
                            format!("Failed to init ServerConfig, got {:?}", err),
                        )
                    })?;

                let acceptor = TlsAcceptor::from(Arc::new(config));

                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(ListenerProtocol::Wss(listener, acceptor)));
                }
            }
        }
        Err(Error::with_string(
            ErrorKind::SocketError,
            format!("Failed to create server socket with config: {:?}", listener),
        ))
    }

    pub async fn accept(&self) -> Result<Stream, Error> {
        match self.protocol {
            ListenerProtocol::Mqtt(listener) => {
                let (tcp_stream, _address) = listener.accept().await?;
                return Ok(Stream::Mqtt(tcp_stream));
            }
            ListenerProtocol::Mqtts(listener, acceptor) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let tls_stream = acceptor.accept(tcp_stream).await?;
                return Ok(Stream::Mqtts(tls_stream));
            }
            ListenerProtocol::Ws(listener) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let ws_stream = tokio_tungstenite::accept_async(tcp_stream).await?;
                return Ok(Stream::Ws(ws_stream));
            }
            ListenerProtocol::Wss(listener, acceptor) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let tls_stream = acceptor.accept(tcp_stream).await?;
                let ws_stream = tokio_tungstenite::accept_async(tls_stream).await?;
                return Ok(Stream::Wss(ws_stream));
            }
        }
    }
}

impl Listener {
    async fn new_connection(&mut self, stream: Stream) {
        let (server_tx, server_rx) = mpsc::channel(10);
        let connection_id = self.next_connection_id();
        let pipeline = Pipeline::new(server_tx, connection_id);
        self.pipelines.push(pipeline);
        let connection =
            Session::new(stream, connection_id, self.connection_tx.clone(), server_rx);
        tokio::spawn(connection.run_loop());
    }

    pub async fn run_loop(&mut self) -> never {
        loop {
            match listener.accept().await {
                Ok(stream) => listener.new_connection(stream).await,
                Err(err) => log::error!("Failed to accept incoming connect!"),
            }
        }
    }

    async fn route_cmd(&mut self, cmd: ConnectionCommand) {
        match cmd {
            ConnectionCommand::Publish(packet) => self.on_publish(packet).await,
            ConnectionCommand::Subscribe(connection_id, packet) => {
                self.on_subscribe(connection_id, packet);
            }
            ConnectionCommand::Unsubscribe(connection_id, packet) => {
                self.on_unsubscribe(connection_id, packet)
            }
            ConnectionCommand::Disconnect(connection_id) => {
                if let Some(pos) = self
                    .pipelines
                    .iter()
                    .position(|pipe| pipe.connection_id == connection_id)
                {
                    log::debug!("Remove pipeline: {}", connection_id);
                    self.pipelines.remove(pos);
                }
            }
        }
    }

    fn next_connection_id(&mut self) -> ConnectionId {
        self.current_connection_id += 1;
        self.current_connection_id
    }

    fn on_subscribe(&mut self, connection_id: ConnectionId, packet: SubscribePacket) {
        for pipeline in self.pipelines.iter_mut() {
            if pipeline.connection_id == connection_id {
                for topic in packet.topics() {
                    // TODO(Shaohua): Returns error
                    match Topic::parse(topic.topic()) {
                        Ok(pattern) => pipeline.topics.push(SubscribedTopic {
                            pattern,
                            qos: topic.qos(),
                        }),
                        Err(err) => log::error!("Invalid sub topic: {:?}, err: {:?}", topic, err),
                    }
                }
                break;
            }
        }
    }

    fn on_unsubscribe(&mut self, connection_id: ConnectionId, packet: UnsubscribePacket) {
        for pipeline in self.pipelines.iter_mut() {
            if pipeline.connection_id == connection_id {
                pipeline
                    .topics
                    .retain(|ref topic| !packet.topics().any(|t| t == topic.pattern.topic()));
            }
            break;
        }
    }

    async fn on_publish(&mut self, packet: PublishPacket) {
        let cmd = ServerCommand::Publish(packet.clone());
        // TODO(Shaohua): Replace with a trie tree and a hash table.
        for pipeline in self.pipelines.iter_mut() {
            if topic_match(&pipeline.topics, packet.topic()) {
                if let Err(err) = pipeline.server_tx.send(cmd.clone()).await {
                    log::warn!("Failed to send publish packet to connection: {}", err);
                }
            }
        }
    }
}
