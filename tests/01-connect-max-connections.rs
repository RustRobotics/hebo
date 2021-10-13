// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

mod common;
use common::Server;

#[test]
fn test_conn_max_connections() {
    let server = Server::start("examples/mqtt.toml");
    assert!(server.is_ok());
    let mut server = server.unwrap();
    //let ret = nc::pause();
    // assert!(ret.is_err());
    std::thread::sleep(std::time::Duration::from_secs(3));
    server.terminate();
}
