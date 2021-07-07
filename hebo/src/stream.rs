// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UnixStream};
use tokio_rustls::server::TlsStream;
use tokio_tungstenite::{self, tungstenite::protocol::Message, WebSocketStream};

use crate::error::Error;

/// Each Stream represents a duplex socket connection to client.
#[derive(Debug)]
pub enum Stream {
    Mqtt(TcpStream),
    Mqtts(TlsStream<TcpStream>),
    Ws(WebSocketStream<TcpStream>),
    Wss(WebSocketStream<TlsStream<TcpStream>>),
    Uds(UnixStream),
}

impl Stream {
    // TODO(Shaohua): Replace with bytes::BufMute
    pub async fn read_buf(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        match self {
            Stream::Mqtt(ref mut tcp_stream) => Ok(tcp_stream.read_buf(buf).await?),
            Stream::Mqtts(ref mut tls_stream) => Ok(tls_stream.read_buf(buf).await?),
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
            Stream::Mqtts(tls_stream) => Ok(tls_stream.write(buf).await?),
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
