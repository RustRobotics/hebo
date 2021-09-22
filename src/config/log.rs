// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::path::{Path, PathBuf};

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
