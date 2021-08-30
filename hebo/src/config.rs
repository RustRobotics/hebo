// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::path::PathBuf;

/// Server main config.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub general: General,
    pub listeners: Vec<Listener>,
    pub security: Security,
    pub storage: Storage,
    pub log: Log,
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub max_memory: usize,
    pub message_size_limit: usize,

    /// Time interval to send $SYS messages.
    pub sys_interval: u32,

    /// When run as root, drop privileges to this user and its primary group.
    pub user: Option<String>,
    pub group: Option<String>,

    /// Path to pid file.
    pub pid_file: PathBuf,

    pub max_keepalive: u32,
    pub max_connections: usize,
    pub no_delay: bool,

    pub max_packet_size: usize,
    pub max_queued_messages: usize,
    pub max_queued_bytes: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Listener {
    /// Network interface to bind to.
    pub interface: Option<String>,

    /// Binding protocol.
    pub protocol: Protocol,

    /// Binding address, including domain name and port, e.g. localhost:1883
    /// For unix domain socket, path to socket file.
    pub address: String,

    /// Url path to bind to, only used for websocket protocols.
    #[serde(default = "listener_default_path")]
    pub path: String,

    /// Path to TLS cert file.
    pub cert_file: Option<PathBuf>,

    /// Path to TLS private key file.
    pub key_file: Option<PathBuf>,
}

/// Binding protocol types.
#[derive(Debug, Deserialize, Clone)]
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
    pub allow_anonymous: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Storage {
    pub persistence: bool,
    pub db_path: Option<PathBuf>,
    pub auto_save_interval: usize,
    pub auto_save_on_change: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub console_log: bool,
    pub level: LogLevel,

    #[serde(default = "log_default_file")]
    pub log_file: PathBuf,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

fn listener_default_path() -> String {
    "/".to_owned()
}

fn log_default_file() -> PathBuf {
    PathBuf::from("/var/log/hebo/hebo.log")
}
