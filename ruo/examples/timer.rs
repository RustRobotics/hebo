// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::time::Duration;

use tokio::time::interval;

#[tokio::main]
async fn main() {
    let mut timer = interval(Duration::from_millis(500));
    loop {
        println!("tick()");
        timer.tick().await;
    }
}
