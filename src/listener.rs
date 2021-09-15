// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{
    ConnectAckPacket, ConnectPacket, ConnectReturnCode, PublishPacket, QoS, SubscribeAck,
    SubscribeAckPacket, SubscribePacket, Topic, UnsubscribePacket,
};
use futures_util::StreamExt;
use std::collections::{BTreeMap, HashMap};
use std::convert::Into;
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
    AclToListenerCmd, AuthToListenerCmd, DispatcherToListenerCmd, ListenerToAclCmd,
    ListenerToAuthCmd, ListenerToDispatcherCmd, ListenerToSessionCmd, SessionToListenerCmd,
};
use crate::config;
use crate::error::{Error, ErrorKind};
use crate::session::Session;
use crate::stream::Stream;
use crate::types::{ListenerId, SessionId};

pub const CHANNEL_CAPACITY: usize = 16;

#[derive(Debug)]
pub struct Listener {
    id: ListenerId,
    protocol: Protocol,
    listener_config: config::Listener,
    current_session_id: SessionId,
    pipelines: HashMap<SessionId, Pipeline>,
    session_ids: HashMap<SessionId, String>,
    client_ids: BTreeMap<String, SessionId>,

    session_sender: Sender<SessionToListenerCmd>,
    session_receiver: Option<Receiver<SessionToListenerCmd>>,

    dispatcher_sender: Sender<ListenerToDispatcherCmd>,
    dispatcher_receiver: Option<Receiver<DispatcherToListenerCmd>>,

    auth_sender: Sender<ListenerToAuthCmd>,
    auth_receiver: Option<Receiver<AuthToListenerCmd>>,

    acl_sender: Sender<ListenerToAclCmd>,
    acl_receiver: Option<Receiver<AclToListenerCmd>>,
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
        listener_config: config::Listener,
        // dispatcher module
        dispatcher_sender: Sender<ListenerToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToListenerCmd>,
        // auth module
        auth_sender: Sender<ListenerToAuthCmd>,
        auth_receiver: Receiver<AuthToListenerCmd>,
        // acl module
        acl_sender: Sender<ListenerToAclCmd>,
        acl_receiver: Receiver<AclToListenerCmd>,
    ) -> Self {
        let (session_sender, session_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        Listener {
            id,
            protocol,
            listener_config,
            current_session_id: 0,
            pipelines: HashMap::new(),
            session_ids: HashMap::new(),
            client_ids: BTreeMap::new(),

            session_sender,
            session_receiver: Some(session_receiver),

            dispatcher_sender,
            dispatcher_receiver: Some(dispatcher_receiver),

            auth_sender,
            auth_receiver: Some(auth_receiver),

            acl_sender,
            acl_receiver: Some(acl_receiver),
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

    fn get_cert_config(listener_config: &config::Listener) -> Result<ServerConfig, Error> {
        let cert_file = listener_config
            .cert_file
            .as_ref()
            .ok_or(Error::new(ErrorKind::CertError, "cert_file is required"))?;
        let key_file = listener_config
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
        listener_config: config::Listener,
        // dispatcher
        dispatcher_sender: Sender<ListenerToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToListenerCmd>,
        // auth
        auth_sender: Sender<ListenerToAuthCmd>,
        auth_receiver: Receiver<AuthToListenerCmd>,
        // acl
        acl_sender: Sender<ListenerToAclCmd>,
        acl_receiver: Receiver<AclToListenerCmd>,
    ) -> Result<Listener, Error> {
        let new_listener = |protocol| {
            Ok(Listener::new(
                id,
                protocol,
                listener_config.clone(),
                dispatcher_sender,
                dispatcher_receiver,
                auth_sender,
                auth_receiver,
                acl_sender,
                acl_receiver,
            ))
        };
        match listener_config.protocol {
            config::Protocol::Mqtt => {
                log::info!("bind mqtt://{}", listener_config.address);
                let addrs = listener_config.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return new_listener(Protocol::Mqtt(listener));
                }
            }
            config::Protocol::Mqtts => {
                log::info!("bind mqtts://{}", listener_config.address);
                let config = Listener::get_cert_config(&listener_config)?;
                let acceptor = TlsAcceptor::from(Arc::new(config));
                let addrs = listener_config.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return new_listener(Protocol::Mqtts(listener, acceptor));
                }
            }
            config::Protocol::Ws => {
                log::info!("bind ws://{}", listener_config.address);
                let addrs = listener_config.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return new_listener(Protocol::Ws(listener));
                }
            }
            config::Protocol::Wss => {
                log::info!("bind wss://{}", listener_config.address);
                let config = Listener::get_cert_config(&listener_config)?;
                let acceptor = TlsAcceptor::from(Arc::new(config));
                let addrs = listener_config.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return new_listener(Protocol::Wss(listener, acceptor));
                }
            }

            config::Protocol::Uds => {
                log::info!("bind uds://{}", listener_config.address);

                // Try to clean up old socket file, not that this operation is not atomic.
                if let Ok(_attr) = fs::metadata(&listener_config.address) {
                    fs::remove_file(&listener_config.address)?;
                }
                let listener = UnixListener::bind(&listener_config.address)?;
                return new_listener(Protocol::Uds(listener));
            }

            config::Protocol::Quic => {
                log::info!("bind quic://{}", listener_config.address);

                let key_file = listener_config
                    .key_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "key_file is required"))?;
                let key = fs::read(key_file)?;

                let key = if key_file.extension().map_or(false, |x| x == "der") {
                    quinn::PrivateKey::from_der(&key)?
                } else {
                    quinn::PrivateKey::from_pem(&key)?
                };

                let cert_file = listener_config
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
                let addrs = listener_config.address.to_socket_addrs()?;
                for addr in addrs {
                    // Bind this endpoint to a UDP socket on the given server address.
                    let (endpoint, incoming) = endpoint_builder.bind(&addr)?;
                    return new_listener(Protocol::Quic(endpoint, incoming));
                }
            }
        }
        Err(Error::from_string(
            ErrorKind::SocketError,
            format!(
                "Failed to create server socket with config: {:?}",
                &listener_config
            ),
        ))
    }

    async fn accept(&mut self) -> Result<Stream, Error> {
        use tokio_tungstenite::tungstenite::handshake::server as ws_server;
        let listener_path = self.listener_config.path.as_ref();
        let check_ws_path = |request: &ws_server::Request,
                             response: ws_server::Response|
         -> Result<ws_server::Response, ws_server::ErrorResponse> {
            let path = request.uri().path();
            if listener_path.is_none() || path == listener_path.unwrap() {
                return Ok(response);
            } else {
                let builder = http::Response::builder().status(http::StatusCode::NOT_FOUND);
                let resp = builder.body(None);
                // TODO(Shaohua): Remove unwrap()
                let resp = resp.unwrap();
                return Err(resp);
            }
        };

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
                let ws_stream = if listener_path.is_none() {
                    tokio_tungstenite::accept_async(tcp_stream).await?
                } else {
                    tokio_tungstenite::accept_hdr_async(tcp_stream, check_ws_path).await?
                };
                return Ok(Stream::Ws(ws_stream));
            }
            Protocol::Wss(listener, acceptor) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let tls_stream = acceptor.accept(tcp_stream).await?;
                let ws_stream = if listener_path.is_none() {
                    tokio_tungstenite::accept_async(tls_stream).await?
                } else {
                    tokio_tungstenite::accept_hdr_async(tls_stream, check_ws_path).await?
                };
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
        let mut auth_receiver = self.auth_receiver.take().expect("Invalid auth receiver");
        let mut acl_receiver = self.acl_receiver.take().expect("Invalid acl receiver");

        loop {
            tokio::select! {
                Ok(stream) = self.accept() => {
                    self.new_connection(stream).await;
                },

                Some(cmd) = session_receiver.recv() => {
                    if let Err(err) = self.handle_session_cmd(cmd).await {
                        log::error!("handle session cmd failed: {:?}", err);
                    }
                },

                Some(cmd) = dispatcher_receiver.recv() => {
                    self.handle_dispatcher_cmd(cmd).await;
                }

                Some(cmd) = auth_receiver.recv() => {
                    if let Err(err) = self.handle_auth_cmd(cmd).await {
                        log::error!("handle auth cmd failed: {:?}", err);
                    }
                }

                Some(cmd) = acl_receiver.recv() => {
                    if let Err(err) = self.handle_acl_cmd(cmd).await {
                        log::error!("handle acl cmd failed: {:?}", err);
                    }
                }
            }
        }
    }

    async fn new_connection(&mut self, stream: Stream) {
        let (sender, receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let session_id = self.next_session_id();
        let pipeline = Pipeline::new(sender, session_id);
        self.pipelines.insert(session_id, pipeline);
        let session = Session::new(session_id, stream, self.session_sender.clone(), receiver);
        tokio::spawn(session.run_loop());

        if let Err(err) = self
            .dispatcher_sender
            .send(ListenerToDispatcherCmd::SessionAdded(self.id))
            .await
        {
            log::error!("Failed to send NewSession cmd: {:?}", err);
        }
    }

    async fn handle_session_cmd(&mut self, cmd: SessionToListenerCmd) -> Result<(), Error> {
        log::info!("Listener::handle_session_cmd()");
        match cmd {
            SessionToListenerCmd::Connect(session_id, packet) => {
                self.on_session_connect(session_id, packet).await
            }
            SessionToListenerCmd::Publish(packet) => self.on_session_publish(packet).await,
            SessionToListenerCmd::Subscribe(session_id, packet) => {
                self.on_session_subscribe(session_id, packet).await
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

    async fn on_session_connect(
        &mut self,
        session_id: SessionId,
        packet: ConnectPacket,
    ) -> Result<(), Error> {
        log::info!("Listener::on_session_connect()");
        // If client id already exists, notify session to send disconnect packet.
        if self.client_ids.get(packet.client_id()).is_some() {
            let ack_packet = ConnectAckPacket::new(false, ConnectReturnCode::IdentifierRejected);
            let cmd = ListenerToSessionCmd::ConnectAck(ack_packet);
            if let Some(pipeline) = self.pipelines.get(&session_id) {
                return pipeline.sender.send(cmd).await.map_err(Into::into);
            } else {
                return Err(Error::session_error(session_id));
            }
        }

        self.session_ids
            .insert(session_id, packet.client_id().to_string());

        // Send request to auth app.
        self.auth_sender
            .send(ListenerToAuthCmd::RequestAuth(
                self.id,
                session_id,
                packet.username().to_string(),
                packet.password().to_vec(),
            ))
            .await
            .map_err(Into::into)
    }

    async fn on_session_disconnect(&mut self, session_id: SessionId) -> Result<(), Error> {
        log::info!("Listener::on_session_disconnect()");
        // Delete session info
        if self.pipelines.remove(&session_id).is_none() {
            log::error!("Failed to remove pipeline with session id: {}", session_id);
        }
        if let Some(client_id) = self.session_ids.remove(&session_id) {
            if self.client_ids.remove(&client_id).is_none() {
                log::error!("Failed to remove client id: {}", client_id);
            }
        } else {
            log::error!("Failed to remove session id: {}", session_id);
        }

        self.dispatcher_sender
            .send(ListenerToDispatcherCmd::SessionRemoved(self.id))
            .await
            .map_err(Into::into)
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
            return Err(Error::session_error(session_id));
        }

        // TODO(Shaohua): Send notify to dispatcher.
        Ok(())
    }

    async fn on_session_unsubscribe(
        &mut self,
        session_id: SessionId,
        packet: UnsubscribePacket,
    ) -> Result<(), Error> {
        // Remove topic from sub tree.
        for (_, pipeline) in self.pipelines.iter_mut() {
            if pipeline.session_id == session_id {
                pipeline
                    .topics
                    .retain(|ref topic| !packet.topics().any(|t| t == topic.pattern.topic()));
            }
            break;
        }

        // Send subRemoved to dispatcher.
        self.dispatcher_sender
            .send(ListenerToDispatcherCmd::SubscriptionsRemoved(self.id))
            .await
            .map_err(Into::into)
    }

    async fn on_session_publish(&mut self, packet: PublishPacket) -> Result<(), Error> {
        let cmd = ListenerToDispatcherCmd::Publish(packet.clone());
        self.dispatcher_sender.send(cmd).await.map_err(Into::into)
    }

    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToListenerCmd) {
        match cmd {
            DispatcherToListenerCmd::Publish(packet) => self.on_dispatcher_publish(packet).await,
        }
    }

    async fn on_dispatcher_publish(&mut self, packet: PublishPacket) {
        let cmd = ListenerToSessionCmd::Publish(packet.clone());
        // TODO(Shaohua): Replace with a trie tree and a hash table.

        // TODO(Shaohua): Handle errors
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

    async fn handle_auth_cmd(&mut self, cmd: AuthToListenerCmd) -> Result<(), Error> {
        match cmd {
            AuthToListenerCmd::ResponseAuth(session_id, access_granted) => {
                self.on_auth_response(session_id, access_granted).await
            }
        }
    }

    async fn on_auth_response(
        &mut self,
        session_id: SessionId,
        access_granted: bool,
    ) -> Result<(), Error> {
        let ack_packet = if access_granted {
            ConnectAckPacket::new(true, ConnectReturnCode::Accepted)
        } else {
            ConnectAckPacket::new(false, ConnectReturnCode::Unauthorized)
        };
        let cmd = ListenerToSessionCmd::ConnectAck(ack_packet);

        if access_granted {
            // Add client id to cache.
            if let Some(client_id) = self.session_ids.get(&session_id) {
                self.client_ids.insert(client_id.to_string(), session_id);
            } else {
                log::error!(
                    "listener: Failed to find client id with session: {}",
                    session_id
                );
            }
        }

        if let Some(pipeline) = self.pipelines.get(&session_id) {
            pipeline.sender.send(cmd).await.map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }

    /// Acl cmd handler.
    async fn handle_acl_cmd(&mut self, cmd: AclToListenerCmd) -> Result<(), Error> {
        log::info!("Handle acl cmd: {:?}", cmd);
        Ok(())
    }
}

// TODO(Shaohua): Move to dispatcher app.
fn topic_match(topics: &[SubscribedTopic], topic_str: &str) -> bool {
    for topic in topics {
        if topic.pattern.is_match(topic_str) {
            return true;
        }
    }
    false
}
