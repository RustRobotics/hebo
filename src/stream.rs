// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::net::{SocketAddr};

#[derive(Debug)]
pub struct AsyncStream {
    addr: SocketAddr,
}

impl AsyncStream {
    pub fn new(addr: SocketAddr) -> AsyncStream {
        AsyncStream {
            addr,
        }
    }
}
