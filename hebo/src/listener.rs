// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use codec::{
    ConnectAckPacket, ConnectPacket, ConnectReturnCode, PublishPacket, QoS, SubscribeAck,
    SubscribeAckPacket, SubscribePacket, Topic, UnsubscribePacket,
};
use futures_util::StreamExt;
use std::collections::HashMap;
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
    DispatcherToListenerCmd, ListenerId, ListenerToDispatcherCmd, ListenerToSessionCmd, SessionId,
    SessionToListenerCmd,
};
use crate::config;
use crate::constants;
use crate::error::{Error, ErrorKind};
use crate::session::Session;
use crate::stream::Stream;

#[derive(Debug)]
pub struct Listener {
    id: ListenerId,
    protocol: Protocol,
    current_session_id: SessionId,
    pipelines: HashMap<SessionId, Pipeline>,
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
    session_id: SessionId,
}

impl Pipeline {
    pub fn new(sender: Sender<ListenerToSessionCmd>, session_id: SessionId) -> Pipeline {
        Pipeline {
            sender,
            topics: Vec::new(),
            session_id,
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
        id: ListenerId,
        protocol: Protocol,
        dispatcher_sender: Sender<ListenerToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToListenerCmd>,
    ) -> Self {
        let (session_sender, session_receiver) = mpsc::channel(constants::CHANNEL_CAPACITY);
        Listener {
            id,
            protocol,
            current_session_id: 0,
            pipelines: HashMap::new(),
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
        id: u32,
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
                        id,
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
                        id,
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
                        id,
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
                        id,
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
                    id,
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
                        id,
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

    async fn accept(&mut self) -> Result<Stream, Error> {
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
        let session_id = self.next_session_id();
        let pipeline = Pipeline::new(sender, session_id);
        self.pipelines.insert(session_id, pipeline);
        let connection = Session::new(session_id, stream, self.session_sender.clone(), receiver);
        tokio::spawn(connection.run_loop());

        if let Err(err) = self
            .dispatcher_sender
            .send(ListenerToDispatcherCmd::SessionAdded(self.id))
            .await
        {
            log::error!("Failed to send NewSession cmd: {:?}", err);
        }
    }

    async fn handle_session_cmd(&mut self, cmd: SessionToListenerCmd) {
        log::info!("Listener::handle_session_cmd()");
        match cmd {
            SessionToListenerCmd::Connect(session_id, packet) => {
                self.on_session_connect(session_id, packet).await
            }
            SessionToListenerCmd::Publish(packet) => self.on_session_publish(packet).await,
            SessionToListenerCmd::Subscribe(session_id, packet) => {
                self.on_session_subscribe(session_id, packet).await.unwrap()
            }
            SessionToListenerCmd::Unsubscribe(session_id, packet) => {
                self.on_session_unsubscribe(session_id, packet).await
            }
            SessionToListenerCmd::Disconnect(session_id) => {
                self.on_session_disconnect(session_id).await
            }
        }
    }

    fn next_session_id(&mut self) -> SessionId {
        self.current_session_id += 1;
        self.current_session_id
    }

    async fn on_session_connect(&mut self, session_id: SessionId, packet: ConnectPacket) {
        log::info!("Listener::on_session_connect()");
        // TODO(Shaohua): Check auth

        let ack_packet = ConnectAckPacket::new(true, ConnectReturnCode::Accepted);
        let cmd = ListenerToSessionCmd::ConnectAck(ack_packet);
        if let Some(pipeline) = self.pipelines.get(&session_id) {
            if let Err(err) = pipeline.sender.send(cmd).await {
                log::warn!(
                    "Failed to send connect ackpacket from listener to session: {:?}",
                    err
                );
            }
        } else {
            log::error!("Failed to find pipeline with id: {}", session_id);
        }
    }

    async fn on_session_disconnect(&mut self, session_id: SessionId) {
        log::info!("Listener::on_session_disconnect()");
        if self.pipelines.remove(&session_id).is_none() {
            log::error!("Failed to remove pipeline with session id: {}", session_id);
            return;
        }
        if let Err(err) = self
            .dispatcher_sender
            .send(ListenerToDispatcherCmd::SessionRemoved(self.id))
            .await
        {
            log::error!("Failed to send session removed cmd: {:?}", err);
        }
    }

    async fn on_session_subscribe(
        &mut self,
        session_id: SessionId,
        packet: SubscribePacket,
    ) -> Result<(), Error> {
        log::info!("Listener::on_session_subscribe()");

        // TODO(Shaohua): Check auth.

        if let Some(pipeline) = self.pipelines.get_mut(&session_id) {
            let mut ack_vec = vec![];
            for topic in packet.topics() {
                // Update sub tree
                match Topic::parse(topic.topic()) {
                    Ok(pattern) => {
                        ack_vec.push(SubscribeAck::QoS(topic.qos()));
                        pipeline.topics.push(SubscribedTopic {
                            pattern,
                            qos: topic.qos(),
                        });
                    }
                    Err(err) => {
                        log::error!("Invalid sub topic: {:?}, err: {:?}", topic, err);
                        ack_vec.push(SubscribeAck::Failed);
                    }
                }
            }

            // Send subscribe ack to session.
            let ack_packet = SubscribeAckPacket::with_vec(ack_vec, packet.packet_id());
            pipeline
                .sender
                .send(ListenerToSessionCmd::SubscribeAck(ack_packet))
                .await?;
        } else {
            log::error!("Failed to find pipeline with id: {}", session_id);
        }

        // TODO(Shaohua): Send notify to dispatcher.
        Ok(())
    }

    async fn on_session_unsubscribe(&mut self, session_id: SessionId, packet: UnsubscribePacket) {
        log::info!("Listener::on_session_unsubscribe()");
        for (_, pipeline) in self.pipelines.iter_mut() {
            if pipeline.session_id == session_id {
                pipeline
                    .topics
                    .retain(|ref topic| !packet.topics().any(|t| t == topic.pattern.topic()));
            }
            break;
        }
        if let Err(err) = self
            .dispatcher_sender
            .send(ListenerToDispatcherCmd::SubscriptionsRemoved(self.id))
            .await
        {
            log::error!("Failed to send SubscriptionsRemoved cmd: {:?}", err);
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

    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToListenerCmd) {
        log::info!("Listener::handle_dispatcher_cmd()");
        match cmd {
            DispatcherToListenerCmd::Publish(packet) => self.publish_packet(packet).await,
        }
    }

    async fn publish_packet(&mut self, packet: PublishPacket) {
        log::info!("Listener::publish_packet()");
        let cmd = ListenerToSessionCmd::Publish(packet.clone());
        // TODO(Shaohua): Replace with a trie tree and a hash table.

        for (_, pipeline) in self.pipelines.iter_mut() {
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
