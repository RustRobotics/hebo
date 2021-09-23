// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::path::{Path, PathBuf};

/// Binding protocol types.
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Protocol {
    /// Raw Mqtt protocol, int TCP.
    #[serde(alias = "mqtt")]
    Mqtt,

    /// Raw Mqtt protocol, int TCP with TLS encryption.
    #[serde(alias = "mqtts")]
    Mqtts,

    /// Websocket protocol
    #[serde(alias = "ws")]
    Ws,

    /// Secure Websocket protocol
    #[serde(alias = "wss")]
    Wss,

    /// Unix Domain Socket
    #[serde(alias = "uds")]
    Uds,

    /// QUIC protocol
    #[serde(alias = "quic")]
    Quic,
}

/// Listener represent an unique ip/port combination and mqtt connection protocol.
#[derive(Debug, Deserialize, Clone)]
pub struct Listener {
    /// Bind the listener to a specific interface.
    ///
    /// This is useful when an interface has multiple addresses or the address may change.
    /// If used with the [ip address/host name] part of the address definition, then the
    /// bind_interface option will take priority.
    /// Example: bind_interface eth0
    ///
    /// Default is empty.
    #[serde(default = "Listener::default_bind_interface")]
    bind_interface: String,

    /// The maximum number of client connections to this listener allowed.
    ///
    /// Note that other process limits mean that unlimited connections
    /// are not really possible. Typically the default maximum number of
    /// connections possible is around 1024.
    ///
    /// Default is 0, which means unlimited connections.
    #[serde(default = "Listener::default_max_connections")]
    max_connections: usize,

    /// Binding protocol.
    ///
    /// Default is mqtt.
    #[serde(default = "Listener::default_protocol")]
    protocol: Protocol,

    /// Binding address, including domain name and port.
    ///
    /// For unix domain socket, path to socket file.
    /// Command addresses are:
    /// - 0.0.0.0:1883, for mqtt
    /// - 0.0.0.0:8883, for mqtts
    /// - 0.0.0.0:8993, for mqtt over QUIC
    /// - 0.0.0.0:8083, for mqtt over WebSocket
    /// - 0.0.0.0:8084, for mqtt over secure WebSocket
    ///
    /// Default is 0.0.0.0:1883
    #[serde(default = "Listener::default_address")]
    address: String,

    /// Url path to bind to, only used for websocket protocols.
    ///
    /// Default is None, which means do not check url path.
    #[serde(default = "Listener::default_path")]
    path: Option<String>,

    /// Path to TLS cert file.
    ///
    /// Default is None.
    #[serde(default = "Listener::default_cert_file")]
    cert_file: Option<PathBuf>,

    /// Path to TLS private key file.
    ///
    /// Default is None.
    #[serde(default = "Listener::default_key_file")]
    key_file: Option<PathBuf>,

    /// Set `username_as_client_id` to true to replace the client id that a client
    /// connected with with its username.
    ///
    /// This allows authentication to be tied to the client id, which means
    /// that it is possible to prevent one client disconnecting another
    /// by using the same client id.
    /// If a client connects with no username it will be disconnected as not
    /// authorised when this option is set to true.
    ///
    /// Default is false.
    #[serde(default = "Listener::default_username_as_client_id")]
    username_as_client_id: bool,

    /// Connection keep alive timeout in seconds.
    ///
    /// Disconnect the client if the maximium time interval is reached before
    /// receiving Control Packet from client.
    ///
    /// If client does not set keep_alive flag in ConnectPacket, this value will be
    /// used.
    ///
    /// Default is 60.
    #[serde(default = "Listener::default_keep_alive")]
    keep_alive: u64,

    /// Timeout value in seconds before receiving Connect Packet from client.
    ///
    /// The timer is triggered when client stream is connected.
    ///
    /// Default is 60s.
    #[serde(default = "Listener::default_connect_timeout")]
    connect_timeout: u64,

    /// MAY allow a Client to supply a ClientId that has a length of zero bytes.
    ///
    /// Hebo treats this as a special case and assignis a unique ClientId to that Client.
    /// if this flags is true.
    ///
    /// Or send IdentifierRejected ConnectAckPackdet if this flag is false.
    ///
    /// Default is false.
    #[serde(default = "Listener::default_allow_empty_client_id")]
    allow_empty_client_id: bool,

    /// The maximum number of QoS 1 and 2 messages currently inflight per
    /// client.
    ///
    /// This includes messages that are partway through handshakes and
    /// those that are being retried.
    ///
    /// Defaults to 20.
    #[serde(default = "Listener::default_max_inflight_messages")]
    max_inflight_messages: usize,
}

impl Listener {
    pub fn default_listeners() -> Vec<Self> {
        vec![Self::default()]
    }

    pub fn default_bind_interface() -> String {
        "".to_string()
    }

    pub const fn default_max_connections() -> usize {
        0
    }

    pub const fn default_protocol() -> Protocol {
        Protocol::Mqtt
    }

    pub fn default_address() -> String {
        "0.0.0.0:1883".to_string()
    }

    pub const fn default_path() -> Option<String> {
        None
    }

    pub const fn default_cert_file() -> Option<PathBuf> {
        None
    }

    pub const fn default_key_file() -> Option<PathBuf> {
        None
    }

    pub const fn default_username_as_client_id() -> bool {
        false
    }

    pub const fn default_keep_alive() -> u64 {
        60
    }

    pub const fn default_connect_timeout() -> u64 {
        60
    }

    pub const fn default_allow_empty_client_id() -> bool {
        false
    }

    pub const fn default_max_inflight_messages() -> usize {
        20
    }

    pub fn bind_interface(&self) -> &str {
        &self.bind_interface
    }

    pub fn max_connections(&self) -> usize {
        self.max_connections
    }

    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_ref().and_then(|s| Some(s.as_str()))
    }

    pub fn cert_file(&self) -> Option<&Path> {
        self.cert_file.as_ref().and_then(|p| Some(p.as_path()))
    }

    pub fn key_file(&self) -> Option<&Path> {
        self.key_file.as_ref().and_then(|p| Some(p.as_path()))
    }

    pub fn username_as_client_id(&self) -> bool {
        self.username_as_client_id
    }

    pub fn keep_alive(&self) -> u64 {
        self.keep_alive
    }

    pub fn connect_timeout(&self) -> u64 {
        self.connect_timeout
    }

    pub fn allow_empty_client_id(&self) -> bool {
        self.allow_empty_client_id
    }

    pub fn max_inflight_messages(&self) -> usize {
        self.max_inflight_messages
    }
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            bind_interface: Self::default_bind_interface(),
            max_connections: Self::default_max_connections(),
            protocol: Self::default_protocol(),
            address: Self::default_address(),
            path: Self::default_path(),
            cert_file: Self::default_cert_file(),
            key_file: Self::default_key_file(),
            username_as_client_id: Self::default_username_as_client_id(),
            keep_alive: Self::default_keep_alive(),
            connect_timeout: Self::default_connect_timeout(),
            allow_empty_client_id: Self::default_allow_empty_client_id(),
            max_inflight_messages: Self::default_max_inflight_messages(),
        }
    }
}
