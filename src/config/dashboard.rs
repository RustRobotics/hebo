// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;

/// Configuration for dashboard app.
#[derive(Debug, Deserialize, Clone)]
pub struct Dashboard {
    /// Binding address.
    ///
    /// Default is `127.0.0.1:18083`.
    #[serde(default = "Dashboard::default_address")]
    address: String,
}

impl Dashboard {
    fn default_address() -> String {
        "127.0.0.1:18083".to_string()
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Self {
            address: Self::default_address(),
        }
    }
}
