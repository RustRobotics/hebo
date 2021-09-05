// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

/// Configuration for connection to redis server.
#[derive(Debug, Deserialize, Clone)]
pub struct RedisConnConfig {
    /// Redis server address.
    ///
    /// Default is "127.0.0.1:6379"
    #[serde(default = "RedisConnConfig::default_server")]
    pub server: SocketAddr,

    /// Redis password.
    ///
    /// Default is None.
    #[serde(default = "RedisConnConfig::default_password")]
    pub password: Option<String>,

    /// Connection pool.
    ///
    /// Default is 8.
    #[serde(default = "RedisConnConfig::default_pool_size")]
    pub pool_size: usize,

    /// Redis database number.
    ///
    /// Default is 0.
    #[serde(default = "RedisConnConfig::default_database")]
    pub database: u32,

    /// Default is 5s.
    #[serde(default = "RedisConnConfig::default_query_timeout")]
    pub query_timeout: Duration,
}

impl RedisConnConfig {
    fn default_server() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6379)
    }

    fn default_password() -> Option<String> {
        None
    }

    fn default_pool_size() -> usize {
        8
    }

    fn default_database() -> u32 {
        0
    }

    fn default_query_timeout() -> Duration {
        Duration::from_secs(5)
    }
}
