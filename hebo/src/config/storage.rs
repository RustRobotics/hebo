// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::error::Error;

#[allow(clippy::module_name_repetitions)]
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
    #[must_use]
    pub const fn default_persistence() -> bool {
        true
    }

    #[must_use]
    pub fn default_db_path() -> PathBuf {
        PathBuf::from("/var/lib/hebo/hebo.db")
    }

    #[must_use]
    pub const fn default_auto_save_interval() -> u64 {
        1800
    }

    #[must_use]
    pub const fn default_auto_save_on_change() -> Option<u64> {
        None
    }

    #[must_use]
    pub const fn persistence(&self) -> bool {
        self.persistence
    }

    #[must_use]
    pub fn db_path(&self) -> &Path {
        self.db_path.as_path()
    }

    #[must_use]
    pub const fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(self.auto_save_interval)
    }

    #[must_use]
    pub fn auto_save_on_change(&self) -> Option<Duration> {
        self.auto_save_on_change.map(Duration::from_secs)
    }

    /// Validate storage config.
    ///
    /// # Errors
    ///
    /// Does nothing.
    pub const fn validate(&self) -> Result<(), Error> {
        // TODO(Shaohua): check storage file permission
        Ok(())
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
