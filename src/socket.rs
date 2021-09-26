// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::net::UdpSocket;
use std::os::unix::io::{AsRawFd, RawFd};
use tokio::net::TcpListener;

use crate::error::{Error, ErrorKind};

pub async fn new_tcp_listener(address: &str, interface: &str) -> Result<TcpListener, Error> {
    let listener = TcpListener::bind(address).await?;
    let socket_fd: RawFd = listener.as_raw_fd();

    // Bind interface
    if !interface.is_empty() {
        nc::setsockopt(
            socket_fd,
            nc::SOL_SOCKET,
            nc::SO_BINDTODEVICE,
            interface.as_ptr() as usize,
            interface.len() as u32,
        )
        .map_err(|errno| {
            Error::from_string(
                ErrorKind::ConfigError,
                format!(
                    "Failed to bind interface: {}, err: {}",
                    interface,
                    nc::strerror(errno)
                ),
            )
        })?;
    }

    // TODO(Shaohua): Enable fast open

    Ok(listener)
}

pub fn new_udp_socket(address: &str, interface: &str) -> Result<UdpSocket, Error> {
    unimplemented!()
}
