// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub connections: Connections,
    pub security: Security,
    pub storage: Storage,
    pub log: Log,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub max_memory: usize,
    pub message_size_limit: usize,
    pub sys_interval: u32,

    /// When run as root, drop privileges to this user and its primary group.
    pub user: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Connections {
    pub bind_interface: Option<String>,
    pub mqtt: String,
    pub mqtts: Option<String>,
    pub ws: Option<String>,
    pub wss: Option<String>,
    pub pid_file: Option<String>,
    pub unix_socket: Option<String>,

    pub max_keepalive: u32,
    pub max_connections: usize,
    pub no_delay: bool,

    pub cert_file: Option<String>,
    pub key_file: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Messages {
    pub max_packet_size: usize,
    pub max_queued_messages: usize,
    pub max_queued_bytes: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Security {
    pub allow_anonymous: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Storage {
    pub persistence: bool,
    pub db_path: String,
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
