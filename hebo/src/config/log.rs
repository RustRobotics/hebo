// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use serde::Deserialize;
use std::fs::{self, File};
use std::path::Path;

use crate::error::{Error, ErrorKind};

#[derive(Debug, Deserialize, Clone)]
#[allow(clippy::struct_field_names)]
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
    /// Default is None.
    #[serde(default = "Log::default_log_file")]
    log_file: Option<String>,
}

#[allow(clippy::module_name_repetitions)]
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
    #[must_use]
    pub const fn default_console_log() -> bool {
        true
    }

    #[must_use]
    pub const fn default_log_level() -> LogLevel {
        LogLevel::Info
    }

    #[must_use]
    pub const fn default_log_file() -> Option<String> {
        //PathBuf::from("/var/log/hebo/hebo.log")
        None
    }

    #[must_use]
    pub const fn console_log(&self) -> bool {
        self.console_log
    }

    #[must_use]
    pub const fn log_level(&self) -> LogLevel {
        self.log_level
    }

    #[must_use]
    pub const fn log_file(&self) -> Option<&String> {
        self.log_file.as_ref()
    }

    /// Validate config.
    ///
    /// # Errors
    ///
    /// Returns error if failed to create log file or its parent directory.
    pub fn validate(&self) -> Result<(), Error> {
        if let Some(log_file) = &self.log_file {
            let path = Path::new(log_file);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|err| {
                    Error::from_string(
                        ErrorKind::ConfigError,
                        format!(
                            "Failed to create parent directory for log file: {log_file}, err: {err:?}"
                        ),
                    )
                })?;
                let _fd = File::options()
                    .create(true)
                    .append(true)
                    .open(log_file)
                    .map_err(|err| {
                        Error::from_string(
                            ErrorKind::ConfigError,
                            format!("Failed to create log file: {log_file}, err: {err:?}"),
                        )
                    })?;
            }
        }
        Ok(())
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
