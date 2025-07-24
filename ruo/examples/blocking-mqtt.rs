// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::QoS;
use ruo::blocking::client::Client;
use ruo::connect_options::ConnectOptions;

use ruo::error::Error;

fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut client = Client::new(ConnectOptions::default());
    client.connect()?;
    log::info!(
        "Connected to server, client id: {}",
        client.connect_options().client_id()
    );

    client.subscribe("hello", QoS::AtMostOnce)?;
    client.publish("hello", QoS::AtMostOnce, b"Hello, world")?;
    loop {
        if let Some(message) = client.wait_for_message()? {
            log::info!("got message: {:?}", message);
            break;
        } else {
            log::info!("No message");
        }
    }

    // If client.disconnect() is not called explicitly, it will be called in
    // drop().
    //client.disconnect()?;

    Ok(())
}
