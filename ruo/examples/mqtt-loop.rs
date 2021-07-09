// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::QoS;
use ruo::connect_options::{ConnectOptions, ConnectType, MqttConnect};
use ruo::sync_client::Client;
use std::net::SocketAddr;
use std::time::Instant;

fn on_connect(client: &mut Client) {
    log::info!(
        "[on_connect] client id: {}",
        client.connect_option().client_id()
    );

    //client.subscribe("hello", QoS::AtMostOnce).unwrap();
    let mut count = 0;
    let payload = std::include_str!("../src/client.rs");
    let now = Instant::now();
    loop {
        count += 1;
        if count == 1_000_000 {
            break;
        }
        log::info!("count: {}", count);
        if let Err(err) = client.publish("hello", QoS::AtMostOnce, payload.as_bytes()) {
            log::error!("got error: {:?}", err);
        }
    }
    log::info!("elapsed: {}", now.elapsed().as_millis());
    std::process::exit(0);
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut options = ConnectOptions::new();
    options.set_connect_type(ConnectType::Mqtt(MqttConnect {
        address: SocketAddr::from(([127, 0, 0, 1], 1883)),
    }));
    let mut client = Client::new(options, Some(on_connect), None).unwrap();
    client.start().unwrap();
}
