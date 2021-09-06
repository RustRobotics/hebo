// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use mongodb::options::{ClientOptions, Credential, StreamAddress};
use serde_derive::Deserialize;
use std::time::Duration;

use crate::error::Error;

/// Configuration for connection to pgsql server.
#[derive(Debug, Deserialize, Clone)]
pub struct MongoConnConfig {
    /// Use unix domain socket connection to MongoDB.
    ///
    /// Default is false.
    #[serde(default = "MongoConnConfig::default_use_uds")]
    pub use_uds: bool,

    /// Socket address to server.
    ///
    /// Default is empty.
    #[serde(default = "MongoConnConfig::default_socket")]
    pub socket: String,

    /// Mongo server ip or hostname.
    ///
    /// Default is "127.0.0.1"
    #[serde(default = "MongoConnConfig::default_host")]
    pub host: String,

    /// Server port number.
    ///
    /// Default is 27017.
    #[serde(default = "MongoConnConfig::default_port")]
    pub port: u16,

    /// Mongodb database name.
    ///
    /// Default is `hebo-mqtt`.
    #[serde(default = "MongoConnConfig::default_database")]
    pub database: String,

    /// Connection username.
    ///
    /// Default is None.
    #[serde(default = "MongoConnConfig::default_username")]
    pub username: Option<String>,

    /// Connection password.
    ///
    /// Default is None.
    #[serde(default = "MongoConnConfig::default_password")]
    pub password: Option<String>,

    /// Connection pool.
    ///
    /// Default is 4.
    #[serde(default = "MongoConnConfig::default_pool_size")]
    pub pool_size: usize,

    /// Connection/query timeout in seconds.
    ///
    /// Default is 5s.
    #[serde(default = "MongoConnConfig::default_query_timeout")]
    pub query_timeout: u32,
}

impl MongoConnConfig {
    const fn default_use_uds() -> bool {
        false
    }

    fn default_socket() -> String {
        String::new()
    }

    fn default_host() -> String {
        "127.0.0.1".to_string()
    }

    const fn default_port() -> u16 {
        27017
    }

    fn default_username() -> Option<String> {
        None
    }

    fn default_password() -> Option<String> {
        None
    }

    fn default_database() -> String {
        "hebo-mqtt".to_string()
    }

    const fn default_pool_size() -> usize {
        4
    }

    const fn default_query_timeout() -> u32 {
        5
    }
}

impl Default for MongoConnConfig {
    fn default() -> Self {
        Self {
            use_uds: Self::default_use_uds(),
            socket: Self::default_socket(),
            host: Self::default_host(),
            port: Self::default_port(),
            database: Self::default_database(),
            username: Self::default_username(),
            password: Self::default_password(),
            pool_size: Self::default_pool_size(),
            query_timeout: Self::default_query_timeout(),
        }
    }
}

impl MongoConnConfig {
    pub fn query_timeout(&self) -> Duration {
        Duration::from_secs(self.query_timeout as u64)
    }

    fn get_config(&self) -> ClientOptions {
        let mut credential = Credential::default();
        credential.username = self.username.clone();
        credential.password = self.password.clone();
        let mut builder = ClientOptions::default();
        builder.hosts = vec![StreamAddress {
            hostname: self.host.clone(),
            port: Some(self.port),
        }];
        builder.app_name = Some("hebo".to_string());
        builder.connect_timeout = Some(self.query_timeout());
        builder.credential = Some(credential);

        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mongo_config() {
        let config: Result<MongoConnConfig, Error> = toml::from_str(
            r#"
        use_ds = false
        database = "hebo-mqtt"
        pool_size = 8
        query_timeout = 6
        "#,
        )
        .map_err(Into::into);
        assert!(config.is_ok());
    }
}
