// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::time::Duration;
use std::net::{ToSocketAddrs, SocketAddr};
use std::io;

#[derive(Clone, Debug)]
pub struct HttpProxy {
    pub hostname: String,
    pub port: u16,
    pub login: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct Socks5Proxy {
    pub hostname: String,
    pub port: u16,
    pub login: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub enum Proxy {
    None,
    Http(HttpProxy),
    Socks5(Socks5Proxy),
}

pub trait Authentication {
}

#[derive(Clone, Debug)]
pub struct UsernameAuth {
    pub username: String,
    pub password: String,
}

impl Authentication for UsernameAuth {
}

#[derive(Clone, Debug)]
pub struct ConnectOptions {
    address: SocketAddr,
    client_id: String,
    keep_alive: Duration,
    proxy: Proxy,
}

impl Default for ConnectOptions {
    fn default() -> Self {
        ConnectOptions {
            address: SocketAddr::from(([127, 0, 0, 1], 1883)),
            client_id: String::new(),
            keep_alive: Duration::from_secs(30),
            proxy: Proxy::None,
        }
    }
}

impl ConnectOptions {
    pub fn new<A: ToSocketAddrs>(address: A) -> io::Result<ConnectOptions> {
        let mut addrs = address.to_socket_addrs()?;
        Ok(ConnectOptions {
            address: addrs.nth(0).unwrap(),
            ..Self::default()
        })
    }

    pub fn address(&self) -> &SocketAddr {
        &self.address
    }

    pub fn set_client_id(&mut self, client_id: &str) -> &mut Self {
        self.client_id = client_id.to_string();
        self
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn set_keepalive(&mut self, keep_alive: Duration) -> &mut Self {
        self.keep_alive = keep_alive;
        self
    }

    pub fn keep_alive(&self) -> Duration {
        self.keep_alive
    }

    pub fn set_proxy(&mut self, proxy: Proxy) -> &mut Self {
        self.proxy = proxy;
        self
    }

    pub fn proxy(&self) -> &Proxy {
        &self.proxy
    }

    pub fn set_auth(&mut self, ) -> &mut Self {
        self
    }

    pub fn auth(&self) {
    }
}
