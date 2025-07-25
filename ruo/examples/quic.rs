// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::QoS;
use ruo::client::Client;
use ruo::connect_options::{ConnectOptions, ConnectType, QuicConnect, SelfSignedTls, TlsType};
use std::path::PathBuf;

async fn on_connect(client: &mut Client) {
    log::info!(
        "[on_connect] client id: {}",
        client.connect_options().client_id()
    );

    client
        .subscribe("hello", QoS::AtMostOnce)
        .await
        .expect("Failed to subscribe");
    client
        .publish("hello", QoS::AtMostOnce, b"Hello, world")
        .await
        .expect("Failed to publish");
}

#[tokio::main]
async fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let mut options = ConnectOptions::new();
    let tls_type = TlsType::SelfSigned(SelfSignedTls {
        cert: PathBuf::from("../hebo/examples/certs/cert.pem"),
    });
    options.set_connect_type(ConnectType::Quic(QuicConnect {
        client_address: "127.0.0.1:0".parse().unwrap(),
        server_address: "127.0.0.1:8993".parse().unwrap(),
        domain: "localhost".to_owned(),
        tls_type,
    }));
    let mut client = Client::new(options);
    //client.set_connect_callback(Box::new(on_connect));
    client.connect().await.expect("Failed to start");
    client.run_loop().await;
}
