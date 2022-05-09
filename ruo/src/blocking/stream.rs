// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::fmt;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::os::unix::net::UnixStream;

use crate::connect_options::{ConnectType, MqttConnect, UdsConnect};
use crate::error::{Error, ErrorKind};

pub enum Stream {
    Mqtt(TcpStream),
}

impl fmt::Debug for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mqtt(..) => f.write_str("Mqtt"),
        }
    }
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
    pub fn new(connect_type: &ConnectType) -> Result<Self, Error> {
        match connect_type {
            ConnectType::Mqtt(mqtt_connect) => Stream::new_mqtt(mqtt_connect),
            _ => todo!(),
        }
    }

    fn new_mqtt(mqtt_connect: &MqttConnect) -> Result<Self, Error> {
        let socket = TcpStream::connect(mqtt_connect.address)?;
        Ok(Stream::Mqtt(socket))
    }

    pub fn read_buf(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        match self {
            Stream::Mqtt(socket) => {
                // let reference = std::io::Read::by_ref(socket);
                // reference.take(buf.capacity() as u64).read_to_end(buf)
                socket.read(buf).map_err(|err| {
                    Error::from_string(
                        ErrorKind::SocketError,
                        format!(
                            "Failed to read from mqtt stream, buffer len: {}, err: {:?}",
                            buf.len(),
                            err
                        ),
                    )
                })
            }
        }
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        match self {
            Stream::Mqtt(socket) => socket.write_all(buf).map_err(|err| {
                Error::from_string(
                    ErrorKind::SocketError,
                    format!(
                        "Failed to write {} bytes to mqtt stream, err: {:?}",
                        buf.len(),
                        err
                    ),
                )
            }),
        }
    }
}
