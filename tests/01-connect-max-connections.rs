// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

mod common;
use common::Server;

#[test]
fn test_conn_max_connections() {
    let server = Server::start();
    assert!(server.is_ok());
    let mut server = server.unwrap();
    server.terminate();
}
