// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{ProtocolLevel, QoS};
use ruo::blocking::client::Client;
use ruo::connect_options::{ConnectOptions, ConnectType, MqttConnect};
use std::net::SocketAddr;

use ruo::error::Error;

fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut options = ConnectOptions::new();
    options.set_connect_type(ConnectType::Mqtt(MqttConnect {
        address: SocketAddr::from(([127, 0, 0, 1], 1883)),
    }));
    let mut client = Client::new(options, ProtocolLevel::V31);
    let is_connected = client.connect()?;
    log::info!("is connected");
    assert!(is_connected);
    log::info!(
        "Connected to server, client id: {}",
        client.connect_options().client_id()
    );

    client.subscribe("hello", QoS::AtMostOnce)?;
    client.publish("hello", QoS::AtMostOnce, b"Hello, world")?;

    client.disconnect()?;

    Ok(())
}
