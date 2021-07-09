// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UnixStream};
use tokio_rustls::client::TlsStream;
use tokio_rustls::{rustls::ClientConfig, webpki::DNSNameRef, TlsConnector};
use tokio_tungstenite::{self, tungstenite::protocol::Message, WebSocketStream};

use crate::connect_options::{
    ConnectType, MqttsConnect, TlsType, UdsConnect, WsConnect, WssConnect,
};
use crate::error::Error;

pub enum Stream {
    Mqtt(TcpStream),
    Mqtts(TlsStream<TcpStream>),
    Ws(WebSocketStream<TcpStream>),
    Wss(WebSocketStream<TlsStream<TcpStream>>),
    Uds(UnixStream),
}

impl Stream {
    pub async fn new(address: &SocketAddr, connect_type: &ConnectType) -> Result<Stream, Error> {
        match connect_type {
            ConnectType::Mqtt(..) => Stream::new_mqtt(address).await,
            ConnectType::Mqtts(mqtts_connect) => Stream::new_mqtts(address, mqtts_connect).await,
            ConnectType::Ws(ws_connect) => Stream::new_ws(address, ws_connect).await,
            ConnectType::Wss(wss_connect) => Stream::new_wss(address, wss_connect).await,
            ConnectType::Uds(uds_connect) => Stream::new_uds(uds_connect).await,
        }
    }

    async fn new_mqtt(address: &SocketAddr) -> Result<Stream, Error> {
        let tcp_stream = TcpStream::connect(address).await?;
        Ok(Stream::Mqtt(tcp_stream))
    }

    async fn new_mqtts(
        address: &SocketAddr,
        mqtts_connect: &MqttsConnect,
    ) -> Result<Stream, Error> {
        let mut config = ClientConfig::new();
        match &mqtts_connect.tls_type {
            TlsType::SelfSigned(self_signed) => {
                let mut pem = BufReader::new(File::open(&self_signed.root_ca)?);
                config
                    .root_store
                    .add_pem_file(&mut pem)
                    .expect("Invalid ca");
            }
            TlsType::CASigned => {
                config
                    .root_store
                    .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
            }
        }
        let config = Arc::new(config);
        let connector = TlsConnector::from(config);
        let tcp_stream = TcpStream::connect(address).await?;
        let domain = DNSNameRef::try_from_ascii_str(&mqtts_connect.domain).unwrap();

        let tls_stream = connector
            .connect(domain, tcp_stream)
            .await
            .expect("Invalid connector");
        Ok(Stream::Mqtts(tls_stream))
    }

    async fn new_ws(address: &SocketAddr, ws_connect: &WsConnect) -> Result<Stream, Error> {
        let ws_url = format!("ws://{}{}", address, &ws_connect.path);
        let tcp_stream = TcpStream::connect(address).await?;
        let (ws_stream, _) = tokio_tungstenite::client_async(ws_url, tcp_stream).await?;
        Ok(Stream::Ws(ws_stream))
    }

    async fn new_wss(address: &SocketAddr, wss_connect: &WssConnect) -> Result<Stream, Error> {
        let mut config = ClientConfig::new();
        match &wss_connect.tls_type {
            TlsType::SelfSigned(self_signed) => {
                let mut pem = BufReader::new(File::open(&self_signed.root_ca)?);
                config
                    .root_store
                    .add_pem_file(&mut pem)
                    .expect("Invalid ca");
            }
            TlsType::CASigned => {
                config
                    .root_store
                    .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
            }
        }
        let config = Arc::new(config);
        let connector = TlsConnector::from(config);
        let tcp_stream = TcpStream::connect(address).await?;
        let domain = DNSNameRef::try_from_ascii_str(&wss_connect.domain).unwrap();

        let tls_stream = connector
            .connect(domain, tcp_stream)
            .await
            .expect("Invalid connector");

        let ws_url = format!("ws://{}{}", address, &wss_connect.path);
        let (ws_stream, _) = tokio_tungstenite::client_async(ws_url, tls_stream).await?;
        Ok(Stream::Wss(ws_stream))
    }

    async fn new_uds(uds_connect: &UdsConnect) -> Result<Stream, Error> {
        let uds_stream = UnixStream::connect(&uds_connect.sock_path).await?;
        Ok(Stream::Uds(uds_stream))
    }

    pub async fn read_buf(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        match self {
            Stream::Mqtt(tcp_stream) => Ok(tcp_stream.read_buf(buf).await?),
            Stream::Mqtts(tls_stream) => Ok(tls_stream.read(buf).await?),
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
            Stream::Wss(ref mut wss_stream) => {
                if let Some(msg) = wss_stream.next().await {
                    let msg = msg?;
                    let data = msg.into_data();
                    let data_len = data.len();
                    buf.extend(data);
                    Ok(data_len)
                } else {
                    Ok(0)
                }
            }
            Stream::Uds(ref mut uds_stream) => Ok(uds_stream.read_buf(buf).await?),
        }
    }

    pub async fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        match self {
            Stream::Mqtt(tcp_stream) => Ok(tcp_stream.write(buf).await?),
            Stream::Mqtts(tls_socket) => Ok(tls_socket.write(buf).await?),
            Stream::Ws(ws_stream) => {
                let msg = Message::binary(buf);
                ws_stream.send(msg).await?;
                Ok(buf.len())
            }
            Stream::Wss(wss_stream) => {
                let msg = Message::binary(buf);
                wss_stream.send(msg).await?;
                Ok(buf.len())
            }
            Stream::Uds(uds_stream) => Ok(uds_stream.write(buf).await?),
        }
    }
}
