// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::QoS;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::time::Duration;

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