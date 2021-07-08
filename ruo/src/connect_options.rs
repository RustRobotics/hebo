// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

use codec::utils::random_string;

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

pub trait Authentication {}

#[derive(Clone, Debug)]
pub struct UsernameAuth {
    pub username: String,
    pub password: String,
}

impl Authentication for UsernameAuth {}

#[derive(Clone, Debug)]
pub struct SelfSignedTls {
    pub root_ca: String,
    pub cert: String,
}

#[derive(Clone, Debug)]
pub enum TlsType {
    /// Signed by Root CA, like `Let's Encrypt`.
    CASigned,

    /// Generated self signed ca file with `openssl` or other tools.
    SelfSigned(SelfSignedTls),
}

#[derive(Clone, Debug)]
pub struct MqttConnect {}

#[derive(Clone, Debug)]
pub struct MqttsConnect {
    pub domain: String,
    pub tls_type: TlsType,
}

#[derive(Clone, Debug)]
pub struct WsConnect {
    pub path: String,
}

#[derive(Clone, Debug)]
pub struct WssConnect {
    pub tls_type: TlsType,
    pub path: String,
}

#[derive(Clone, Debug)]
pub enum ConnectType {
    Mqtt(MqttConnect),
    Mqtts(MqttsConnect),
    Ws(WsConnect),
    Wss(WssConnect),
}

#[derive(Clone, Debug)]
pub struct ConnectOptions {
    address: SocketAddr,
    connect_type: ConnectType,
    client_id: String,
    keep_alive: Duration,
    connect_timeout: Duration,
    proxy: Proxy,
}

impl Default for ConnectOptions {
    fn default() -> Self {
        ConnectOptions {
            address: SocketAddr::from(([127, 0, 0, 1], 1883)),
            connect_type: ConnectType::Mqtt(MqttConnect {}),
            client_id: random_string(8),
            connect_timeout: Duration::from_secs(10),
            keep_alive: Duration::from_secs(30),
            proxy: Proxy::None,
        }
    }
}

impl ConnectOptions {
    pub fn new<A: ToSocketAddrs>(address: A) -> io::Result<ConnectOptions> {
        let mut addrs = address.to_socket_addrs()?;
        Ok(ConnectOptions {
            address: addrs.next().unwrap(),
            ..Self::default()
        })
    }

    pub fn set_address<A: ToSocketAddrs>(&mut self, address: A) -> io::Result<&mut Self> {
        let mut address = address.to_socket_addrs()?;
        self.address = address.next().unwrap();
        Ok(self)
    }

    pub fn address(&self) -> &SocketAddr {
        &self.address
    }

    pub fn set_connect_type(&mut self, connect_type: ConnectType) -> &mut Self {
        self.connect_type = connect_type;
        self
    }

    pub fn connect_type(&self) -> &ConnectType {
        &self.connect_type
    }

    pub fn set_client_id(&mut self, client_id: &str) -> &mut Self {
        self.client_id = client_id.to_string();
        self
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn set_connect_timeout(&mut self, connect_timeout: Duration) -> &mut Self {
        self.connect_timeout = connect_timeout;
        self
    }

    pub fn connect_timeout(&self) -> &Duration {
        &self.connect_timeout
    }

    pub fn set_keepalive(&mut self, keep_alive: Duration) -> &mut Self {
        self.keep_alive = keep_alive;
        self
    }

    pub fn keep_alive(&self) -> &Duration {
        &self.keep_alive
    }

    pub fn set_proxy(&mut self, proxy: Proxy) -> &mut Self {
        self.proxy = proxy;
        self
    }

    pub fn proxy(&self) -> &Proxy {
        &self.proxy
    }

    pub fn set_auth(&mut self) -> &mut Self {
        self
    }

    pub fn auth(&self) {}
}
