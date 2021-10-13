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

    /// Path to redis socket.
    ///
    /// Default is empty.
    #[serde(default = "RedisConnConfig::default_socket")]
    pub socket: String,

    /// Redis server ip or hostname.
    ///
    /// Default is "127.0.0.1"
    #[serde(default = "RedisConnConfig::default_host")]
    pub host: String,

    /// Redis server port.
    ///
    /// Default is 6379
    #[serde(default = "RedisConnConfig::default_port")]
    pub port: u16,

    /// Redis database number.
    ///
    /// Default is None.
    #[serde(default = "RedisConnConfig::default_database")]
    pub database: Option<u32>,

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

    /// Connection pool.
    ///
    /// Default is 4.
    #[serde(default = "RedisConnConfig::default_pool_size")]
    pub pool_size: usize,

    /// Redis query timeout in seconds.
    ///
    /// Default is 5s.
    #[serde(default = "RedisConnConfig::default_query_timeout")]
    pub query_timeout: u32,
}

impl RedisConnConfig {
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
        6379
    }

    fn default_username() -> Option<String> {
        None
    }

    fn default_password() -> Option<String> {
        None
    }

    const fn default_database() -> Option<u32> {
        None
    }

    const fn default_pool_size() -> usize {
        4
    }

    const fn default_query_timeout() -> u32 {
        5
    }
}

impl Default for RedisConnConfig {
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

impl RedisConnConfig {
    pub fn query_timeout(&self) -> Duration {
        Duration::from_secs(self.query_timeout as u64)
    }

    pub fn get_uri(&self) -> String {
        let mut uri = String::new();
        if self.use_uds {
            // For `redis+unix:///<path>[?db=<db>[&pass=<password>][&user=<username>]]`
            uri.push_str(&format!("redis+unix://{}", self.socket));
            if let Some(db) = self.database {
                uri.push_str(&format!("?db={}", db));
            }
            if let Some(username) = &self.username {
                uri.push_str(&format!("&username={}", username));
            }
            if let Some(password) = &self.password {
                uri.push_str(&format!("&pass={}", password));
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
            uri.push_str(&format!("{}:{}", self.host, self.port));
            if let Some(db) = self.database {
                uri.push_str(&format!("/{}", db));
            }
        }

        uri
    }
}

#[derive(Clone)]
pub struct RedisConn {
    client: redis::Client,
    conn: Option<redis::aio::ConnectionManager>,
}

impl RedisConn {
    pub fn new(config: &RedisConnConfig) -> Result<Self, Error> {
        let client = redis::Client::open(config.get_uri())?;
        Ok(Self { client, conn: None })
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        self.conn = Some(self.client.get_tokio_connection_manager().await?);
        Ok(())
    }

    pub fn get_conn(&self) -> Option<redis::aio::ConnectionManager> {
        self.conn.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn test_redis_conn_config() {
        let config: Result<RedisConnConfig, Error> = toml::from_str(
            r#"
        database = 1
        username = "user1"
        password = "password1"
        pool_size = 8
        query_timeout = 6
        "#,
        )
        .map_err(Into::into);
        assert!(config.is_ok());
        let config = config.unwrap();
        println!("{:?}", config.query_timeout);
        assert_eq!(config.query_timeout(), Duration::from_secs(6));
        let uri = config.get_uri();
        assert_eq!(uri, "redis://user1:password1@127.0.0.1:6379/1");
    }

    #[test]
    fn test_redis_conn_config_uds() {
        let config: Result<RedisConnConfig, Error> = toml::from_str(
            r#"
        use_uds = true
        socket = "/var/run/redis.sock"
        database = 1
        password = "password1"
        pool_size = 8
        "#,
        )
        .map_err(Into::into);
        assert!(config.is_ok());
        let config = config.unwrap();
        let uri = config.get_uri();
        assert_eq!(
            uri,
            "redis+unix:///var/run/redis.sock?db=1&password=password1"
        );
    }
}
