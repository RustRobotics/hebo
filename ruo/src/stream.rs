// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::io;
use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tls::TlsStream;
use crate::connect_options::{ConnectType, MqttsConnect};

#[derive(Debug)]
pub enum Stream {
    Mqtt(TcpStream),
    Mqtts(TlsStream<TcpStream>),
}

impl Stream {
    pub async fn new(address: &SocketAddr, connect_type: &ConnectType) -> io::Result<Stream> {
        match connect_type {
            ConnectType::Mqtt(_) => Stream::new_mqtt(address).await,
            ConnectType::Mqtts(mqtts) => Stream::new_mqtts(address, mqtts).await,
            _ => unimplemented!(),
        }
    }

    pub async fn new_mqtt(address: &SocketAddr) -> io::Result<Stream> {
        let socket = TcpStream::connect(address).await?;
        Ok(Stream::Mqtt(socket))
    }

    pub async fn new_mqtts(address: &SocketAddr, mqtts_connect: &MqttsConnect) -> io::Result<Stream> {
        // TODO(Shaohua): Convert error types.
        let builder = native_tls::TlsConnector::builder();
        let connector = builder.build().unwrap();
        let connector = tokio_tls::TlsConnector::from(connector);
        // TODO(Shaohua): Support self signed cert.
        let socket = TcpStream::connect(address).await?;
        let socket = connector.connect(&mqtts_connect.domain, socket).await.unwrap();
        Ok(Stream::Mqtts(socket))
    }

    pub async fn read_buf(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        match self {
            Stream::Mqtt(socket) => socket.read_buf(buf).await,
            _ => Ok(0),
        }
    }

    pub async fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self {
            Stream::Mqtt(socket) => socket.write_all(buf).await,
            _ => Ok(()),
        }
    }
}