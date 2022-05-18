// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde::Deserialize;

use crate::error::Error;

mod dashboard;
mod general;
mod listener;
mod log;
mod security;
mod storage;

pub use self::log::{Log, LogLevel};
pub use dashboard::Dashboard;
pub use general::General;
pub use listener::{Listener, Protocol};
pub use security::Security;
pub use storage::Storage;

/// Server main config.
#[derive(Debug, Default, Clone, Deserialize)]
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
    #[must_use]
    pub const fn general(&self) -> &General {
        &self.general
    }

    #[must_use]
    pub fn listeners(&self) -> &[Listener] {
        &self.listeners
    }

    #[must_use]
    pub const fn security(&self) -> &Security {
        &self.security
    }

    #[must_use]
    pub const fn storage(&self) -> &Storage {
        &self.storage
    }

    #[must_use]
    pub const fn log(&self) -> &Log {
        &self.log
    }

    #[must_use]
    pub const fn dashboard(&self) -> &Dashboard {
        &self.dashboard
    }

    /// Validate config.
    ///
    /// # Errors
    ///
    /// Returns error if some options in config is invalid.
    pub fn validate(&self, bind_address: bool) -> Result<(), Error> {
        self.general.validate()?;

        for listener in &self.listeners {
            listener.validate(bind_address)?;
        }

        self.security.validate()?;
        self.storage.validate()?;
        self.log.validate()?;
        self.dashboard.validate(bind_address)
    }
}
