// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

#![allow(clippy::unsafe_derive_deserialize)]

use codec::QoS;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::error::{Error, ErrorKind};

/// General section in config.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct General {
    /// Time interval to send $SYS messages in seconds.
    ///
    /// Set to 0 to disable $SYS messages.
    ///
    /// Default is 3s.
    #[serde(default = "General::default_sys_interval")]
    sys_interval: u32,

    /// When run as root, drop privileges to this user.
    ///
    /// If hebo is launched by non-root account, this property is ignored.
    ///
    /// Default user is "hebo".
    #[serde(default = "General::default_user")]
    user: String,

    /// Write process id to a file. A blank string means a pid file shouldn't be written.
    ///
    /// Default is `/run/hebo.pid` for root user,
    /// and `/run/user/UID/hebo.pid` for non-root users.
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

    /// Set maximum size for publish message payload.
    ///
    /// Received messages that exceed this size will not be accepted by the broker.
    /// MQTT imposes a maximum payload size of 268435455 bytes.
    ///
    /// Default value is 0, which means that all valid MQTT messages are accepted.
    #[serde(default = "General::default_message_size_limit")]
    message_size_limit: u32,

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
    #[serde(default = "General::default_maximum_keep_alive")]
    maximum_keep_alive: u32,

    /// Set the maximum QoS supported.
    ///
    /// Clients publishing at a QoS higher than specified here will be disconnected.
    /// Available values are 0, 1 and 2.
    ///
    /// Default is 2.
    #[serde(default = "General::default_maximum_qos")]
    maximum_qos: QoS,

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
    #[serde(default = "General::default_maximum_packet_size")]
    maximum_packet_size: u32,
    //pub max_queued_messages: usize,
    //pub max_queued_bytes: usize,
}

impl General {
    #[must_use]
    pub const fn default_sys_interval() -> u32 {
        3
    }

    #[must_use]
    pub fn default_user() -> String {
        "hebo".to_string()
    }

    #[cfg(not(unix))]
    #[must_use]
    pub fn default_pid_file() -> PathBuf {
        PathBuf::from("hebo.pid")
    }

    #[cfg(unix)]
    #[must_use]
    pub fn default_pid_file() -> PathBuf {
        let uid = unsafe { nc::geteuid() };
        if uid == 0 {
            PathBuf::from("/run/hebo.pid")
        } else {
            PathBuf::from(&format!("/run/user/{uid}/hebo.pid"))
        }
    }

    #[must_use]
    pub const fn default_no_delay() -> bool {
        false
    }

    #[must_use]
    pub const fn default_message_size_limit() -> u32 {
        64 * 1024
    }

    #[must_use]
    pub const fn default_maximum_qos() -> QoS {
        QoS::ExactOnce
    }

    #[must_use]
    pub const fn default_maximum_keep_alive() -> u32 {
        65535
    }

    #[must_use]
    pub const fn default_maximum_packet_size() -> u32 {
        0
    }

    #[must_use]
    pub const fn sys_interval(&self) -> Duration {
        Duration::from_secs(self.sys_interval as u64)
    }

    #[must_use]
    pub fn user(&self) -> &str {
        &self.user
    }

    #[must_use]
    pub fn pid_file(&self) -> &Path {
        self.pid_file.as_path()
    }

    #[must_use]
    pub const fn no_delay(&self) -> bool {
        self.no_delay
    }

    #[must_use]
    pub const fn message_size_limit(&self) -> u32 {
        self.message_size_limit
    }

    #[must_use]
    pub const fn maximum_keep_alive(&self) -> u32 {
        self.maximum_keep_alive
    }

    #[must_use]
    pub const fn maximum_qos(&self) -> QoS {
        self.maximum_qos
    }

    #[must_use]
    pub const fn maximum_packet_size(&self) -> u32 {
        self.maximum_packet_size
    }

    /// Validate config.
    ///
    /// # Errors
    ///
    /// Returns error if username not found.
    #[cfg(not(unix))]
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }

    #[cfg(unix)]
    /// # Errors
    /// Returns error if specific user id does not exist.
    pub fn validate(&self) -> Result<(), Error> {
        let euid = unsafe { nc::geteuid() };
        if euid == 0 {
            // For root only.
            if users::get_user_by_name(&self.user).is_none() {
                return Err(Error::from_string(
                    ErrorKind::ConfigError,
                    format!("Failed to find user info with name: {}", &self.user),
                ));
            }
        }
        Ok(())
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
            maximum_qos: Self::default_maximum_qos(),
            maximum_keep_alive: Self::default_maximum_keep_alive(),
            maximum_packet_size: Self::default_maximum_packet_size(),
        }
    }
}
