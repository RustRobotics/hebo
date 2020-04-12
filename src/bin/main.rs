// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use hebo::server_context::ServerContext;
use std::io;
use tokio::net::{TcpListener, TcpStream};

async fn process_socket(socket: TcpStream) {
    log::info!("process socket!");
}

#[tokio::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "log");
    env_logger::init();

    let server = ServerContext::new();
    let mut listener = TcpListener::bind("127.0.0.1:1883").await?;
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                // TODO(Shaohua): Spawn a sub task and create a ConnectContext
                process_socket(socket).await;
            }
            Err(err) => log::error!("Failed to accept incoming connection: {:?}", err),
        }
    }
}
