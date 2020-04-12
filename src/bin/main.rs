// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use hebo::server_context::ServerContext;
use std::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut server = ServerContext::new();
    let mut listener = TcpListener::bind("127.0.0.1:1883").await?;
    loop {
        log::info!("accept()");
        match listener.accept().await {
            Ok((socket, address)) => {
                log::info!("remote address: {:?}", address);
                server.new_connection(socket, address).await;
            }
            Err(err) => log::error!("Failed to accept incoming connection: {:?}", err),
        }
    }
}
