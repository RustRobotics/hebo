// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::convert::Into;
use std::fmt;
use std::io::{Read, Write};
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use tungstenite::{Message, WebSocket};

#[cfg(unix)]
use crate::connect_options::UdsConnect;
use crate::connect_options::{ConnectType, MqttConnect, WsConnect};
use crate::error::Error;

pub enum Stream {
    Mqtt(TcpStream),
    Ws(Box<WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>>),
    #[cfg(unix)]
    Uds(UnixStream),
}

impl fmt::Debug for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mqtt(..) => f.write_str("Mqtt"),
            Self::Ws(..) => f.write_str("Ws"),
            #[cfg(unix)]
            Self::Uds(..) => f.write_str("Uds"),
        }
    }
}

impl Stream {
    /// Create a new stream based on `connect_type`.
    ///
    /// # Errors
    ///
    /// Returns error if failed to initialize local socket or connect remote socket.
    pub fn new(connect_type: &ConnectType) -> Result<Self, Error> {
        match connect_type {
            ConnectType::Mqtt(mqtt_connect) => Self::new_mqtt(mqtt_connect),
            ConnectType::Ws(ws_connect) => Self::new_ws(ws_connect),
            #[cfg(unix)]
            ConnectType::Uds(uds_connect) => Self::new_uds(uds_connect),
            _ => unimplemented!(),
        }
    }

    fn new_mqtt(mqtt_connect: &MqttConnect) -> Result<Self, Error> {
        let stream = TcpStream::connect(mqtt_connect.address)?;
        Ok(Self::Mqtt(stream))
    }

    fn new_ws(ws_connect: &WsConnect) -> Result<Self, Error> {
        let ws_url = format!("ws://{}{}", ws_connect.address, &ws_connect.path);
        let (ws_stream, _resp) = tungstenite::connect(ws_url)?;
        Ok(Self::Ws(Box::new(ws_stream)))
    }

    #[cfg(unix)]
    fn new_uds(uds_connect: &UdsConnect) -> Result<Self, Error> {
        let uds_stream = UnixStream::connect(&uds_connect.sock_path)?;
        Ok(Self::Uds(uds_stream))
    }

    /// Pull some bytes from this source into the specified buffer, returning how many bytes were read.
    ///
    /// # Errors
    ///
    /// If this function encounters any form of I/O or other error, an error variant will be returned.
    /// If an error is returned then it must be guaranteed that no bytes were read.
    pub fn read_buf(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        match self {
            Self::Mqtt(stream) => {
                // let reference = std::io::Read::by_ref(socket);
                // reference.take(buf.capacity() as u64).read_to_end(buf)
                stream.read(buf).map_err(Into::into)
            }

            Self::Ws(ws_stream) => {
                let msg = ws_stream.read()?;
                let data = msg.into_data();
                let data_len = data.len();
                buf.extend(data);
                Ok(data_len)
            }
            #[cfg(unix)]
            Self::Uds(uds_stream) => uds_stream.read(buf).map_err(Into::into),
        }
    }

    /// Write buffers to stream.
    ///
    /// # Errors
    ///
    /// Each call to write may generate an I/O error indicating that the operation could not be completed.
    /// If an error is returned then no bytes in the buffer were written to this writer.
    pub fn write_all(&mut self, buf: &[u8]) -> Result<usize, Error> {
        // TODO(Shaohua): Replace with io::Write trait.
        match self {
            Self::Mqtt(stream) => {
                stream.write_all(buf)?;
                Ok(buf.len())
            }

            Self::Ws(ws_stream) => {
                let msg = Message::binary(buf);
                ws_stream.send(msg)?;
                Ok(buf.len())
            }

            #[cfg(unix)]
            Self::Uds(uds_stream) => {
                uds_stream.write_all(buf)?;
                Ok(buf.len())
            }
        }
    }
}
