// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use std::ffi::c_void;
use std::net::UdpSocket;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
use tokio::net::TcpListener;

use crate::error::{Error, ErrorKind};

#[cfg(unix)]
fn bind_device(socket_fd: RawFd, device: &str) -> Result<(), Error> {
    if !device.is_empty() {
        unsafe {
            #[allow(clippy::cast_possible_truncation)]
            let socket_len = device.len() as nc::socklen_t;
            nc::setsockopt(
                socket_fd,
                nc::SOL_SOCKET,
                nc::SO_BINDTODEVICE,
                device.as_ptr().cast::<c_void>(),
                socket_len,
            )
            .map_err(|errno| {
                Error::from_string(
                    ErrorKind::KernelError,
                    format!(
                        "Failed to bind device: {}, err: {}",
                        device,
                        nc::strerror(errno)
                    ),
                )
            })?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn enable_fast_open(socket_fd: RawFd) -> Result<(), Error> {
    // For Linux, value is the queue length of pending packets.
    //
    // TODO(Shaohua): Add a config option
    #[cfg(unix)]
    let queue_len: i32 = 5;
    // For the others, just a boolean value for enable and disable.
    #[cfg(not(unix))]
    let queue_len: i32 = 1;
    let queue_len_ptr = std::ptr::addr_of!(queue_len).cast::<c_void>();

    unsafe {
        #[allow(clippy::cast_possible_truncation)]
        let len = std::mem::size_of_val(&queue_len) as u32;
        nc::setsockopt(
            socket_fd,
            nc::IPPROTO_TCP,
            nc::TCP_FASTOPEN,
            queue_len_ptr,
            len,
        )
        .map_err(|errno| {
            Error::from_string(
                ErrorKind::KernelError,
                format!(
                    "Failed to enable socket fast open, got err: {}",
                    nc::strerror(errno)
                ),
            )
        })
    }
}

/// Create a new tcp server socket at `address` and binds to `device`.
///
/// # Errors
///
/// Returns error if socket `address` is invalid or failed to bind to specific `device`.
#[cfg(unix)]
pub async fn new_tcp_listener(address: &str, device: &str) -> Result<TcpListener, Error> {
    let listener = TcpListener::bind(address).await?;
    let socket_fd: RawFd = listener.as_raw_fd();

    bind_device(socket_fd, device)?;
    enable_fast_open(socket_fd)?;

    // TODO(Shaohua): Tuning tcp keep alive flag.
    // TODO(Shaohua): Tuning cpu affinity flag.

    Ok(listener)
}

#[cfg(not(unix))]
pub async fn new_tcp_listener(address: &str, _device: &str) -> Result<TcpListener, Error> {
    let listener = TcpListener::bind(address).await?;
    Ok(listener)
}

/// Create a new udp socket at `address` and binds to `device`.
///
/// # Errors
///
/// Returns error if socket `address` is invalid or failed to bind to specific `device`.
#[cfg(unix)]
pub fn new_udp_socket(address: &str, device: &str) -> Result<UdpSocket, Error> {
    let socket = UdpSocket::bind(address)?;
    let socket_fd: RawFd = socket.as_raw_fd();

    bind_device(socket_fd, device)?;

    Ok(socket)
}

/// Create a new udp socket at `address` and binds to `device`.
///
/// # Errors
///
/// Returns error if socket `address` is invalid or failed to bind to specific `device`.
#[cfg(not(unix))]
pub fn new_udp_socket(address: &str, _device: &str) -> Result<UdpSocket, Error> {
    let socket = UdpSocket::bind(address)?;
    Ok(socket)
}
