// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io;

use hebo::config::Config;
use hebo::server_context::ServerContext;

#[tokio::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config: Config = toml::from_str("").unwrap();
    let mut server = ServerContext::new(config);
    server.run_loop().await?;
    Ok(())
}
