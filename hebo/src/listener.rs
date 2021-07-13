// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use codec::{PublishPacket, QoS, SubscribePacket, Topic, UnsubscribePacket};
use futures_util::StreamExt;
use std::fmt;
use std::fs::{self, File};
use std::io::BufReader;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::sync::Arc;
use tokio::net::{TcpListener, UnixListener};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_rustls::rustls::internal::pemfile;
use tokio_rustls::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;

use crate::commands::{
    ConnectionId, DispatcherToListenerCmd, ListenerToDispatcherCmd, ListenerToSessionCmd,
    SessionToListenerCmd,
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

    dispatcher_sender: Sender<ListenerToDispatcherCmd>,
    dispatcher_receiver: Option<Receiver<DispatcherToListenerCmd>>,
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
        dispatcher_sender: Sender<ListenerToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToListenerCmd>,
    ) -> Self {
        let (session_sender, session_receiver) = mpsc::channel(constants::CHANNEL_CAPACITY);
        Listener {
            protocol,
            current_connection_id: 0,
            pipelines: Vec::new(),
            session_sender,
            session_receiver: Some(session_receiver),

            dispatcher_sender,
            dispatcher_receiver: Some(dispatcher_receiver),
        }
    }

    fn load_certs(path: &Path) -> Result<Vec<Certificate>, Error> {
        pemfile::certs(&mut BufReader::new(File::open(path)?)).map_err(|err| {
            Error::from_string(
                ErrorKind::CertError,
                format!("Failed to load cert file at {:?}, got: {:?}", path, err),
            )
        })
    }

    fn load_keys(path: &Path) -> Result<Vec<PrivateKey>, Error> {
        if let Ok(keys) = pemfile::rsa_private_keys(&mut BufReader::new(File::open(path)?)) {
            if !keys.is_empty() {
                return Ok(keys);
            }
        }
        if let Ok(keys) = pemfile::pkcs8_private_keys(&mut BufReader::new(File::open(path)?)) {
            if !keys.is_empty() {
                return Ok(keys);
            }
        }

        Err(Error::from_string(
            ErrorKind::CertError,
            format!("Failed to load key file at {:?}", path),
        ))
    }

    fn get_cert_config(listener: &config::Listener) -> Result<ServerConfig, Error> {
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
        Ok(config)
    }

    pub async fn bind(
        listener: &config::Listener,
        dispatcher_sender: Sender<ListenerToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToListenerCmd>,
    ) -> Result<Listener, Error> {
        match listener.protocol {
            config::Protocol::Mqtt => {
                log::info!("bind mqtt://{}", listener.address);
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(
                        Protocol::Mqtt(listener),
                        dispatcher_sender,
                        dispatcher_receiver,
                    ));
                }
            }
            config::Protocol::Mqtts => {
                log::info!("bind mqtts://{}", listener.address);
                let config = Listener::get_cert_config(listener)?;
                let acceptor = TlsAcceptor::from(Arc::new(config));
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(
                        Protocol::Mqtts(listener, acceptor),
                        dispatcher_sender,
                        dispatcher_receiver,
                    ));
                }
            }
            config::Protocol::Ws => {
                log::info!("bind ws://{}", listener.address);
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(
                        Protocol::Ws(listener),
                        dispatcher_sender,
                        dispatcher_receiver,
                    ));
                }
            }
            config::Protocol::Wss => {
                log::info!("bind wss://{}", listener.address);
                let config = Listener::get_cert_config(listener)?;
                let acceptor = TlsAcceptor::from(Arc::new(config));
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::new(
                        Protocol::Wss(listener, acceptor),
                        dispatcher_sender,
                        dispatcher_receiver,
                    ));
                }
            }

            config::Protocol::Uds => {
                log::info!("bind uds://{}", listener.address);

                // Try to clean up old socket file, not that this operation is not atomic.
                if let Ok(_attr) = fs::metadata(&listener.address) {
                    fs::remove_file(&listener.address)?;
                }
                let listener = UnixListener::bind(&listener.address)?;
                return Ok(Listener::new(
                    Protocol::Uds(listener),
                    dispatcher_sender,
                    dispatcher_receiver,
                ));
            }

            config::Protocol::Quic => {
                log::info!("bind quic://{}", listener.address);

                let key_file = listener
                    .key_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "key_file is required"))?;
                let key = fs::read(key_file)?;

                let key = if key_file.extension().map_or(false, |x| x == "der") {
                    quinn::PrivateKey::from_der(&key)?
                } else {
                    quinn::PrivateKey::from_pem(&key)?
                };

                let cert_file = listener
                    .cert_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "cert_file is required"))?;
                let cert_chain = fs::read(cert_file)?;

                let cert_chain = if cert_file.extension().map_or(false, |x| x == "der") {
                    quinn::CertificateChain::from_certs(Some(
                        quinn::Certificate::from_der(&cert_chain).map_err(|err| {
                            Error::from_string(
                                ErrorKind::CertError,
                                format!("cert_file {:?} is invalid, err: {:?}", &cert_file, err),
                            )
                        })?,
                    ))
                } else {
                    quinn::CertificateChain::from_pem(&cert_chain).map_err(|err| {
                        Error::from_string(
                            ErrorKind::CertError,
                            format!("cert_file {:?} is invalid, err: {:?}", &cert_file, err),
                        )
                    })?
                };

                let mut config_builder = quinn::ServerConfigBuilder::default();
                config_builder.certificate(cert_chain, key)?;

                let mut endpoint_builder = quinn::Endpoint::builder();
                endpoint_builder.listen(config_builder.build());
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    // Bind this endpoint to a UDP socket on the given server address.
                    let (endpoint, incoming) = endpoint_builder.bind(&addr)?;
                    return Ok(Listener::new(
                        Protocol::Quic(endpoint, incoming),
                        dispatcher_sender,
                        dispatcher_receiver,
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

        let mut dispatcher_receiver = self
            .dispatcher_receiver
            .take()
            .expect("Invalid dispatcher receiver");

        loop {
            tokio::select! {
                Ok(stream) = self.accept() => {
                    self.new_connection(stream).await;
                },

                Some(cmd) = session_receiver.recv() => {
                    self.handle_session_cmd(cmd).await;
                },

                Some(cmd) = dispatcher_receiver.recv() => {
                    self.handle_dispatcher_cmd(cmd).await;
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

    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToListenerCmd) {
        log::info!("Listener::handle_dispatcher_cmd()");
        match cmd {
            DispatcherToListenerCmd::Publish(packet) => self.publish_packet(packet).await,
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
        let cmd = ListenerToDispatcherCmd::Publish(packet.clone());
        if let Err(err) = self.dispatcher_sender.send(cmd).await {
            log::error!(
                "Failed to send publish packet from listener to dispatcher : {:?}",
                err
            );
        }
    }

    async fn publish_packet(&mut self, packet: PublishPacket) {
        log::info!("Listener::publish_packet()");
        let cmd = ListenerToSessionCmd::Publish(packet.clone());
        // TODO(Shaohua): Replace with a trie tree and a hash table.
        for pipeline in self.pipelines.iter_mut() {
            if topic_match(&pipeline.topics, packet.topic()) {
                if let Err(err) = pipeline.sender.send(cmd.clone()).await {
                    log::warn!(
                        "Failed to send publish packet from listener to session: {:?}",
                        err
                    );
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
