// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io;

use hebo::server_context::ServerContext;

#[tokio::main]
async fn main() -> io::Result<()> {
    // std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut server = ServerContext::new("127.0.0.1:1883");
    server.run_loop().await?;
    Ok(())
}
