// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use futures_util::{SinkExt, StreamExt};
use std::fs::File;
use std::io::BufReader;
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::rustls::internal::pemfile;
use tokio_rustls::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;
use tokio_tungstenite::{self, tungstenite::protocol::Message, WebSocketStream};

use crate::config::{self, Protocol};
use crate::error::Error;

pub enum Listener {
    Mqtt(TcpListener),
    Mqtts(TcpListener),
    Ws(TcpListener),
}

#[derive(Debug)]
pub enum Stream {
    Mqtt(TcpStream),
    Mqtts(TcpStream),
    Ws(WebSocketStream<TcpStream>),
}

impl Listener {
    fn load_certs(path: &String) -> Result<Vec<Certificate>, Error> {
        pemfile::certs(&mut BufReader::new(File::open(path)?)).map_err(|_| Error::CertError)
    }

    fn load_keys(path: &String) -> Result<Vec<PrivateKey>, Error> {
        pemfile::rsa_private_keys(&mut BufReader::new(File::open(path)?))
            .map_err(|_| Error::KeyError)
    }

    pub async fn bind(listener: &config::Listener) -> Result<Listener, Error> {
        match listener.protocol {
            Protocol::Mqtt => {
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::Mqtt(listener));
                }
            }
            Protocol::Mqtts => {
                let cert_file = listener.cert_file.as_ref().ok_or(Error::CertError)?;
                let certs = Listener::load_certs(cert_file)?;
                let key_file = listener.key_file.as_ref().ok_or(Error::CertError)?;
                let mut keys = Listener::load_keys(key_file)?;
                let mut config = ServerConfig::new(NoClientAuth::new());
                config
                    .set_single_cert(certs, keys.remove(0))
                    .map_err(|_| Error::CertError)?;

                let acceptor = TlsAcceptor::from(Arc::new(config));

                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::Mqtts(listener));
                }
            }
            Protocol::Ws => {
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::Ws(listener));
                }
            }
            _ => {
                // TODO(Shaohua): Support more protocols
                return Err(Error::SocketError);
            }
        }
        Err(Error::SocketError)
    }

    pub async fn accept(&self) -> Result<Stream, Error> {
        match self {
            Listener::Mqtt(listener) => {
                let (tcp_stream, _address) = listener.accept().await?;
                return Ok(Stream::Mqtt(tcp_stream));
            }
            Listener::Mqtts(listener) => {
                let (tcp_stream, _address) = listener.accept().await?;
                return Ok(Stream::Mqtts(tcp_stream));
            }
            Listener::Ws(listener) => {
                // TODO(Shaohua): Convert error type
                let (tcp_stream, _address) = listener.accept().await?;
                let ws_stream = tokio_tungstenite::accept_async(tcp_stream).await.unwrap();
                return Ok(Stream::Ws(ws_stream));
            }
        }
    }
}

impl Stream {
    // TODO(Shaohua): Replace with bytes::BufMute
    pub async fn read_buf(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        match self {
            Stream::Mqtt(ref mut tcp_stream) => {
                // TODO(Shaohua): Convert error types
                Ok(tcp_stream.read_buf(buf).await?)
            }
            Stream::Mqtts(ref mut tls_stream) => {
                // TODO(Shaohua): Convert error types
                Ok(tls_stream.read_buf(buf).await?)
            }
            Stream::Ws(ref mut ws_stream) => {
                // TODO(Shaohua): Handle errors
                if let Some(msg) = ws_stream.next().await {
                    let msg = msg.unwrap();
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
            Stream::Mqtt(tcp_stream) => {
                // TODO(Shaohua): Handle errors
                Ok(tcp_stream.write(buf).await?)
            }
            Stream::Mqtts(tls_stream) => {
                // TODO(Shaohua): Handle errors
                Ok(tls_stream.write(buf).await?)
            }
            Stream::Ws(ws_stream) => {
                // TODO(Shaohua): Handle errors
                let msg = Message::binary(buf);
                ws_stream.send(msg).await.unwrap();
                Ok(buf.len())
            }
        }
    }
}
