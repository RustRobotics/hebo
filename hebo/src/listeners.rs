// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use futures_util::{SinkExt, StreamExt};
use std::fs::File;
use std::io::BufReader;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::rustls::internal::pemfile;
use tokio_rustls::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;
use tokio_tungstenite::{self, tungstenite::protocol::Message, WebSocketStream};

use crate::config::{self, Protocol};
use crate::error::{Error, ErrorKind};

/// Each Listener binds to a specific port
pub enum Listener {
    Mqtt(TcpListener),
    Mqtts(TcpListener, TlsAcceptor),
    Ws(TcpListener),
    Wss(TcpListener, TlsAcceptor),
}

/// Each Stream represents a duplex socket connection to client.
#[derive(Debug)]
pub enum Stream {
    Mqtt(TcpStream),
    Mqtts(TlsStream<TcpStream>),
    Ws(WebSocketStream<TcpStream>),
    Wss(WebSocketStream<TlsStream<TcpStream>>),
}

impl Listener {
    fn load_certs(path: &String) -> Result<Vec<Certificate>, Error> {
        pemfile::certs(&mut BufReader::new(File::open(path)?)).map_err(|err| {
            Error::with_string(
                ErrorKind::CertError,
                format!("Failed to load cert file at {}, got: {:?}", &path, err),
            )
        })
    }

    fn load_keys(path: &String) -> Result<Vec<PrivateKey>, Error> {
        pemfile::rsa_private_keys(&mut BufReader::new(File::open(path)?)).map_err(|err| {
            Error::with_string(
                ErrorKind::CertError,
                format!("Failed to load key file at {}, got {:?}", &path, err),
            )
        })
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
                let cert_file = listener
                    .cert_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "cert_file is required"))?;
                let key_file = listener
                    .key_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "key_file is required"))?;

                let certs = Listener::load_certs(cert_file)?;
                let mut keys = Listener::load_keys(key_file)?;
                let mut config = ServerConfig::new(NoClientAuth::new());
                config
                    .set_single_cert(certs, keys.remove(0))
                    .map_err(|err| {
                        Error::with_string(
                            ErrorKind::CertError,
                            format!("Failed to init ServerConfig, got {:?}", err),
                        )
                    })?;

                let acceptor = TlsAcceptor::from(Arc::new(config));

                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::Mqtts(listener, acceptor));
                }
            }
            Protocol::Ws => {
                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::Ws(listener));
                }
            }
            Protocol::Wss => {
                let cert_file = listener
                    .cert_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "cert_file is required"))?;
                let key_file = listener
                    .key_file
                    .as_ref()
                    .ok_or(Error::new(ErrorKind::CertError, "key_file is required"))?;

                let certs = Listener::load_certs(cert_file)?;
                let mut keys = Listener::load_keys(key_file)?;
                let mut config = ServerConfig::new(NoClientAuth::new());
                config
                    .set_single_cert(certs, keys.remove(0))
                    .map_err(|err| {
                        Error::with_string(
                            ErrorKind::CertError,
                            format!("Failed to init ServerConfig, got {:?}", err),
                        )
                    })?;

                let acceptor = TlsAcceptor::from(Arc::new(config));

                let addrs = listener.address.to_socket_addrs()?;
                for addr in addrs {
                    let listener = TcpListener::bind(&addr).await?;
                    return Ok(Listener::Wss(listener, acceptor));
                }
            }
        }
        Err(Error::with_string(
            ErrorKind::SocketError,
            format!("Failed to create server socket with config: {:?}", listener),
        ))
    }

    pub async fn accept(&self) -> Result<Stream, Error> {
        match self {
            Listener::Mqtt(listener) => {
                let (tcp_stream, _address) = listener.accept().await?;
                return Ok(Stream::Mqtt(tcp_stream));
            }
            Listener::Mqtts(listener, acceptor) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let tls_stream = acceptor.accept(tcp_stream).await?;
                return Ok(Stream::Mqtts(tls_stream));
            }
            Listener::Ws(listener) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let ws_stream = tokio_tungstenite::accept_async(tcp_stream).await?;
                return Ok(Stream::Ws(ws_stream));
            }
            Listener::Wss(listener, acceptor) => {
                let (tcp_stream, _address) = listener.accept().await?;
                let tls_stream = acceptor.accept(tcp_stream).await?;
                let ws_stream = tokio_tungstenite::accept_async(tls_stream).await?;
                return Ok(Stream::Wss(ws_stream));
            }
        }
    }
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
        }
    }
}
