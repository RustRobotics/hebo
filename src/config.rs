// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::path::{Path, PathBuf};
use std::time::Duration;

use codec::QoS;

/// Server main config.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "General::default")]
    general: General,

    #[serde(default = "Listener::default_listeners")]
    listeners: Vec<Listener>,

    #[serde(default = "Security::default")]
    security: Security,

    #[serde(default = "Storage::default")]
    storage: Storage,

    #[serde(default = "Log::default")]
    log: Log,

    #[serde(default = "Dashboard::default")]
    dashboard: Dashboard,
}

impl Config {
    pub fn general(&self) -> &General {
        &self.general
    }

    pub fn listeners(&self) -> &[Listener] {
        &self.listeners
    }

    pub fn security(&self) -> &Security {
        &self.security
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn dashboard(&self) -> &Dashboard {
        &self.dashboard
    }
}

/// General section in config.
#[derive(Debug, Deserialize, Clone)]
pub struct General {
    /// Time interval to send $SYS messages in seconds.
    ///
    /// Set to 0 to disable $SYS messages.
    ///
    /// Default is 3s.
    #[serde(default = "General::default_sys_interval")]
    sys_interval: u64,

    /// When run as root, drop privileges to this user.
    ///
    /// Default user is "hebo".
    #[serde(default = "General::default_user")]
    user: String,

    /// Write process id to a file. A blank string means a pid file shouldn't be written.
    ///
    /// Default is `/var/run/hebo.pid`.
    #[serde(default = "General::default_pid_file")]
    pid_file: PathBuf,

    /// Disable Nagle's algorithm on client sockets.
    ///
    /// This has the effect of reducing latency of individual messages
    /// at the potential cost of increasing the number of packets being sent.
    ///
    /// Default is false.
    #[serde(default = "General::default_no_delay")]
    no_delay: bool,

    /// Set maximium size for publish message payload.
    ///
    /// Received messages that exceed this size will not be accepted by the broker.
    /// MQTT imposes a maximum payload size of 268435455 bytes.
    ///
    /// Default value is 0, which means that all valid MQTT messages are accepted.
    #[serde(default = "General::default_message_size_limit")]
    message_size_limit: usize,

    /// For MQTT v5 clients, it is possible to have the server send a "server keep_alive" value
    /// that will override the keep_alive value set by the client.
    ///
    /// This is intended to be used as a mechanism to say that the server will disconnect the client
    /// earlier than it anticipated, and that the client should use the new keep_alive value.
    /// The `max_keep_alive` option allows you to specify that clients may only
    /// connect with keep_alive less than or equal to this value, otherwise they will be
    /// sent a server keep_alive telling them to use `max_keep_alive`.
    /// This only applies to MQTT v5 clients. The maximum value allowable is 65535. Do not set below 10.
    ///
    /// Default value is 65535.
    #[serde(default = "General::default_max_keep_alive")]
    max_keep_alive: u64,

    /// Set the maximum QoS supported.
    ///
    /// Clients publishing at a QoS higher than specified here will be disconnected.
    /// Available values are 0, 1 and 2.
    ///
    /// Default is 2.
    #[serde(default = "General::default_max_qos")]
    max_qos: QoS,

    /// For MQTT v5 clients, it is possible to have the server send a "maximum packet size" value
    /// that will instruct the client it will not accept MQTT packets with size
    /// greater than max_packet_size bytes.
    ///
    /// This applies to the full MQTT packet, not just the payload. Setting this option
    /// to a positive value will set the maximum packet size to that number of bytes.
    /// If a client sends a packet which is larger than this value, it will be disconnected.
    /// This applies to all clients regardless of the protocol version they are using, but v3.1.1
    /// and earlier clients will of course not have received the maximum packet size information.
    /// Setting below 20 bytes is forbidden because it is likely to interfere with ordinary client operation,
    /// even with very small payloads.
    ///
    /// Defaults is 0, which means no limit.
    #[serde(default = "General::default_max_packet_size")]
    max_packet_size: usize,
    //pub max_queued_messages: usize,
    //pub max_queued_bytes: usize,
}

impl General {
    pub const fn default_sys_interval() -> u64 {
        3
    }

    pub fn default_user() -> String {
        "hebo".to_string()
    }

    pub fn default_pid_file() -> PathBuf {
        PathBuf::from("/var/run/hebo.pid")
    }

    pub const fn default_no_delay() -> bool {
        false
    }

    pub const fn default_message_size_limit() -> usize {
        64 * 1024
    }

    pub const fn default_max_qos() -> QoS {
        QoS::ExactOnce
    }

    pub const fn default_max_keep_alive() -> u64 {
        65535
    }

    pub const fn default_max_packet_size() -> usize {
        0
    }

    pub fn sys_interval(&self) -> Duration {
        Duration::from_secs(self.sys_interval)
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn pid_file(&self) -> &Path {
        self.pid_file.as_path()
    }

    pub fn no_delay(&self) -> bool {
        self.no_delay
    }

    pub fn message_size_limit(&self) -> usize {
        self.message_size_limit
    }

    pub fn max_keep_alive(&self) -> u64 {
        self.max_keep_alive
    }

    pub fn max_qos(&self) -> QoS {
        self.max_qos
    }

    pub fn max_packet_size(&self) -> usize {
        self.max_packet_size
    }
}

impl Default for General {
    fn default() -> Self {
        Self {
            sys_interval: Self::default_sys_interval(),
            user: Self::default_user(),
            pid_file: Self::default_pid_file(),
            no_delay: Self::default_no_delay(),
            message_size_limit: Self::default_message_size_limit(),
            max_qos: Self::default_max_qos(),
            max_keep_alive: Self::default_max_keep_alive(),
            max_packet_size: Self::default_max_packet_size(),
        }
    }
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
        }
    }
}

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

#[derive(Debug, Deserialize, Clone)]
pub struct Security {
    /// Determines whether clients that connect without providing a username are allowed to connect.
    /// It is highly recommended to disable this switch and configure an authorization policy.
    ///
    /// Default is true.
    #[serde(default = "Security::default_allow_anonymous")]
    allow_anonymous: bool,

    /// Control access to the broker using a password file.
    ///
    /// This file can be generated using the hebo-passwd utility.
    /// The file should be a text file with lines in the format:
    /// `username:password`.
    /// The password (and colon) may be omitted if desired, although this
    /// offers very little in the way of security.
    ///
    /// If an auth_plugin is used as well as password_file, the auth_plugin check will be made first.
    ///
    /// Default is None.
    #[serde(default = "Security::default_password_file")]
    password_file: Option<PathBuf>,
}

impl Security {
    pub const fn default_allow_anonymous() -> bool {
        true
    }

    pub fn default_password_file() -> Option<PathBuf> {
        None
    }

    pub fn allow_anonymous(&self) -> bool {
        self.allow_anonymous
    }

    pub fn password_file(&self) -> Option<&Path> {
        self.password_file.as_ref().and_then(|p| Some(p.as_path()))
    }
}

impl Default for Security {
    fn default() -> Self {
        Self {
            allow_anonymous: Self::default_allow_anonymous(),
            password_file: Self::default_password_file(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Storage {
    /// Save persistent message data to disk.
    ///
    /// This saves information about all messages, including subscriptions, currently in-flight messages
    /// and retained messages.
    ///
    /// Default is true.
    #[serde(default = "Storage::default_persistence")]
    persistence: bool,

    /// Location for persistent database.
    ///
    /// Default is "/var/lib/hebo/hebo.db"
    #[serde(default = "Storage::default_db_path")]
    db_path: PathBuf,

    /// If persistence is enabled, save the in-memory database to disk every autosave_interval seconds.
    ///
    /// If set to 0, the persistence database will only be written when hebo exits.
    /// See also `autosave_on_changes`.
    /// Note that writing of the persistence database can be forced by sending a SIGUSR1 signal.
    ///
    /// Default is 1800 seconds.
    #[serde(default = "Storage::default_auto_save_interval")]
    auto_save_interval: u64,

    /// If is not None, hebo will count the number of subscription changes, retained messages received
    /// and queued messages and if the total exceeds specified threshold then
    /// the in-memory database will be saved to disk.
    ///
    /// Default is None.
    #[serde(default = "Storage::default_auto_save_on_change")]
    auto_save_on_change: Option<u64>,
}

impl Storage {
    pub const fn default_persistence() -> bool {
        true
    }

    pub fn default_db_path() -> PathBuf {
        PathBuf::from("/var/lib/hebo/hebo.db")
    }

    pub const fn default_auto_save_interval() -> u64 {
        1800
    }

    pub const fn default_auto_save_on_change() -> Option<u64> {
        None
    }

    pub fn persistence(&self) -> bool {
        self.persistence
    }

    pub fn db_path(&self) -> &Path {
        self.db_path.as_path()
    }

    pub fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(self.auto_save_interval)
    }

    pub fn auto_save_on_change(&self) -> Option<Duration> {
        self.auto_save_on_change
            .and_then(|interval| Some(Duration::from_secs(interval)))
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            persistence: Self::default_persistence(),
            db_path: Self::default_db_path(),
            auto_save_interval: Self::default_auto_save_interval(),
            auto_save_on_change: Self::default_auto_save_on_change(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    /// Alaso print log to console.
    ///
    /// Default is true.
    #[serde(default = "Log::default_console_log")]
    console_log: bool,

    /// Set minimum log level.
    ///
    /// Avaliable values are:
    /// - off, disable log
    /// - error
    /// - warn
    /// - info
    /// - debug
    /// - trace
    ///
    /// Default is "info".
    #[serde(default = "Log::default_log_level")]
    log_level: LogLevel,

    /// Path to log file.
    ///
    /// Default is "/var/log/hebo/hebo.log".
    #[serde(default = "Log::default_log_file")]
    log_file: PathBuf,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum LogLevel {
    #[serde(alias = "off")]
    Off,

    #[serde(alias = "error")]
    Error,

    #[serde(alias = "warn")]
    Warn,

    #[serde(alias = "info")]
    Info,

    #[serde(alias = "debug")]
    Debug,

    #[serde(alias = "trace")]
    Trace,
}

impl Log {
    pub const fn default_console_log() -> bool {
        true
    }

    pub const fn default_log_level() -> LogLevel {
        LogLevel::Info
    }

    pub fn default_log_file() -> PathBuf {
        PathBuf::from("/var/log/hebo/hebo.log")
    }

    pub fn console_log(&self) -> bool {
        self.console_log
    }

    pub fn log_level(&self) -> LogLevel {
        self.log_level
    }

    pub fn log_file(&self) -> &Path {
        self.log_file.as_path()
    }
}

impl Default for Log {
    fn default() -> Self {
        Self {
            console_log: Self::default_console_log(),
            log_level: Self::default_log_level(),
            log_file: Self::default_log_file(),
        }
    }
}

/// Configuration for dashboard app.
#[derive(Debug, Deserialize, Clone)]
pub struct Dashboard {
    /// Binding address.
    ///
    /// Default is `127.0.0.1:18083`.
    #[serde(default = "Dashboard::default_address")]
    address: String,
}

impl Dashboard {
    fn default_address() -> String {
        "127.0.0.1:18083".to_string()
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Self {
            address: Self::default_address(),
        }
    }
}
