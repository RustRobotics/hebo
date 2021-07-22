// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::connect_options::{ConnectType, MqttConnect};

#[derive(Debug)]
pub enum Stream {
    Mqtt(TcpStream),
}

impl Drop for Stream {
    fn drop(&mut self) {
        log::info!("Stream::drop()");
        match self {
            Stream::Mqtt(stream) => drop(stream),
        }
    }
}

impl Stream {
    pub fn new(connect_type: &ConnectType) -> io::Result<Stream> {
        match connect_type {
            ConnectType::Mqtt(mqtt_connect) => Stream::new_mqtt(mqtt_connect),
            _ => unimplemented!(),
        }
    }

    fn new_mqtt(mqtt_connect: &MqttConnect) -> io::Result<Stream> {
        let socket = TcpStream::connect(mqtt_connect.address)?;
        // Enable non-blocking mode.
        socket.set_nonblocking(true)?;
        Ok(Stream::Mqtt(socket))
    }

    pub fn read_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Stream::Mqtt(socket) => {
                // let reference = std::io::Read::by_ref(socket);
                // reference.take(buf.capacity() as u64).read_to_end(buf)
                socket.read(buf)
            }
        }
    }

    pub fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self {
            Stream::Mqtt(socket) => socket.write_all(buf),
        }
    }
}
