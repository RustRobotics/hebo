// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/hello_world.proto", "proto/connection.proto"],
            &["proto"],
        )
        .unwrap();
}
