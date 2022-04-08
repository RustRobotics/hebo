// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub mod hello_world {
    tonic::include_proto!("hello_world");
}

pub mod connection {
    tonic::include_proto!("connection");
}