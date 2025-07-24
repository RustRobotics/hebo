// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Test whether `max_keepalive` violations are rejected for MQTT < 5.0.

use hebo::error::Error;
use std::thread::sleep;
use std::time::Duration;

mod common;
use common::{Server, ServerConfig};

const CONFIG: &str = r#"
[general]
pid_file = "/tmp/hebo-tests/mqtt-1894.pid"
max_keepalive = 10

[[listeners]]
protocol = "mqtt"
address = "0.0.0.0:1894"

[security]
allow_anonymous = true

[dashboard]
enable = false

[log]
log_file = "/tmp/hebo-tests/hebo-1894.log"
"#;

#[test]
fn test_connect_max_keepalive() -> Result<(), Error> {
    let config = ServerConfig::new("/tmp/hebo-tests/01-connect-max-keepalive.toml", CONFIG)?;
    let server = Server::start(config.filename())?;
    sleep(Duration::from_secs(5));
    server.terminate();
    Ok(())
}
