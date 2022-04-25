// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::thread::sleep;
use std::time::Duration;

mod common;
use common::{Error, Server, ServerConfig};

const ALLOW_CONFIG: &str = r#"
[general]
pid_file = "/tmp/hebo-tests/mqtt-1888.pid"

[[listeners]]
protocol = "mqtt"
address = "0.0.0.0:1888"

[security]
allow_anonymous = true

[log]
log_file = "/tmp/hebo-tests/hebo-1888.log"
"#;

const DENY_CONFIG: &str = r#"
[general]
pid_file = "/tmp/hebo-tests/mqtt-1888.pid"

[[listeners]]
protocol = "mqtt"
address = "0.0.0.0:1888"

[security]
allow_anonymous = false

[log]
log_file = "/tmp/hebo-tests/hebo-1888.log"
"#;

#[test]
fn test_conn_allow_anonymous() -> Result<(), Error> {
    let config = ServerConfig::new(
        "/tmp/hebo-tests/01-connect-allow-anonymous.conf",
        ALLOW_CONFIG,
    )?;
    let mut server = Server::start(config.filename())?;
    // TODO(Shaohua): Run ruo client
    sleep(Duration::from_secs(5));
    server.terminate();
    Ok(())
}

#[test]
fn test_conn_deny_anonymous() -> Result<(), Error> {
    let config = ServerConfig::new(
        "/tmp/hebo-tests/01-connect-allow-anonymous.conf",
        DENY_CONFIG,
    )?;
    let mut server = Server::start(config.filename())?;
    sleep(Duration::from_secs(5));
    server.terminate();
    Ok(())
}
