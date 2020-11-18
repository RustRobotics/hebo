// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

mod protos;

use codec::base::QoS;
use codec::publish_packet::PublishPacket;
use protobuf::Message;
use ruo::client::Client;
use ruo::connect_options::ConnectOptions;

use protos::geometry::Geometry;

fn on_connect(client: &mut Client) {
    log::info!(
        "[on_connect] client id: {}",
        client.connect_option().client_id()
    );

    // client.subscribe("device/42/geometry", QoS::AtMostOnce).unwrap();
    let mut rect = Geometry::new();
    rect.set_x(1);
    rect.set_y(4);
    rect.set_width(960);
    rect.set_height(720);
    let buf: Vec<u8> = rect.write_to_bytes().unwrap();
    loop {
        log::info!("Publish now");
        client.publish("device/42/geometry", QoS::AtMostOnce, &buf).unwrap();
    }
}

fn on_message(_client: &mut Client, packet: &PublishPacket) {
    log::info!(
        "Got message: {:?}, topic: {}",
        packet.message(),
        packet.topic()
    );
    match protobuf::parse_from_bytes::<Geometry>(packet.message()) {
        Ok(geometry) => {
            log::info!("geometry: {:?}", geometry);
        }
        Err(err) => {
            log::error!("Failed to parse pub msg: {:?}", err);
        }
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let address = "127.0.0.1:1883";
    let options = ConnectOptions::new(address).unwrap();
    log::info!("options: {:?}", options);
    let mut client = Client::new(options, Some(on_connect), Some(on_message)).unwrap();
    client.start().unwrap();
}
