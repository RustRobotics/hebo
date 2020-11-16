// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use ruo::{async_client::AsyncClient, connect_options::ConnectOptions};
use ruo::connect_options::{ConnectType, MqttsConnect, TlsType};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let address = "hebo.biofan.org:8883";
    let mut options = ConnectOptions::new(address).unwrap();
    options.set_connect_type(ConnectType::Mqtts(MqttsConnect {
        tls_type: TlsType::CaSigned,
        domain: "hebo.biofan.org".to_string(),
    }));
    log::info!("options: {:?}", options);
    let mut client = AsyncClient::new(options).await;
    client.start().await;
}
