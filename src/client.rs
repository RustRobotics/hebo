// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::{
    base::*, connect_options::*, connect_packet::ConnectPacket, publish_packet::PublishPacket,
    sync_stream::SyncStream,
};
use std::io;

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

    pub fn publish(&mut self, topic: &str, qos: QoS, data: &[u8]) {
        let _packet = PublishPacket::new(topic, qos, data);
        //self.stream.send(msg_packet);
    }

    pub fn disconnect(&mut self) {}

    pub fn on_connect(&mut self) {}

    pub fn on_disconnect(&mut self) {}

    pub fn on_message(&mut self) {}
}
