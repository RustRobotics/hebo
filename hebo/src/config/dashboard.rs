// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde::Deserialize;
use std::net::TcpListener;

use crate::error::Error;

/// Configuration for dashboard app.
#[derive(Debug, Deserialize, Clone)]
pub struct Dashboard {
    /// Enable dashboard or not.
    ///
    /// Default is true.
    #[serde(default = "Dashboard::default_enable")]
    enable: bool,

    /// Binding address.
    ///
    /// Default is `127.0.0.1:18083`.
    #[serde(default = "Dashboard::default_address")]
    address: String,
}

impl Dashboard {
    fn default_enable() -> bool {
        true
    }

    fn default_address() -> String {
        "127.0.0.1:18083".to_string()
    }

    pub fn enable(&self) -> bool {
        self.enable
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.enable {
            let _socket = TcpListener::bind(&self.address)?;
        }
        Ok(())
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Self {
            enable: Self::default_enable(),
            address: Self::default_address(),
        }
    }
}
