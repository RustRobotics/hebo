// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::error::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Clone)]
pub struct Security {
    /// Determines whether clients that connect without providing a username are allowed to connect.
    /// It is highly recommended to disable this switch and configure an authorization policy.
    ///
    /// Default is true.
    #[serde(default = "Security::default_allow_anonymous")]
    allow_anonymous: bool,

    /// Control access to the broker using a password file.
    ///
    /// This file can be generated using the hebo-passwd utility.
    /// The file should be a text file with lines in the format:
    /// `username:password`.
    /// The password (and colon) may be omitted if desired, although this
    /// offers very little in the way of security.
    ///
    /// If an auth_plugin is used as well as password_file, the auth_plugin check will be made first.
    ///
    /// Default is None.
    #[serde(default = "Security::default_password_file")]
    password_file: Option<PathBuf>,
}

impl Security {
    #[must_use]
    pub const fn default_allow_anonymous() -> bool {
        true
    }

    #[must_use]
    pub const fn default_password_file() -> Option<PathBuf> {
        None
    }

    #[must_use]
    pub const fn allow_anonymous(&self) -> bool {
        self.allow_anonymous
    }

    #[must_use]
    pub fn password_file(&self) -> Option<&Path> {
        self.password_file.as_deref()
    }

    /// Validate security config.
    ///
    /// # Errors
    ///
    /// Does nothing.
    pub const fn validate(&self) -> Result<(), Error> {
        // TODO(Shaohua): Validate password file entry
        Ok(())
    }
}

impl Default for Security {
    fn default() -> Self {
        Self {
            allow_anonymous: Self::default_allow_anonymous(),
            password_file: Self::default_password_file(),
        }
    }
}
