// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde::Deserialize;
use std::time::Duration;
use tokio_postgres::config::{Config, SslMode};
use tokio_postgres::NoTls;

use crate::error::Error;

/// Configuration for connection to pgsql server.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Clone)]
pub struct PgSQLConnConfig {
    /// Use unix domain socket connection to `PgSQL`.
    ///
    /// Default is false.
    #[serde(default = "PgSQLConnConfig::default_use_uds")]
    pub use_uds: bool,

    /// Socket address to server.
    ///
    /// Default is empty.
    #[serde(default = "PgSQLConnConfig::default_socket")]
    pub socket: String,

    /// `PgSQL` server ip or hostname.
    ///
    /// Default is "127.0.0.1"
    #[serde(default = "PgSQLConnConfig::default_host")]
    pub host: String,

    /// Server port number.
    ///
    /// Default is 5432.
    #[serde(default = "PgSQLConnConfig::default_port")]
    pub port: u16,

    /// `PgSQL` database .
    ///
    /// Default is `hebo-mqtt`.
    #[serde(default = "PgSQLConnConfig::default_database")]
    pub database: String,

    /// Connection username.
    ///
    /// Default is `postgres`.
    #[serde(default = "PgSQLConnConfig::default_username")]
    pub username: String,

    /// Connection password.
    ///
    /// Default is empty.
    #[serde(default = "PgSQLConnConfig::default_password")]
    pub password: String,

    /// Connection pool.
    ///
    /// Default is 4.
    #[serde(default = "PgSQLConnConfig::default_pool_size")]
    pub pool_size: usize,

    /// Connection/query timeout in seconds.
    ///
    /// Default is 5s.
    #[serde(default = "PgSQLConnConfig::default_query_timeout")]
    pub query_timeout: u32,
}

impl PgSQLConnConfig {
    const fn default_use_uds() -> bool {
        false
    }

    const fn default_socket() -> String {
        String::new()
    }

    fn default_host() -> String {
        "127.0.0.1".to_string()
    }

    const fn default_port() -> u16 {
        5432
    }

    fn default_username() -> String {
        "postgres".to_string()
    }

    const fn default_password() -> String {
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

impl Default for PgSQLConnConfig {
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

impl PgSQLConnConfig {
    #[must_use]
    pub fn query_timeout(&self) -> Duration {
        Duration::from_secs(u64::from(self.query_timeout))
    }

    fn get_config(&self) -> Config {
        let mut builder = Config::new();
        builder
            .user(&self.username)
            .password(self.password.as_bytes())
            .dbname(&self.database)
            .application_name("hebo")
            .ssl_mode(SslMode::Disable)
            .port(self.port)
            .connect_timeout(self.query_timeout());
        if self.use_uds {
            builder.host_path(&self.socket);
        } else {
            builder.host(&self.host);
        }

        builder
    }
}

pub struct PgSQLConn {
    client: tokio_postgres::Client,
}

impl PgSQLConn {
    /// Connect to postgres database.
    ///
    /// # Errors
    ///
    /// Returns error if failed to connect to db.
    pub async fn connect(pg_config: &PgSQLConnConfig) -> Result<Self, Error> {
        let config = pg_config.get_config();
        let (client, connection) = config.connect(NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        // No need to keep reference to connection, it will be disconnected and dropped
        // once client is dropped.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {e}");
            }
        });

        Ok(Self { client })
    }

    pub fn get_conn(&mut self) -> &tokio_postgres::Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pgsql_config() {
        let config: Result<PgSQLConnConfig, Error> = toml::from_str(
            r#"
        use_ds = false
        database = "hebo-mqtt"
        username = "user1"
        password = "password1"
        pool_size = 8
        query_timeout = 6
        "#,
        )
        .map_err(Into::into);
        assert!(config.is_ok());
    }

    #[test]
    #[ignore]
    fn test_pgsql_conn() {
        let config = PgSQLConnConfig {
            password: "hebo-password".to_string(),
            ..PgSQLConnConfig::default()
        };

        tokio_test::block_on(async {
            let pgsql_conn = PgSQLConn::connect(&config).await;
            assert!(pgsql_conn.is_ok());
            let mut pgsql_conn = pgsql_conn.unwrap();

            let conn = pgsql_conn.get_conn();

            // Now we can execute a simple statement that just returns its parameter.
            let rows = conn.query("SELECT $1::TEXT", &[&"hello world"]).await;
            assert!(rows.is_ok());
            let rows = rows.unwrap();

            // And then check that we got back the same string we sent over.
            let value: &str = rows[0].get(0);
            assert_eq!(value, "hello world");
        });
    }
}
