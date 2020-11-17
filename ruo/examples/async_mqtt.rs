// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::base::QoS;
use ruo::async_client::AsyncClient;
use ruo::connect_options::ConnectOptions;
use std::future::Future;

async fn on_connect(client: &mut AsyncClient) {
    log::info!(
        "[on_connect] client id: {}",
        client.connect_option().client_id()
    );

    // self.subscribe("hello", QoS::AtMostOnce).await;
    client.subscribe("hello", QoS::AtMostOnce).await;
    client
        .publish("hello", QoS::AtMostOnce, b"Hello, world")
        .await
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let address = "127.0.0.1:1883";
    let options = ConnectOptions::new(address).unwrap();
    log::info!("options: {:?}", options);
    let mut client = AsyncClient::new(options).await;
    client.set_connect_callback(Box::new(on_connect));
    client.start().await;
}
