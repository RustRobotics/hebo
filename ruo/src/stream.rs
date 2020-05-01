// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::io;
use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tls::TlsStream;

#[derive(Debug)]
pub enum Stream {
    Tcp(TcpStream),
    SecureTcp(TlsStream<TcpStream>),
}

impl Stream {
    pub async fn new_tcp(addr: &SocketAddr) -> Result<Stream, io::Error> {
        let socket = TcpStream::connect(addr).await?;
        Ok(Stream::Tcp(socket))
    }

    pub async fn read_buf(&mut self, buf: &mut Vec<u8>) -> Result<usize, io::Error> {
        match self {
            Stream::Tcp(socket) => socket.read_buf(buf).await,
            _ => Ok(0),
        }
    }

    pub async fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
        match self {
            Stream::Tcp(socket) => socket.write_all(buf).await,
            _ => Ok(()),
        }
    }
}