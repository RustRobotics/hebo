// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::path::PathBuf;

use codec::QoS;

/// Server main config.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "General::default")]
    pub general: General,
    pub listeners: Vec<Listener>,
    pub security: Security,
    pub storage: Storage,
    pub log: Log,
}

/// General section in config.
#[derive(Debug, Deserialize, Clone)]
pub struct General {
    /// Time interval to send $SYS messages in seconds.
    ///
    /// Set to 0 to disable $SYS messages.
    /// Default is 3s.
    #[serde(default = "General::default_sys_interval")]
    pub sys_interval: u32,

    /// When run as root, drop privileges to this user.
    ///
    /// Default user is "hebo".
    #[serde(default = "General::default_user")]
    pub user: String,

    /// Write process id to a file. A blank string means a pid file shouldn't be written.
    ///
    /// Default is `/var/run/hebo.pid`.
    #[serde(default = "General::default_pid_file")]
    pub pid_file: PathBuf,

    /// Disable Nagle's algorithm on client sockets.
    ///
    /// This has the effect of reducing latency of individual messages
    /// at the potential cost of increasing the number of packets being sent.
    /// Default is false.
    #[serde(default = "General::default_no_delay")]
    pub no_delay: bool,

    /// Set maximium size for publish message payload.
    ///
    /// Received messages that exceed this size will not be accepted by the broker.
    /// MQTT imposes a maximum payload size of 268435455 bytes.
    /// Default value is 0, which means that all valid MQTT messages are accepted.
    #[serde(default = "General::default_message_size_limit")]
    pub message_size_limit: usize,

    /// For MQTT v5 clients, it is possible to have the server send a "server keepalive" value
    /// that will override the keepalive value set by the client.
    ///
    /// This is intended to be used as a mechanism to say that the server will disconnect the client
    /// earlier than it anticipated, and that the client should use the new keepalive value.
    /// The `max_keepalive` option allows you to specify that clients may only
    /// connect with keepalive less than or equal to this value, otherwise they will be
    /// sent a server keepalive telling them to use `max_keepalive`.
    /// This only applies to MQTT v5 clients. The maximum value allowable is 65535. Do not set below 10.
    /// Default value is 65535.
    #[serde(default = "General::default_max_keepalive")]
    pub max_keepalive: u32,

    /// Set the maximum QoS supported.
    ///
    /// Clients publishing at a QoS higher than specified here will be disconnected.
    /// Available values are 0, 1 and 2.
    /// Default is 2.
    #[serde(default = "General::default_max_qos")]
    pub max_qos: QoS,

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
    /// Defaults is 0, which means no limit.
    #[serde(default = "General::default_max_packet_size")]
    pub max_packet_size: usize,
    //pub max_queued_messages: usize,
    //pub max_queued_bytes: usize,
}

impl General {
    pub const fn default_sys_interval() -> u32 {
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

    pub const fn default_max_keepalive() -> u32 {
        65535
    }

    pub const fn default_max_packet_size() -> usize {
        0
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
            max_keepalive: Self::default_max_keepalive(),
            max_packet_size: Self::default_max_packet_size(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Listener {
    /// Network interface to bind to.
    pub interface: Option<String>,

    pub max_connections: usize,

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
