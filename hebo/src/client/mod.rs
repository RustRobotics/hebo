// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug)]
pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }
}

impl Client {
    pub fn connect(&mut self) -> bool {
        false
    }

    pub fn close(&mut self) -> bool {
        false
    }

    pub fn gen_connack(&mut self) {
        unimplemented!()
    }
}
