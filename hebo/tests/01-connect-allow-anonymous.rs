// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use hebo::error::Error;
use std::thread::sleep;
use std::time::Duration;

mod common;
use common::{Server, ServerConfig};

const ALLOW_CONFIG_V4: &str = r#"
[general]
pid_file = "/tmp/hebo-tests/mqtt-1889.pid"

[[listeners]]
protocol = "mqtt"
address = "0.0.0.0:1889"

[security]
allow_anonymous = true

[dashboard]
address = "0.0.0.0:18084"

[log]
log_file = "/tmp/hebo-tests/hebo-1889.log"
"#;

const ALLOW_CONFIG_V5: &str = r#"
[general]
pid_file = "/tmp/hebo-tests/mqtt-1890.pid"

[[listeners]]
protocol = "mqtt"
address = "0.0.0.0:1890"

[security]
allow_anonymous = true

[dashboard]
address = "0.0.0.0:18085"

[log]
log_file = "/tmp/hebo-tests/hebo-1890.log"
"#;

const DENY_CONFIG_V4: &str = r#"
[general]
pid_file = "/tmp/hebo-tests/mqtt-1891.pid"

[[listeners]]
protocol = "mqtt"
address = "0.0.0.0:1891"

[security]
allow_anonymous = false

[dashboard]
address = "0.0.0.0:18086"

[log]
log_file = "/tmp/hebo-tests/hebo-1891.log"
"#;

const DENY_CONFIG_V5: &str = r#"
[general]
pid_file = "/tmp/hebo-tests/mqtt-1892.pid"

[[listeners]]
protocol = "mqtt"
address = "0.0.0.0:1892"

[security]
allow_anonymous = false

[dashboard]
address = "0.0.0.0:18087"

[log]
log_file = "/tmp/hebo-tests/hebo-1892.log"
"#;

#[test]
fn test_conn_allow_anonymous_v4() -> Result<(), Error> {
    let config = ServerConfig::new(
        "/tmp/hebo-tests/01-connect-allow-anonymous-v4.toml",
        ALLOW_CONFIG_V4,
    )?;
    let mut server = Server::start(config.filename())?;
    // TODO(Shaohua): Run ruo client
    sleep(Duration::from_secs(5));
    server.terminate();
    Ok(())
}

#[test]
fn test_conn_allow_anonymous_v5() -> Result<(), Error> {
    let config = ServerConfig::new(
        "/tmp/hebo-tests/01-connect-allow-anonymous-v5.toml",
        ALLOW_CONFIG_V5,
    )?;
    let mut server = Server::start(config.filename())?;
    // TODO(Shaohua): Run ruo client
    sleep(Duration::from_secs(5));
    server.terminate();
    Ok(())
}

#[test]
fn test_conn_deny_anonymous_v4() -> Result<(), Error> {
    let config = ServerConfig::new(
        "/tmp/hebo-tests/01-connect-deny-anonymous-v4.toml",
        DENY_CONFIG_V4,
    )?;
    let mut server = Server::start(config.filename())?;
    sleep(Duration::from_secs(5));
    server.terminate();
    Ok(())
}

#[test]
fn test_conn_deny_anonymous_v5() -> Result<(), Error> {
    let config = ServerConfig::new(
        "/tmp/hebo-tests/01-connect-deny-anonymous-v5.toml",
        DENY_CONFIG_V5,
    )?;
    let mut server = Server::start(config.filename())?;
    sleep(Duration::from_secs(5));
    server.terminate();
    Ok(())
}
