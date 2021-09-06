// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::time::Duration;

use crate::error::Error;

/// Configuration for connection to MySQL server.
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
    pub socket: Option<String>,

    /// MySQL server ip or hostname.
    ///
    /// Default is "127.0.0.1"
    #[serde(default = "MySQLConnConfig::default_host")]
    pub host: String,

    /// Server port number.
    ///
    /// Default is 3306.
    #[serde(default = "MySQLConnConfig::default_port")]
    pub port: u16,

    /// MySQL database name.
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

    fn default_socket() -> Option<String> {
        None
    }

    fn default_host() -> String {
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

impl MySQLConnConfig {
    pub fn query_timeout(&self) -> Duration {
        Duration::from_secs(self.query_timeout as u64)
    }
}

pub struct MySQLConn {
    pool: mysql_async::Pool,
    conn: mysql_async::Conn,
}

impl MySQLConn {
    pub async fn connect(config: &MySQLConnConfig) -> Result<Self, Error> {
        let builder = mysql_async::OptsBuilder::default()
            .user(Some(&config.username))
            .pass(Some(&config.password));
        let builder = if config.use_uds {
            builder.socket(config.socket.as_ref())
        } else {
            builder
                .ip_or_hostname(&config.host)
                .tcp_port(config.port)
                .db_name(Some(&config.database))
        };
        let pool = mysql_async::Pool::new(builder);
        let conn = pool.get_conn().await?;
        Ok(Self { pool, conn })
    }

    pub fn get_conn(&mut self) -> &mut mysql_async::Conn {
        &mut self.conn
    }

    pub async fn disconnect(self) -> Result<(), Error> {
        drop(self.conn);
        self.pool.disconnect().await.map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use mysql_async::prelude::*;

    use super::*;

    #[test]
    fn test_mysql_config() {
        let config: Result<MySQLConnConfig, Error> = toml::from_str(
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

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Payment {
        customer_id: i32,
        amount: i32,
        account_name: Option<String>,
    }

    #[test]
    fn test_mysql_conn() {
        let config = MySQLConnConfig {
            username: "hebo-user".to_string(),
            password: "hebo-password".to_string(),
            ..MySQLConnConfig::default()
        };

        tokio_test::block_on(async {
            let mysql_conn = MySQLConn::connect(&config).await;
            assert!(mysql_conn.is_ok());
            let mut mysql_conn = mysql_conn.unwrap();

            let conn = mysql_conn.get_conn();

            // Create temporary table
            let ret = conn
                .query_drop(
                    r"CREATE TEMPORARY TABLE payment (
                        customer_id int not null,
                        amount int not null,
                        account_name text
                    )",
                )
                .await;
            assert!(ret.is_ok());

            let payments = vec![
                Payment {
                    customer_id: 1,
                    amount: 2,
                    account_name: None,
                },
                Payment {
                    customer_id: 3,
                    amount: 4,
                    account_name: Some("foo".into()),
                },
                Payment {
                    customer_id: 5,
                    amount: 6,
                    account_name: None,
                },
                Payment {
                    customer_id: 7,
                    amount: 8,
                    account_name: None,
                },
                Payment {
                    customer_id: 9,
                    amount: 10,
                    account_name: Some("bar".into()),
                },
            ];

            // Save payments
            let params = payments.clone().into_iter().map(|payment| {
                params! {
                    "customer_id" => payment.customer_id,
                    "amount" => payment.amount,
                    "account_name" => payment.account_name,
                }
            });

            let ret = conn
                .exec_batch(
                    r"INSERT INTO payment (customer_id, amount, account_name)
                      VALUES (:customer_id, :amount, :account_name)",
                    params,
                )
                .await;
            assert!(ret.is_ok());

            let ret = mysql_conn.disconnect().await;
            assert!(ret.is_ok());
        });
    }
}
