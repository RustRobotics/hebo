// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::thread::sleep;
use std::time::Duration;

mod common;
use common::{Error, Server, ServerConfig};

const CONFIG: &str = r#"
[general]
pid_file = "/tmp/hebo-tests/mqtt-1888.pid"

[[listeners]]
protocol = "mqtt"
address = "0.0.0.0:1888"

[log]
log_file = "/tmp/hebo-tests/hebo-1888.log"
"#;

#[test]
fn test_conn_max_connections() -> Result<(), Error> {
    let config = ServerConfig::new("/tmp/hebo-tests/01-connect-allow-anonymous.conf", CONFIG)?;
    let mut server = Server::start(config.filename())?;
    sleep(Duration::from_secs(10));
    server.terminate();
    Ok(())
}
