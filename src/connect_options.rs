// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::time::Duration;

#[derive(Debug)]
pub struct HttpProxy {
}

#[derive(Debug)]
pub struct Socks5Proxy {
}

#[derive(Debug)]
pub enum Proxy {
    None,
    Http(HttpProxy),
    Socks5(Socks5Proxy),
}

pub trait Authentication {
}

#[derive(Debug)]
pub struct UsernameAuth {
    pub username: String,
    pub password: String,
}

impl Authentication for UsernameAuth {
}

pub struct ConnectOptions {
    address: String,
    port: u16,
    keep_alive: Duration,
    proxy: Proxy,
    auth: Option<Box<dyn Authentication>>,
}

impl Default for ConnectOptions {
    fn default() -> Self {
        ConnectOptions {
            address: "127.0.0.1".to_string(),
            port: 1883,
            keep_alive: Duration::from_secs(30),
            proxy: Proxy::None,
            auth: None,
        }
    }
}

impl ConnectOptions {
    pub fn new(addr: &str, port: u16) -> ConnectOptions {
        ConnectOptions {
            address: addr.to_string(),
            port: port,
            ..Self::default()
        }
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
