// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Initialize Listener

use futures_util::StreamExt;
use std::collections::{BTreeMap, HashMap};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_rustls::rustls::internal::pemfile;
use tokio_rustls::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;

use super::Listener;
use super::Protocol;
use super::CHANNEL_CAPACITY;
use crate::commands::{
    AclToListenerCmd, AuthToListenerCmd, DispatcherToListenerCmd, ListenerToAclCmd,
    ListenerToAuthCmd, ListenerToDispatcherCmd,
};
use crate::config;
use crate::error::{Error, ErrorKind};
use crate::socket::{new_tcp_listener, new_udp_socket};
use crate::stream::Stream;
use crate::types::ListenerId;

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
            config: listener_config,
            current_session_id: 0,

            session_senders: HashMap::new(),
            session_ids: HashMap::new(),
            client_ids: BTreeMap::new(),
            connecting_sessions: HashMap::new(),

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
            .cert_file()
            .ok_or(Error::new(ErrorKind::CertError, "cert_file is required"))?;
        let key_file = listener_config
            .key_file()
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
        let device = listener_config.bind_device();
        let address = listener_config.address();

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
        match listener_config.protocol() {
            config::Protocol::Mqtt => {
                log::info!("bind mqtt://{}", address);
                let listener = new_tcp_listener(address, device).await?;
                return new_listener(Protocol::Mqtt(listener));
            }
            config::Protocol::Mqtts => {
                log::info!("bind mqtts://{}", address);
                let config = Listener::get_cert_config(&listener_config)?;
                let acceptor = TlsAcceptor::from(Arc::new(config));
                let listener = new_tcp_listener(address, device).await?;
                return new_listener(Protocol::Mqtts(listener, acceptor));
            }
            config::Protocol::Ws => {
                log::info!("bind ws://{}", address);
                let listener = new_tcp_listener(address, device).await?;
                return new_listener(Protocol::Ws(listener));
            }
            config::Protocol::Wss => {
                log::info!("bind wss://{}", address);
                let config = Listener::get_cert_config(&listener_config)?;
                let acceptor = TlsAcceptor::from(Arc::new(config));
                let listener = new_tcp_listener(address, device).await?;
                return new_listener(Protocol::Wss(listener, acceptor));
            }

            config::Protocol::Uds => {
                log::info!("bind uds://{}", address);

                // Try to clean up old socket file, not that this operation is not atomic.
                if let Ok(_attr) = fs::metadata(address) {
                    fs::remove_file(address)?;
                }
                let listener = UnixListener::bind(address)?;
                return new_listener(Protocol::Uds(listener));
            }

            config::Protocol::Quic => {
                log::info!("bind quic://{}", address);

                let key_file = listener_config
                    .key_file()
                    .ok_or(Error::new(ErrorKind::CertError, "key_file is required"))?;
                let key = fs::read(key_file)?;

                let key = if key_file.extension().map_or(false, |x| x == "der") {
                    quinn::PrivateKey::from_der(&key)?
                } else {
                    quinn::PrivateKey::from_pem(&key)?
                };

                let cert_file = listener_config
                    .cert_file()
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
                // Bind this endpoint to a UDP socket on the given server address.
                let udp_socket = new_udp_socket(address, device)?;
                let (endpoint, incoming) = endpoint_builder.with_socket(udp_socket)?;
                return new_listener(Protocol::Quic(endpoint, incoming));
            }
        }
    }

    pub(super) async fn accept(&mut self) -> Result<Stream, Error> {
        use tokio_tungstenite::tungstenite::handshake::server as ws_server;
        let listener_path = self.config.path();
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
