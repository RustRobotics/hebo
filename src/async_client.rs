// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::io;
use super::{
    async_stream::AsyncStream,
    base::*,
    connect_options::*,
    connect_packet::ConnectPacket,
    error::Error,
    publish_packet::PublishPacket,
};

#[derive(Debug)]
pub struct AsyncClient {
    connect_options: ConnectOptions,
    stream: AsyncStream,
}

impl AsyncClient {
    pub async fn connect(connect_options: ConnectOptions) -> io::Result<AsyncClient> {
        let mut stream = AsyncStream::connect(connect_options.address().clone()).await;
        let conn_packet = ConnectPacket::new();
        stream.send(conn_packet).await;
        log::info!("stream send conn packet");

        let client = AsyncClient {
            connect_options,
            stream,
        };

        Ok(client)
    }

    pub async fn publish(&mut self, topic: &str, qos: QoSLevel, data: &[u8]) {
        let mut msg_packet = PublishPacket::new(topic.as_bytes());
        msg_packet.set_message(data).unwrap();
        log::info!("Send publish packet");
        self.stream.send(msg_packet).await;
    }

    pub async fn disconnect(&mut self) {
    }

    pub fn on_connect(&mut self) {
    }

    pub fn on_disconnect(&mut self) {
    }

    pub fn on_message(&mut self) {
    }
}
