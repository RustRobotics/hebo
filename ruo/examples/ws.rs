// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use ruo::{async_client::AsyncClient, connect_options::ConnectOptions};
use ruo::connect_options::{WsConnect, ConnectType};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let address = "127.0.0.1:8083";
    let mut options = ConnectOptions::new(address).unwrap();
    options.set_connect_type(ConnectType::Ws(WsConnect { path: "/mqtt".to_string() }));
    log::info!("options: {:?}", options);
    let mut client = AsyncClient::new(options).await;
    client.start().await;
}
