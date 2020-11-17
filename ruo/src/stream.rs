// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::Read;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::net::TcpStream;

use crate::connect_options::ConnectType;

#[derive(Debug)]
pub enum Stream {
    Mqtt(TcpStream),
}

impl Stream {
    pub fn new(address: &SocketAddr, connect_type: &ConnectType) -> io::Result<Stream> {
        match connect_type {
            ConnectType::Mqtt(_) => Stream::new_mqtt(address),
            _ => unimplemented!(),
        }
    }

    fn new_mqtt(address: &SocketAddr) -> io::Result<Stream> {
        let socket = TcpStream::connect(address)?;
        Ok(Stream::Mqtt(socket))
    }

    pub fn read_buf(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        match self {
            Stream::Mqtt(socket) => socket.read(buf),
        }
    }

    pub fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        log::info!("write_all(): {:?}", buf);

        match self {
            Stream::Mqtt(socket) => socket.write_all(buf),
        }
    }
}
