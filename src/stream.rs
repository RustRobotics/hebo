// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use futures_util::{SinkExt, StreamExt};
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UnixStream};
use tokio_rustls::client::TlsStream;
use tokio_rustls::rustls;
use tokio_tungstenite::{self, tungstenite::protocol::Message, WebSocketStream};

use crate::connect_options::{
    ConnectType, MqttConnect, MqttsConnect, QuicConnect, TlsType, UdsConnect, WsConnect, WssConnect,
};
use crate::error::Error;

pub enum Stream {
    Mqtt(TcpStream),
    Mqtts(TlsStream<TcpStream>),
    Ws(WebSocketStream<TcpStream>),
    Wss(WebSocketStream<TlsStream<TcpStream>>),
    Uds(UnixStream),
    Quic(quinn::NewConnection),
    // TODO(Shaohua): Remove this value.
    None,
}

impl fmt::Debug for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mqtt(..) => f.write_str("Mqtt"),
            Self::Mqtts(..) => f.write_str("Mqtts"),
            Self::Ws(..) => f.write_str("Ws"),
            Self::Wss(..) => f.write_str("Wsx"),
            Self::Uds(..) => f.write_str("Uds"),
            Self::Quic(..) => f.write_str("Quic"),
            Self::None => f.write_str("None"),
        }
    }
}

impl Stream {
    pub async fn connect(connect_type: &ConnectType) -> Result<Self, Error> {
        match connect_type {
            ConnectType::Mqtt(mqtt_connect) => Self::new_mqtt(mqtt_connect).await,
            ConnectType::Mqtts(mqtts_connect) => Self::new_mqtts(mqtts_connect).await,
            ConnectType::Ws(ws_connect) => Self::new_ws(ws_connect).await,
            ConnectType::Wss(wss_connect) => Self::new_wss(wss_connect).await,
            ConnectType::Uds(uds_connect) => Self::new_uds(uds_connect).await,
            ConnectType::Quic(quic_connect) => Self::new_quic(quic_connect).await,
        }
    }

    async fn new_mqtt(mqtt_connect: &MqttConnect) -> Result<Self, Error> {
        let tcp_stream = TcpStream::connect(mqtt_connect.address).await?;
        Ok(Self::Mqtt(tcp_stream))
    }

    async fn new_tls_stream(
        tls_type: &TlsType,
        server_address: &SocketAddr,
        server_domain: &str,
    ) -> Result<TlsStream<TcpStream>, Error> {
        let mut root_store = rustls::RootCertStore::empty();
        match tls_type {
            TlsType::SelfSigned(self_signed) => {
                let mut pem_buf = BufReader::new(File::open(&self_signed.cert)?);
                let pem_data = rustls_pemfile::certs(&mut pem_buf).unwrap();
                root_store.add_parsable_certificates(&pem_data);
            }
            TlsType::CASigned => {
                root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
            }
        }
        let config_builder = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store);
        let client_config: rustls::ClientConfig = config_builder.with_no_client_auth();
        let rc_client_config = Arc::new(client_config);
        let connector = tokio_rustls::TlsConnector::from(rc_client_config);
        let tcp_stream = TcpStream::connect(server_address).await?;
        // TODO(Shaohua): Convert error type.
        let domain = rustls::ServerName::try_from(server_domain).unwrap();

        // TODO(Shaohua): Convert error type.
        let tls_stream = connector
            .connect(domain, tcp_stream)
            .await
            .expect("Invalid connector");

        return Ok(tls_stream);
    }

    async fn new_mqtts(mqtts_connect: &MqttsConnect) -> Result<Self, Error> {
        let tls_stream = Self::new_tls_stream(
            &mqtts_connect.tls_type,
            &mqtts_connect.address,
            &mqtts_connect.domain,
        )
        .await?;
        Ok(Self::Mqtts(tls_stream))
    }

    async fn new_ws(ws_connect: &WsConnect) -> Result<Stream, Error> {
        let ws_url = format!("ws://{}{}", ws_connect.address, &ws_connect.path);
        let tcp_stream = TcpStream::connect(ws_connect.address).await?;
        let (ws_stream, _) = tokio_tungstenite::client_async(ws_url, tcp_stream).await?;
        Ok(Stream::Ws(ws_stream))
    }

    async fn new_wss(wss_connect: &WssConnect) -> Result<Stream, Error> {
        let tls_stream = Self::new_tls_stream(
            &wss_connect.tls_type,
            &wss_connect.address,
            &wss_connect.domain,
        )
        .await?;
        let ws_url = format!("ws://{}{}", wss_connect.address, &wss_connect.path);
        let (ws_stream, _) = tokio_tungstenite::client_async(ws_url, tls_stream).await?;
        Ok(Stream::Wss(ws_stream))
    }

    async fn new_uds(uds_connect: &UdsConnect) -> Result<Stream, Error> {
        let uds_stream = UnixStream::connect(&uds_connect.sock_path).await?;
        Ok(Stream::Uds(uds_stream))
    }

    async fn new_quic(quic_connect: &QuicConnect) -> Result<Stream, Error> {
        // TODO(Shaohua): set client config.
        //        let mut client_config = quinn::ClientConfigBuilder::default();
        //        match &quic_connect.tls_type {
        //            TlsType::SelfSigned(self_signed) => {
        //                let cert_content = fs::read(&self_signed.cert)?;
        //                let cert_chain = if self_signed.cert.extension().map_or(false, |x| x == "der") {
        //                    quinn::Certificate::from_der(&cert_content)?
        //                } else {
        //                    quinn::Certificate::from_pem(&cert_content)?
        //                };
        //                client_config.add_certificate_authority(cert_chain)?;
        //            }
        //            TlsType::CASigned => {
        //                // Use default ca roots
        //            }
        //        }

        let endpoint = quinn::Endpoint::client(quic_connect.client_address)?;
        //endpoint.set_default_client_config(client_config.build());
        let quic_connection = endpoint
            .connect(quic_connect.server_address, &quic_connect.domain)?
            .await?;
        Ok(Stream::Quic(quic_connection))
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
            Stream::Quic(ref mut quic_connection) => {
                if let Some(Ok(mut recv)) = quic_connection.uni_streams.next().await {
                    Ok(recv.read_buf(buf).await?)
                } else {
                    Ok(0)
                }
            }
            Stream::None => unreachable!(),
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
            Stream::Quic(quic_connection) => {
                let mut send = quic_connection.connection.open_uni().await?;
                send.write_all(buf).await?;
                send.finish().await?;
                Ok(buf.len())
            }
            Stream::None => unreachable!(),
        }
    }
}
