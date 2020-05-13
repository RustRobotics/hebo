// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::env;

use ruo::{async_client::AsyncClient, connect_options::ConnectOptions};
use ruo::connect_options::{ConnectType, MqttsConnect, SelfSignedTls, TlsType};
use std::ffi::OsString;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let address = "127.0.0.1:8883";
    let mut options = ConnectOptions::new(address).unwrap();
    let args: Vec<String> = env::args().collect();
    options.set_connect_type(ConnectType::Mqtts(MqttsConnect {
        tls_type: TlsType::SelfSigned(SelfSignedTls {
            root_ca_pem: OsString::from(&args[1]),
            cert_pem: OsString::from(&args[2]),
            private_key_pem: OsString::from(&args[3]),
        }),
        domain: "broker.biofan.org".to_string(),
    }));
    log::info!("options: {:?}", options);
    let mut client = AsyncClient::new(options).await;
    client.start().await;
}
