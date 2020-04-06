// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use super::base::*;
use super::connect_options::*;
use super::error::Error;
use std::io;
use super::connect_packet::ConnectPacket;
use super::publish_packet::PublishPacket;
use super::stream::{Stream, SyncStream};

#[derive(Debug)]
pub struct Client {
    connect_options: ConnectOptions,
    stream: SyncStream,
}

impl Client {
    pub fn connect(connect_options: ConnectOptions) -> io::Result<Client> {
        let mut stream = SyncStream::connect(connect_options.address().clone())?;
        let conn_packet = ConnectPacket::new();
        stream.send(conn_packet);

        let client = Client {
            connect_options,
            stream,
        };

        Ok(client)
    }

    pub fn publish(&mut self, topic: &str, qos: QoSLevel, data: &[u8]) {
        let mut msg_packet = PublishPacket::new(topic.as_bytes());
        msg_packet.set_message(data).unwrap();
        self.stream.send(msg_packet);
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
