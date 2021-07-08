// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::QoS;
use ruo::connect_options::ConnectOptions;
use ruo::sync_client::Client;

fn on_connect(client: &mut Client) {
    log::info!(
        "[on_connect] client id: {}",
        client.connect_option().client_id()
    );

    // self.subscribe("hello", QoS::AtMostOnce).await;
    client.subscribe("hello", QoS::AtMostOnce).unwrap();
    client
        .publish("hello", QoS::AtMostOnce, b"Hello, world")
        .unwrap();
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let address = "127.0.0.1:1883";
    let options = ConnectOptions::new(address).unwrap();
    log::info!("options: {:?}", options);
    let mut client = Client::new(options, Some(on_connect), None).unwrap();
    client.start().unwrap();
}
