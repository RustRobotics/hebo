// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::net::{SocketAddr, ToSocketAddrs};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;

use crate::config::{self, Protocol};
use crate::error::Error;

pub enum Listener {
    Mqtt(TcpListener),
    Ws(TcpListener),
}

#[derive(Debug)]
pub enum Stream {
    Mqtt(TcpStream),
    Ws(WebSocketStream<TcpStream>),
}

pub async fn bind_address(listener: &config::Listener) -> Result<Listener, Error> {
    match listener.protocol {
        Protocol::Mqtt => {
            let addrs = listener.address.to_socket_addrs()?;
            for addr in addrs {
                let listener = TcpListener::bind(&addr).await?;
                return Ok(Listener::Mqtt(listener));
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
