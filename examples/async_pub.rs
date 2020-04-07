// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use ruo::async_client::AsyncDelegate;
use ruo::{AsyncClient, ConnectOptions, QoSLevel};

#[derive(Debug)]
struct Delegate {}

impl Delegate {
    pub fn new() -> Delegate {
        Delegate {}
    }
}

impl AsyncDelegate for Delegate {
    fn on_connect(&self) {
        log::info!("on_connect()");
    }

    fn on_message(&self, buf: &[u8]) {
        log::info!("on message: {}", buf.len());
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let addr = "127.0.0.1:1883";
    let options = ConnectOptions::new(addr).unwrap();
    let delegate = Box::new(Delegate::new());
    let mut client = AsyncClient::new(options, Some(delegate)).await;
    client.start().await;
}
