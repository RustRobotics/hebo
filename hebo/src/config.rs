// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;

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
    pub sys_interval: u32,

    /// When run as root, drop privileges to this user and its primary group.
    pub user: Option<String>,
    pub group: Option<String>,

    /// Path to pid file.
    pub pid_file: String,

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

    pub protocol: Protocol,
    pub address: String,

    pub ca_file: Option<String>,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
}

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
    //Unix,
    // Quic,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Security {
    pub allow_anonymous: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Storage {
    pub persistence: bool,
    pub db_path: Option<String>,
    pub auto_save_interval: usize,
    pub auto_save_on_change: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub console_log: bool,
    pub level: LogLevel,
    pub log_file: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}
