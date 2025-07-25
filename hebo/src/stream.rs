// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
#[cfg(unix)]
use tokio::net::UnixStream;
use tokio_rustls::server::TlsStream;
use tokio_tungstenite::{self, tungstenite::protocol::Message, WebSocketStream};

use crate::error::Error;

/// Each Stream represents a duplex socket connection to client.
#[derive(Debug)]
pub enum Stream {
    Mqtt(TcpStream),
    Mqtts(Box<TlsStream<TcpStream>>),
    Ws(Box<WebSocketStream<TcpStream>>),
    Wss(Box<WebSocketStream<TlsStream<TcpStream>>>),
    #[cfg(unix)]
    Uds(UnixStream),
    Quic(quinn::Connection),
}

impl Stream {
    /// Read from stream.
    ///
    /// # Errors
    ///
    /// Returns error if stream/socket gets error.
    pub async fn read_buf(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        // TODO(Shaohua): Replace with bytes::BufMute
        match self {
            Self::Mqtt(tcp_stream) => Ok(tcp_stream.read_buf(buf).await?),
            Self::Mqtts(tls_stream) => Ok(tls_stream.read_buf(buf).await?),
            Self::Ws(ws_stream) => {
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
            Self::Wss(wss_stream) => {
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
            #[cfg(unix)]
            Self::Uds(uds_stream) => Ok(uds_stream.read_buf(buf).await?),
            Self::Quic(quic_connection) => {
                if let Ok(mut recv) = quic_connection.accept_uni().await {
                    Ok(recv.read_buf(buf).await?)
                } else {
                    Ok(0)
                }
            }
        }
    }

    /// Write buffer to stream.
    ///
    /// # Errors
    ///
    /// Returns error if socket/stream gets error.
    pub async fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        // TODO(Shaohua): Replace type of buf with bytes::Bytes
        match self {
            Self::Mqtt(tcp_stream) => Ok(tcp_stream.write(buf).await?),
            Self::Mqtts(tls_stream) => Ok(tls_stream.write(buf).await?),
            Self::Ws(ws_stream) => {
                let msg = Message::binary(buf.to_vec());
                ws_stream.send(msg).await?;
                Ok(buf.len())
            }
            Self::Wss(wss_stream) => {
                let msg = Message::binary(buf.to_vec());
                wss_stream.send(msg).await?;
                Ok(buf.len())
            }
            #[cfg(unix)]
            Self::Uds(uds_stream) => Ok(uds_stream.write(buf).await?),
            Self::Quic(quic_connection) => {
                let mut send = quic_connection.open_uni().await?;
                send.write_all(buf).await?;
                send.finish()?;
                Ok(buf.len())
            }
        }
    }
}
