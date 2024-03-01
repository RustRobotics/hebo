// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Initialize Listener

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{self, File};
use std::io::BufReader;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
#[cfg(unix)]
use tokio::net::UnixListener;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_rustls::{rustls, TlsAcceptor};

use super::Listener;
use super::Protocol;
use super::CHANNEL_CAPACITY;
use crate::commands::{
    AclToListenerCmd, AuthToListenerCmd, DispatcherToListenerCmd, ListenerToAclCmd,
    ListenerToAuthCmd, ListenerToDispatcherCmd,
};
use crate::config;
use crate::error::{Error, ErrorKind};
use crate::socket::new_tcp_listener;
use crate::stream::Stream;
use crate::types::ListenerId;

impl Listener {
    #[allow(clippy::too_many_arguments)]
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
        Self {
            id,
            protocol,
            config: listener_config,
            current_session_id: 0,

            session_senders: HashMap::new(),
            client_ids: BTreeMap::new(),

            connecting_sessions: HashSet::new(),

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

    fn load_certs(path: &Path) -> Result<Vec<rustls::Certificate>, Error> {
        let items =
            rustls_pemfile::certs(&mut BufReader::new(File::open(path)?)).map_err(|err| {
                Error::from_string(
                    ErrorKind::CertError,
                    format!("Failed to load cert file at {path:?}, got: {err:?}"),
                )
            })?;
        Ok(items.into_iter().map(rustls::Certificate).collect())
    }

    fn load_keys(path: &Path) -> Result<Vec<rustls::PrivateKey>, Error> {
        if let Ok(keys) = rustls_pemfile::rsa_private_keys(&mut BufReader::new(File::open(path)?)) {
            if !keys.is_empty() {
                return Ok(keys.into_iter().map(rustls::PrivateKey).collect());
            }
        }
        if let Ok(keys) = rustls_pemfile::pkcs8_private_keys(&mut BufReader::new(File::open(path)?))
        {
            if !keys.is_empty() {
                return Ok(keys.into_iter().map(rustls::PrivateKey).collect());
            }
        }

        Err(Error::from_string(
            ErrorKind::CertError,
            format!("Failed to load key file at {path:?}"),
        ))
    }

    fn get_cert_config(listener_config: &config::Listener) -> Result<rustls::ServerConfig, Error> {
        let cert_file = listener_config
            .cert_file()
            .ok_or_else(|| Error::new(ErrorKind::CertError, "cert_file is required"))?;
        let key_file = listener_config
            .key_file()
            .ok_or_else(|| Error::new(ErrorKind::CertError, "key_file is required"))?;

        let certs = Self::load_certs(cert_file)?;
        let mut keys = Self::load_keys(key_file)?;

        rustls::ServerConfig::builder()
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_protocol_versions(rustls::ALL_VERSIONS)
            .map_err(|err| {
                Error::from_string(
                    ErrorKind::CertError,
                    format!("Failed to init ConfigBuilder, got {err:?}"),
                )
            })?
            .with_no_client_auth()
            .with_single_cert(certs, keys.remove(0))
            .map_err(|err| {
                Error::from_string(
                    ErrorKind::CertError,
                    format!("Failed to init ServerConfig, got {err:?}"),
                )
            })
    }

    /// Bind to specific socket address.
    ///
    /// # Errors
    ///
    /// Returns error if:
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::too_many_arguments)]
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
    ) -> Result<Self, Error> {
        let device = listener_config.bind_device();
        let address = listener_config.address();

        let new_listener = |protocol| {
            Ok(Self::new(
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
                new_listener(Protocol::Mqtt(listener))
            }
            config::Protocol::Mqtts => {
                log::info!("bind mqtts://{}", address);
                let config = Self::get_cert_config(&listener_config)?;
                let acceptor = TlsAcceptor::from(Arc::new(config));
                let listener = new_tcp_listener(address, device).await?;
                new_listener(Protocol::Mqtts(listener, acceptor))
            }
            config::Protocol::Ws => {
                log::info!("bind ws://{}", address);
                let listener = new_tcp_listener(address, device).await?;
                new_listener(Protocol::Ws(listener))
            }
            config::Protocol::Wss => {
                log::info!("bind wss://{}", address);
                let config = Self::get_cert_config(&listener_config)?;
                let acceptor = TlsAcceptor::from(Arc::new(config));
                let listener = new_tcp_listener(address, device).await?;
                new_listener(Protocol::Wss(listener, acceptor))
            }

            #[cfg(unix)]
            config::Protocol::Uds => {
                log::info!("bind uds://{}", address);

                // Try to clean up old socket file, not that this operation is not atomic.
                if let Ok(_attr) = fs::metadata(address) {
                    fs::remove_file(address)?;
                }
                let listener = UnixListener::bind(address)?;
                new_listener(Protocol::Uds(listener))
            }

            config::Protocol::Quic => {
                log::info!("bind quic://{}", address);

                let key_file = listener_config
                    .key_file()
                    .ok_or_else(|| Error::new(ErrorKind::CertError, "key_file is required"))?;
                let key = fs::read(key_file)?;
                let key = rustls::PrivateKey(key);

                let cert_file = listener_config
                    .cert_file()
                    .ok_or_else(|| Error::new(ErrorKind::CertError, "cert_file is required"))?;
                let cert = fs::read(cert_file)?;
                let cert = rustls::Certificate(cert);

                let server_config = quinn::ServerConfig::with_single_cert(vec![cert], key)?;

                // TODO(Shaohua): Bind this endpoint to a UDP socket on the given server address.
                //let udp_socket = new_udp_socket(address, device)?;
                let sock_addr: SocketAddr = address.parse()?;
                let endpoint = quinn::Endpoint::server(server_config, sock_addr)?;
                new_listener(Protocol::Quic(endpoint))
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
            }
            let builder = http::Response::builder().status(http::StatusCode::NOT_FOUND);
            let resp = builder.body(None);
            // TODO(Shaohua): Remove unwrap()
            let resp = resp.unwrap();
            Err(resp)
        };

        match &mut self.protocol {
            Protocol::Mqtt(listener) => {
                let (tcp_stream, _address) = listener.accept().await?;
                Ok(Stream::Mqtt(tcp_stream))
            }
            Protocol::Mqtts(listener, acceptor) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let tls_stream = acceptor.accept(tcp_stream).await?;
                Ok(Stream::Mqtts(Box::new(tls_stream)))
            }
            Protocol::Ws(listener) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let ws_stream = if listener_path.is_none() {
                    tokio_tungstenite::accept_async(tcp_stream).await?
                } else {
                    tokio_tungstenite::accept_hdr_async(tcp_stream, check_ws_path).await?
                };
                Ok(Stream::Ws(Box::new(ws_stream)))
            }
            Protocol::Wss(listener, acceptor) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let tls_stream = acceptor.accept(tcp_stream).await?;
                let ws_stream = if listener_path.is_none() {
                    tokio_tungstenite::accept_async(tls_stream).await?
                } else {
                    tokio_tungstenite::accept_hdr_async(tls_stream, check_ws_path).await?
                };
                Ok(Stream::Wss(Box::new(ws_stream)))
            }
            #[cfg(unix)]
            Protocol::Uds(listener) => {
                let (uds_stream, _address) = listener.accept().await?;
                Ok(Stream::Uds(uds_stream))
            }
            Protocol::Quic(endpoint) => {
                if let Some(conn) = endpoint.accept().await {
                    let connection: quinn::Connection = conn.await?;
                    return Ok(Stream::Quic(connection));
                }
                Err(Error::new(
                    ErrorKind::SocketError,
                    "Failed to accept new quic connection",
                ))
            }
        }
    }
}
