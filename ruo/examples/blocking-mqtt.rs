// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::QoS;
use ruo::blocking::client::Client;
use ruo::connect_options::{ConnectOptions, ConnectType, MqttConnect};
use std::net::SocketAddr;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut options = ConnectOptions::new();
    options.set_connect_type(ConnectType::Mqtt(MqttConnect {
        address: SocketAddr::from(([127, 0, 0, 1], 1883)),
    }));
    let mut client = Client::new(options, Some(on_connect), None).unwrap();
    let ret = client.connect();
    assert!(ret.is_ok());
    log::info!(
        "Connected to server, client id: {}",
        client.connect_option().client_id()
    );

    client.subscribe("hello", QoS::AtMostOnce).unwrap();
    client
        .publish("hello", QoS::AtMostOnce, b"Hello, world")
        .unwrap();
}
