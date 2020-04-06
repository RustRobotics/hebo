// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use tokio::{
    net::TcpStream,
    prelude::*,
    };

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:1883";
    let mut stream = TcpStream::connect(addr).await.unwrap();
    log::info!("stream created");
}
