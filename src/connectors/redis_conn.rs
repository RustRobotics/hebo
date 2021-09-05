// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::time::Duration;

use crate::error::Error;

/// Configuration for connection to redis server.
#[derive(Debug, Deserialize, Clone)]
pub struct RedisConnConfig {
    /// Use unix domain socket connection to redis.
    ///
    /// Default is false.
    #[serde(default = "RedisConnConfig::default_use_uds")]
    pub use_uds: bool,

    /// Redis server address.
    ///
    /// Default is "127.0.0.1:6379"
    #[serde(default = "RedisConnConfig::default_address")]
    pub address: String,

    /// Redis username.
    ///
    /// Default is None.
    #[serde(default = "RedisConnConfig::default_username")]
    pub username: Option<String>,

    /// Redis password.
    ///
    /// Default is None.
    #[serde(default = "RedisConnConfig::default_password")]
    pub password: Option<String>,

    /// Redis database number.
    ///
    /// Default is None.
    #[serde(default = "RedisConnConfig::default_database")]
    pub database: Option<u32>,

    /// Connection pool.
    ///
    /// Default is 8.
    #[serde(default = "RedisConnConfig::default_pool_size")]
    pub pool_size: usize,

    /// Default is 5s.
    #[serde(default = "RedisConnConfig::default_query_timeout")]
    pub query_timeout: Duration,
}

impl RedisConnConfig {
    fn default_use_uds() -> bool {
        false
    }

    fn default_address() -> String {
        "127.0.0.1:6379".to_string()
    }

    fn default_username() -> Option<String> {
        None
    }

    fn default_password() -> Option<String> {
        None
    }

    fn default_database() -> Option<u32> {
        None
    }

    fn default_pool_size() -> usize {
        8
    }

    fn default_query_timeout() -> Duration {
        Duration::from_secs(5)
    }

    pub fn get_uri(&self) -> String {
        let mut uri = String::new();
        if self.use_uds {
            // For `redis+unix:///<path>[?db=<db>[&pass=<password>][&user=<username>]]`
            uri.push_str("redis+unix://");
            uri.push_str(&self.address);
            if let Some(db) = self.database {
                uri.push_str(&format!("?db={}", db));
            }
            if let Some(username) = &self.username {
                uri.push_str(&format!("&username={}", username));
            }
            if let Some(password) = &self.password {
                uri.push_str(&format!("&password={}", password));
            }
        } else {
            // For `redis://[<username>][:<password>@]<hostname>[:port][/<db>]`
            uri.push_str("redis://");
            if let Some(username) = &self.username {
                uri.push_str(username);
            }
            if let Some(password) = &self.password {
                uri.push_str(&format!(":{}@", password));
            }
            uri.push_str(&self.address);
            if let Some(db) = self.database {
                uri.push_str(&format!("/{}", db));
            }
        }

        uri
    }
}

#[derive(Debug, Clone)]
pub struct RedisConn {
    config: RedisConnConfig,
    client: redis::Client,
}

impl RedisConn {
    pub fn new(config: RedisConnConfig) -> Result<Self, Error> {
        let client = redis::Client::open(config.get_uri())?;

        unimplemented!()
    }
}
