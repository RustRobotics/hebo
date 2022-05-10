// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

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
    Ws(WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>),
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

impl Drop for Stream {
    fn drop(&mut self) {
        log::info!("Stream::drop()");
        match self {
            Stream::Mqtt(stream) => drop(stream),
            Stream::Ws(stream) => drop(stream),
            #[cfg(unix)]
            Stream::Uds(stream) => drop(stream),
        }
    }
}

impl Stream {
    pub fn new(connect_type: &ConnectType) -> Result<Self, Error> {
        match connect_type {
            ConnectType::Mqtt(mqtt_connect) => Stream::new_mqtt(mqtt_connect),
            ConnectType::Ws(ws_connect) => Stream::new_ws(ws_connect),
            #[cfg(unix)]
            ConnectType::Uds(uds_connect) => Stream::new_uds(uds_connect),
            _ => todo!(),
        }
    }

    fn new_mqtt(mqtt_connect: &MqttConnect) -> Result<Self, Error> {
        let stream = TcpStream::connect(mqtt_connect.address)?;
        Ok(Stream::Mqtt(stream))
    }

    fn new_ws(ws_connect: &WsConnect) -> Result<Self, Error> {
        let ws_url = format!("ws://{}{}", ws_connect.address, &ws_connect.path);
        let (ws_stream, _resp) = tungstenite::connect(ws_url)?;
        Ok(Stream::Ws(ws_stream))
    }

    #[cfg(unix)]
    fn new_uds(uds_connect: &UdsConnect) -> Result<Stream, Error> {
        let uds_stream = UnixStream::connect(&uds_connect.sock_path)?;
        Ok(Stream::Uds(uds_stream))
    }

    pub fn read_buf(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        match self {
            Stream::Mqtt(stream) => {
                // let reference = std::io::Read::by_ref(socket);
                // reference.take(buf.capacity() as u64).read_to_end(buf)
                stream.read(buf).map_err(|err| err.into())
            }

            Stream::Ws(ws_stream) => {
                let msg = ws_stream.read_message()?;
                let data = msg.into_data();
                let data_len = data.len();
                buf.extend(data);
                Ok(data_len)
            }
            #[cfg(unix)]
            Stream::Uds(uds_stream) => uds_stream.read(buf).map_err(|err| err.into()),
        }
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<usize, Error> {
        match self {
            Stream::Mqtt(stream) => {
                stream.write_all(buf)?;
                Ok(buf.len())
            }

            Stream::Ws(ws_stream) => {
                let msg = Message::binary(buf);
                ws_stream.write_message(msg)?;
                Ok(buf.len())
            }

            #[cfg(unix)]
            Stream::Uds(uds_stream) => {
                uds_stream.write_all(buf)?;
                Ok(buf.len())
            }
        }
    }
}
