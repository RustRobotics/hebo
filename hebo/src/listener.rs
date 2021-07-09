// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use codec::{PublishPacket, QoS, SubscribePacket, Topic, UnsubscribePacket};
use futures_util::StreamExt;
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
use crate::constants;
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
    Quic(quinn::Endpoint, quinn::Incoming),
}

impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Protocol::Mqtt(..) => "Mqtt",
            Protocol::Mqtts(..) => "Mqtts",
            Protocol::Ws(..) => "Ws",
            Protocol::Wss(..) => "Wss",
            Protocol::Uds(..) => "Uds",
            Protocol::Quic(..) => "Quic",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub struct Pipeline {
    sender: Sender<ListenerToSessionCmd>,
    topics: Vec<SubscribedTopic>,
    connection_id: ConnectionId,
}

impl Pipeline {
    pub fn new(sender: Sender<ListenerToSessionCmd>, connection_id: ConnectionId) -> Pipeline {
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
        let (session_sender, session_receiver) = mpsc::channel(constants::CHANNEL_CAPACITY);
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

            config::Protocol::Quic => {
                let mut endpoint_builder = quinn::Endpoint::builder();
                endpoint_builder.listen(quinn::ServerConfig::default());
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    // Bind this endpoint to a UDP socket on the given server address.
                    let (endpoint, incoming) = endpoint_builder.bind(&addr)?;
                    return Ok(Listener::new(
                        Protocol::Quic(endpoint, incoming),
                        storage_sender,
                        storage_receiver,
                    ));
                }
            }
        }
        Err(Error::from_string(
            ErrorKind::SocketError,
            format!("Failed to create server socket with config: {:?}", listener),
        ))
    }

    pub async fn accept(&mut self) -> Result<Stream, Error> {
        match &mut self.protocol {
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
            Protocol::Quic(_endpoint, incoming) => {
                if let Some(conn) = incoming.next().await {
                    let connection: quinn::NewConnection = conn.await?;
                    return Ok(Stream::Quic(connection));
                } else {
                    return Err(Error::new(
                        ErrorKind::SocketError,
                        "Failed to accept new quic connection",
                    ));
                }
            }
        }
    }
}

// Handles commands and new connections
impl Listener {
    pub async fn run_loop(&mut self) -> ! {
        // Take ownership of mpsc receiver or else tokio select will raise error.
        let mut session_receiver = self
            .session_receiver
            .take()
            .expect("Invalid session receiver");

        let mut storage_receiver = self
            .storage_receiver
            .take()
            .expect("Invalid storage receiver");

        loop {
            tokio::select! {
                Ok(stream) = self.accept() => {
                    self.new_connection(stream).await;
                },

                Some(cmd) = session_receiver.recv() => {
                    self.handle_session_cmd(cmd).await;
                },

                Some(cmd) = storage_receiver.recv() => {
                    self.handle_storage_cmd(cmd).await;
                }
            }
        }
    }

    async fn new_connection(&mut self, stream: Stream) {
        let (sender, receiver) = mpsc::channel(constants::CHANNEL_CAPACITY);
        let connection_id = self.next_connection_id();
        let pipeline = Pipeline::new(sender, connection_id);
        self.pipelines.push(pipeline);
        let connection = Session::new(stream, connection_id, self.session_sender.clone(), receiver);
        tokio::spawn(connection.run_loop());
    }

    async fn handle_session_cmd(&mut self, cmd: SessionToListenerCmd) {
        log::info!("Listener::handle_session_cmd()");
        match cmd {
            SessionToListenerCmd::Publish(packet) => self.on_session_publish(packet).await,
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

    async fn handle_storage_cmd(&mut self, cmd: StorageToListenerCmd) {
        log::info!("Listener::handle_storage_cmd()");
        match cmd {
            StorageToListenerCmd::Publish(packet) => self.publish_packet(packet).await,
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

    async fn on_session_publish(&mut self, packet: PublishPacket) {
        log::info!("Listener::on_session_publish()");
        let cmd = ListenerToStorageCmd::Publish(packet.clone());
        if let Err(err) = self.storage_sender.send(cmd).await {
            log::error!("Listener::on_session_publish() send failed: {:?}", err);
        }
    }

    async fn publish_packet(&mut self, packet: PublishPacket) {
        log::info!("Listener::publish_packet()");
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
