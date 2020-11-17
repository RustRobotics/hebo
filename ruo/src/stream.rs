// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io::{self, Write, Read};
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;

use crate::connect_options::ConnectType;

#[derive(Debug)]
pub enum Stream {
    Mqtt(TcpStream),
}

impl Drop for Stream {
    fn drop(&mut self) {
        match self {
            Stream::Mqtt(socket) => drop(socket),
        }
    }
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
        socket
            .set_read_timeout(Some(Duration::from_secs(20)))
            .unwrap();
        socket
            .set_write_timeout(Some(Duration::from_secs(20)))
            .unwrap();
        Ok(Stream::Mqtt(socket))
    }

    pub fn read_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Stream::Mqtt(socket) => {
                // let reference = std::io::Read::by_ref(socket);
                // reference.take(buf.capacity() as u64).read_to_end(buf)
                socket.read(buf)
            },
        }
    }

    pub fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        log::info!("write_all(): {:?}", buf);

        match self {
            Stream::Mqtt(socket) => socket.write_all(buf),
        }
    }
}
