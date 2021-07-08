// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use codec::{PublishPacket, QoS, SubscribePacket, Topic, UnsubscribePacket};
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::net::{TcpListener, UnixListener};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_rustls::rustls::internal::pemfile;
use tokio_rustls::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;

use crate::commands::{
    ConnectionId, ListenerToSessionCmd, ListenerToStorageCmd, SessionToListenerCmd,
    StorageToListenerCmd,
};
use crate::config;
use crate::error::{Error, ErrorKind};
use crate::session::Session;
use crate::stream::Stream;

#[derive(Debug)]
pub struct Listener {
    protocol: Protocol,
    current_connection_id: ConnectionId,
    pipelines: Vec<Pipeline>,
    session_sender: Sender<SessionToListenerCmd>,
    session_receiver: Option<Receiver<SessionToListenerCmd>>,

    storage_sender: Sender<ListenerToStorageCmd>,
    storage_receiver: Option<Receiver<StorageToListenerCmd>>,
}

/// Each Listener binds to a specific port
enum Protocol {
    Mqtt(TcpListener),
    Mqtts(TcpListener, TlsAcceptor),
    Ws(TcpListener),
    Wss(TcpListener, TlsAcceptor),
    Uds(UnixListener),
}

impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Protocol::Mqtt(..) => "Mqtt",
            Protocol::Mqtts(..) => "Mqtts",
            Protocol::Ws(..) => "Ws",
            Protocol::Wss(..) => "Wss",
            Protocol::Uds(..) => "Uds",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub struct Pipeline {
    sender: mpsc::Sender<ListenerToSessionCmd>,
    topics: Vec<SubscribedTopic>,
    connection_id: ConnectionId,
}

impl Pipeline {
    pub fn new(
        sender: mpsc::Sender<ListenerToSessionCmd>,
        connection_id: ConnectionId,
    ) -> Pipeline {
        Pipeline {
            sender,
            topics: Vec::new(),
            connection_id,
        }
    }
}

#[derive(Debug)]
pub struct SubscribedTopic {
    pattern: Topic,
    qos: QoS,
}

// Initialize Listener
impl Listener {
    fn new(
        protocol: Protocol,
        storage_sender: Sender<ListenerToStorageCmd>,
        storage_receiver: Receiver<StorageToListenerCmd>,
    ) -> Self {
        let (session_sender, session_receiver) = mpsc::channel(10);
        Listener {
            protocol,
            current_connection_id: 0,
            pipelines: Vec::new(),
            session_sender,
            session_receiver: Some(session_receiver),

            storage_sender,
            storage_receiver: Some(storage_receiver),
        }
    }

    fn load_certs(path: &String) -> Result<Vec<Certificate>, Error> {
        pemfile::certs(&mut BufReader::new(File::open(path)?)).map_err(|err| {
            Error::from_string(
                ErrorKind::CertError,
                format!("Failed to load cert file at {}, got: {:?}", &path, err),
            )
        })
    }

    fn load_keys(path: &String) -> Result<Vec<PrivateKey>, Error> {
        pemfile::rsa_private_keys(&mut BufReader::new(File::open(path)?)).map_err(|err| {
            Error::from_string(
                ErrorKind::CertError,
                format!("Failed to load key file at {}, got {:?}", &path, err),
            )
        })
    }

    pub async fn bind(
        listener: &config::Listener,
        storage_sender: Sender<ListenerToStorageCmd>,
        storage_receiver: Receiver<StorageToListenerCmd>,
    ) -> Result<Listener, Error> {
        match listener.protocol {
            config::Protocol::Mqtt => {
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(
                        Protocol::Mqtt(listener),
                        storage_sender,
                        storage_receiver,
                    ));
                }
            }
            config::Protocol::Mqtts => {
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
                        Error::from_string(
                            ErrorKind::CertError,
                            format!("Failed to init ServerConfig, got {:?}", err),
                        )
                    })?;

                let acceptor = TlsAcceptor::from(Arc::new(config));

                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(
                        Protocol::Mqtts(listener, acceptor),
                        storage_sender,
                        storage_receiver,
                    ));
                }
            }
            config::Protocol::Ws => {
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(
                        Protocol::Ws(listener),
                        storage_sender,
                        storage_receiver,
                    ));
                }
            }
            config::Protocol::Wss => {
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
                        Error::from_string(
                            ErrorKind::CertError,
                            format!("Failed to init ServerConfig, got {:?}", err),
                        )
                    })?;

                let acceptor = TlsAcceptor::from(Arc::new(config));

                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(
                        Protocol::Wss(listener, acceptor),
                        storage_sender,
                        storage_receiver,
                    ));
                }
            }

            config::Protocol::Uds => {
                let listener = UnixListener::bind(&listener.address)?;
                return Ok(Listener::new(
                    Protocol::Uds(listener),
                    storage_sender,
                    storage_receiver,
                ));
            }
        }
        Err(Error::from_string(
            ErrorKind::SocketError,
            format!("Failed to create server socket with config: {:?}", listener),
        ))
    }

    pub async fn accept(&self) -> Result<Stream, Error> {
        match &self.protocol {
            Protocol::Mqtt(listener) => {
                let (tcp_stream, _address) = listener.accept().await?;
                return Ok(Stream::Mqtt(tcp_stream));
            }
            Protocol::Mqtts(listener, acceptor) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let tls_stream = acceptor.accept(tcp_stream).await?;
                return Ok(Stream::Mqtts(tls_stream));
            }
            Protocol::Ws(listener) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let ws_stream = tokio_tungstenite::accept_async(tcp_stream).await?;
                return Ok(Stream::Ws(ws_stream));
            }
            Protocol::Wss(listener, acceptor) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let tls_stream = acceptor.accept(tcp_stream).await?;
                let ws_stream = tokio_tungstenite::accept_async(tls_stream).await?;
                return Ok(Stream::Wss(ws_stream));
            }
            Protocol::Uds(listener) => {
                let (uds_stream, _address) = listener.accept().await?;
                return Ok(Stream::Uds(uds_stream));
            }
        }
    }
}

// Handles commands and new connections
impl Listener {
    pub async fn run_loop(&mut self) -> ! {
        // Take ownership of mpsc receiver or else tokio select will raise error.
        let mut receiver = self
            .session_receiver
            .take()
            .expect("Invalid session receiver");

        loop {
            tokio::select! {
                Ok(stream) = self.accept() => {
                    self.new_connection(stream).await;
                },

                Some(cmd) = receiver.recv() => {
                    self.route_cmd(cmd).await;
                },
            }
        }
    }

    async fn new_connection(&mut self, stream: Stream) {
        let (sender, receiver) = mpsc::channel(10);
        let connection_id = self.next_connection_id();
        let pipeline = Pipeline::new(sender, connection_id);
        self.pipelines.push(pipeline);
        let connection = Session::new(stream, connection_id, self.session_sender.clone(), receiver);
        tokio::spawn(connection.run_loop());
    }

    async fn route_cmd(&mut self, cmd: SessionToListenerCmd) {
        log::info!("Listener::route_cmd()");
        match cmd {
            SessionToListenerCmd::Publish(packet) => self.on_publish(packet).await,
            SessionToListenerCmd::Subscribe(connection_id, packet) => {
                self.on_subscribe(connection_id, packet);
            }
            SessionToListenerCmd::Unsubscribe(connection_id, packet) => {
                self.on_unsubscribe(connection_id, packet)
            }
            SessionToListenerCmd::Disconnect(connection_id) => {
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
        log::info!("Listener::on_subscribe()");
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
        log::info!("Listener::on_unsubscribe()");
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
        log::info!("Listener::on_publish()");
        let cmd = ListenerToSessionCmd::Publish(packet.clone());
        // TODO(Shaohua): Replace with a trie tree and a hash table.
        for pipeline in self.pipelines.iter_mut() {
            if topic_match(&pipeline.topics, packet.topic()) {
                if let Err(err) = pipeline.sender.send(cmd.clone()).await {
                    log::warn!("Failed to send publish packet to connection: {:?}", err);
                }
            }
        }
    }
}

fn topic_match(topics: &[SubscribedTopic], topic_str: &str) -> bool {
    for topic in topics {
        if topic.pattern.is_match(topic_str) {
            return true;
        }
    }
    false
}
