// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::base::*;
use super::connect_options::*;
use super::error::Error;
use super::stream::AsyncStream;

#[derive(Debug)]
pub struct Client {
    conn_options: ConnectOptions,
    stream: AsyncStream,
}

impl Client {
    pub fn connect(option: ConnectOptions) -> Result<Client, Error> {
        let stream = AsyncStream::new(option.address().clone());
        let client = Client {
            conn_options: option,
            stream: stream,
        };

        Ok(client)
    }

    pub fn publish(&mut self, topic: &str, qos: QoSLevel, data: &[u8]) {
    }

    pub fn disconnect(&mut self) {
    }

    pub fn on_connect(&mut self) {
    }

    pub fn on_disconnect(&mut self) {
    }

    pub fn on_message(&mut self) {
    }
}
