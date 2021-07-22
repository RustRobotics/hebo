// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::QoS;
use ruo::connect_options::{ConnectOptions, ConnectType, MqttConnect};
use ruo::sync_client::{Client, ClientStatus};
use std::net::SocketAddr;
use std::time::Duration;

fn on_connect(client: &mut Client) {
    log::info!(
        "[on_connect] client id: {}",
        client.connect_option().client_id()
    );

    client.subscribe("hello", QoS::AtMostOnce).unwrap();
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut options = ConnectOptions::new();
    options.set_connect_type(ConnectType::Mqtt(MqttConnect {
        address: SocketAddr::from(([127, 0, 0, 1], 1883)),
    }));
    let mut client = Client::new(options, Some(on_connect), None);
    client.init().unwrap();
    client.connect().unwrap();

    let mut count = 0;
    let payload = std::include_str!("../src/client.rs");
    loop {
        client.process_events();
        std::thread::sleep(Duration::from_secs(1));
        if client.status() != ClientStatus::Connected {
            continue;
        }

        count += 1;
        if count == 1_000_000 {
            break;
        }
        log::info!("Client connected, publish packet count: {}", count);
        if let Err(err) = client.publish("hello", QoS::AtMostOnce, payload.as_bytes()) {
            log::error!("got error: {:?}", err);
        }
    }
}
