// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use hebo::error::Error;
use std::thread::sleep;
use std::time::Duration;

mod common;
use common::{Server, ServerConfig};

const CONFIG: &str = r#"
[general]
pid_file = "/tmp/hebo-tests/mqtt-1893.pid"
max_connections = 10

[[listeners]]
protocol = "mqtt"
address = "0.0.0.0:1893"

[security]
allow_anonymous = true

[dashboard]
enable = false

[log]
log_file = "/tmp/hebo-tests/hebo-1893.log"
"#;

#[test]
fn test_connect_max_connections() -> Result<(), Error> {
    let config = ServerConfig::new("/tmp/hebo-tests/01-connect-max-connections.toml", CONFIG)?;
    let mut server = Server::start(config.filename())?;
    sleep(Duration::from_secs(5));
    server.terminate();
    Ok(())
}
