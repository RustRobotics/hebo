// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{self, tungstenite::protocol::Message, WebSocketStream};

use crate::connect_options::{ConnectType, WsConnect};
use crate::error::Error;

pub enum Stream {
    Mqtt(TcpStream),
    //Mqtts(TlsStream<TcpStream>),
    Ws(WebSocketStream<TcpStream>),
}

impl Stream {
    pub async fn new(address: &SocketAddr, connect_type: &ConnectType) -> Result<Stream, Error> {
        match connect_type {
            ConnectType::Mqtt(_) => Stream::new_mqtt(address).await,
            //ConnectType::Mqtts(mqtts_connect) => Stream::new_mqtts(address, mqtts_connect).await,
            ConnectType::Ws(ws_connect) => Stream::new_ws(address, ws_connect).await,
            _ => unimplemented!(),
        }
    }

    async fn new_mqtt(address: &SocketAddr) -> Result<Stream, Error> {
        let tcp_stream = TcpStream::connect(address).await?;
        Ok(Stream::Mqtt(tcp_stream))
    }

    /*
    async fn new_mqtts(
        address: &SocketAddr,
        mqtts_connect: &MqttsConnect,
    ) -> Result<Stream, Error> {
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
        let socket = connector
            .connect(&mqtts_connect.domain, socket)
            .await
            .unwrap();
        Ok(Stream::Mqtts(socket))
    }
    */

    async fn new_ws(address: &SocketAddr, ws_connect: &WsConnect) -> Result<Stream, Error> {
        let ws_url = format!("ws://{}{}", address, &ws_connect.path);
        let tcp_stream = TcpStream::connect(address).await?;
        let (ws_stream, _) = tokio_tungstenite::client_async(ws_url, tcp_stream).await?;
        Ok(Stream::Ws(ws_stream))
    }

    pub async fn read_buf(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        match self {
            Stream::Mqtt(socket) => Ok(socket.read_buf(buf).await?),
            //Stream::Mqtts(tls_socket) => tls_socket.read(buf).await,
            Stream::Ws(ref mut ws_stream) => {
                if let Some(msg) = ws_stream.next().await {
                    let msg = msg?;
                    let data = msg.into_data();
                    let data_len = data.len();
                    buf.extend(data);
                    Ok(data_len)
                } else {
                    Ok(0)
                }
            }
        }
    }

    pub async fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        match self {
            Stream::Mqtt(tcp_stream) => Ok(tcp_stream.write(buf).await?),
            //            Stream::Mqtts(tls_socket) => tls_socket.write_all(buf).await,
            Stream::Ws(ws_stream) => {
                let msg = Message::binary(buf);
                ws_stream.send(msg).await?;
                Ok(buf.len())
            }
        }
    }
}
