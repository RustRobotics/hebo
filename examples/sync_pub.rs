// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use ruo::{
    Client,
    ConnectOptions,
    QoSLevel,
};

fn main() {
    let addr = "127.0.0.1:1883";
    let options = ConnectOptions::new(addr).unwrap();
    let mut client = Client::connect(options).unwrap();
    client.publish("hello", QoSLevel::QoS0, b"Hello, world");
}
