// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use mysql::prelude::*;
use serde_derive::Deserialize;
use std::path::PathBuf;
use std::time::Duration;

use crate::error::Error;

/// Configuration for connection to mysql server.
#[derive(Debug, Deserialize, Clone)]
pub struct MySQLConnConfig {
    /// Use unix domain socket connection to MySQL.
    ///
    /// Default is false.
    #[serde(default = "MySQLConnConfig::default_use_uds")]
    pub use_uds: bool,

    /// Socket address to server.
    ///
    /// Default is None.
    #[serde(default = "MySQLConnConfig::default_socket")]
    pub socket: Option<PathBuf>,

    /// MySQL server ip or hostname.
    ///
    /// Default is "127.0.0.1"
    #[serde(default = "MySQLConnConfig::default_ip")]
    pub ip: String,

    /// Server port number.
    ///
    /// Default is 3306.
    #[serde(default = "MySQLConnConfig::default_port")]
    pub port: u16,

    /// MySQL database number.
    ///
    /// Default is `hebo-mqtt`.
    #[serde(default = "MySQLConnConfig::default_database")]
    pub database: String,

    /// Connection username.
    ///
    /// Default is `root`.
    #[serde(default = "MySQLConnConfig::default_username")]
    pub username: String,

    /// Connection password.
    ///
    /// Default is empty.
    #[serde(default = "MySQLConnConfig::default_password")]
    pub password: String,

    /// Connection pool.
    ///
    /// Default is 4.
    #[serde(default = "MySQLConnConfig::default_pool_size")]
    pub pool_size: usize,

    /// Connection/query timeout in seconds.
    ///
    /// Default is 5s.
    #[serde(default = "MySQLConnConfig::default_query_timeout")]
    pub query_timeout: u32,
}

impl MySQLConnConfig {
    const fn default_use_uds() -> bool {
        false
    }

    fn default_socket() -> Option<PathBuf> {
        None
    }

    fn default_ip() -> String {
        "127.0.0.1".to_string()
    }

    const fn default_port() -> u16 {
        3306
    }

    fn default_username() -> String {
        "root".to_string()
    }

    fn default_password() -> String {
        String::new()
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

impl Default for MySQLConnConfig {
    fn default() -> Self {
        Self {
            use_uds: Self::default_use_uds(),
            socket: Self::default_socket(),
            ip: Self::default_ip(),
            port: Self::default_port(),
            database: Self::default_database(),
            username: Self::default_username(),
            password: Self::default_password(),
            pool_size: Self::default_pool_size(),
            query_timeout: Self::default_query_timeout(),
        }
    }
}

impl MySQLConnConfig {
    pub fn query_timeout(&self) -> Duration {
        Duration::from_secs(self.query_timeout as u64)
    }
}

#[derive(Clone)]
pub struct MySQLConn {
    pool: mysql::Pool,
}

impl MySQLConn {
    pub fn new(config: &MySQLConnConfig) -> Result<Self, Error> {
        unimplemented!()
    }
}
