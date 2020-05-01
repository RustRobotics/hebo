// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::fs::File;
use std::io;
use std::io::Read;
use std::net::SocketAddr;

use native_tls::Certificate;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tls::TlsStream;

use crate::connect_options::{ConnectType, MqttsConnect, TlsType};

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

    async fn new_mqtt(address: &SocketAddr) -> io::Result<Stream> {
        let socket = TcpStream::connect(address).await?;
        Ok(Stream::Mqtt(socket))
    }

    async fn new_mqtts(address: &SocketAddr, mqtts_connect: &MqttsConnect) -> io::Result<Stream> {
        log::info!("new_mqtts(): {:?}", address);
        // TODO(Shaohua): Convert error types.
        let mut builder = native_tls::TlsConnector::builder();
        if let TlsType::SelfSigned(self_signed) = &mqtts_connect.tls_type {
            let mut root_ca_fd = File::open(&self_signed.root_ca_pem)?;
            let mut root_ca_buf = Vec::new();
            root_ca_fd.read_to_end(&mut root_ca_buf)?;
            let root_ca = Certificate::from_pem(&root_ca_buf).unwrap();
            builder.add_root_certificate(root_ca);
        }
        let connector = builder.build().unwrap();
        let connector = tokio_tls::TlsConnector::from(connector);
        let socket = TcpStream::connect(address).await?;
        let socket = connector.connect(&mqtts_connect.domain, socket).await.unwrap();
        Ok(Stream::Mqtts(socket))
    }

    pub async fn read_buf(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        match self {
            Stream::Mqtt(socket) => socket.read_buf(buf).await,
            Stream::Mqtts(tls_socket) => tls_socket.read(buf).await,
        }
    }

    pub async fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self {
            Stream::Mqtt(socket) => socket.write_all(buf).await,
            Stream::Mqtts(tls_socket) => {
                log::info!("write_all(): {:x?}", buf);
                tls_socket.write_all(buf).await
            },
        }
    }
}