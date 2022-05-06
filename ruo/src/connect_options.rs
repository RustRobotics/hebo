// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::utils::random_string;
use codec::ProtocolLevel;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

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

#[derive(Clone, Debug)]
pub struct UsernameAuth {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct SelfSignedTls {
    pub cert: PathBuf,
}

#[derive(Clone, Debug)]
pub enum TlsType {
    /// Signed by Root CA, like `Let's Encrypt`.
    CASigned,

    /// Generated self signed ca file with `openssl` or other tools.
    SelfSigned(SelfSignedTls),
}

/// Connect to tcp server.
#[derive(Clone, Debug)]
pub struct MqttConnect {
    pub address: SocketAddr,
}

/// Connect to secure tcp server.
#[derive(Clone, Debug)]
pub struct MqttsConnect {
    pub address: SocketAddr,
    pub domain: String,
    pub tls_type: TlsType,
}

/// Connect to websocket server.
#[derive(Clone, Debug)]
pub struct WsConnect {
    pub address: SocketAddr,
    pub path: String,
}

/// Connect to secure websocket server.
#[derive(Clone, Debug)]
pub struct WssConnect {
    pub address: SocketAddr,
    pub domain: String,
    pub tls_type: TlsType,
    pub path: String,
}

/// Connect to unix domain socket server.
#[derive(Clone, Debug)]
pub struct UdsConnect {
    pub sock_path: PathBuf,
}

/// Connect to quic based server.
#[derive(Clone, Debug)]
pub struct QuicConnect {
    /// Specify client ip and port of quic protocol. If port number is 0, kernel
    /// will choose a random port automatically.
    pub client_address: SocketAddr,

    pub server_address: SocketAddr,
    pub domain: String,
    pub tls_type: TlsType,
}

#[derive(Clone, Debug)]
pub enum ConnectType {
    Mqtt(MqttConnect),
    Mqtts(MqttsConnect),
    Ws(WsConnect),
    Wss(WssConnect),
    Uds(UdsConnect),
    Quic(QuicConnect),
}

/// Options for mqtt connection.
#[derive(Clone, Debug)]
pub struct ConnectOptions {
    /// MQTT protocol version.
    ///
    /// Default is MQTT 3.1.1.
    protocol_level: ProtocolLevel,

    /// Specify connection protocol.
    ///
    /// Supported protocols are:
    /// - TCP
    /// - TCP over TLS
    /// - WebSocket
    /// - WebSocket over TLS
    /// - Unix domain socket
    /// - QUIC
    ///
    /// Default is raw TCP.
    connect_type: ConnectType,

    /// Speicify client-id to used to connect to server.
    ///
    /// The server will reject connection with same client-id.
    ///
    /// Default value is randomly generated, and length is 8 chracters.
    client_id: String,

    /// Specify keep alive duration of network connection.
    ///
    /// Default is 60 seconds.
    keep_alive: Duration,

    /// Specify network connection timeout.
    ///
    /// Default is 10 seconds.
    connect_timeout: Duration,

    /// Speicfy network proxy.
    ///
    /// Default is None.
    proxy: Proxy,
}

impl Default for ConnectOptions {
    fn default() -> Self {
        let client_id = "ruo".to_owned() + &random_string(8);
        ConnectOptions {
            protocol_level: ProtocolLevel::V4,
            connect_type: ConnectType::Mqtt(MqttConnect {
                address: SocketAddr::from(([127, 0, 0, 1], 1883)),
            }),
            client_id,
            connect_timeout: Duration::from_secs(10),
            keep_alive: Duration::from_secs(60),
            proxy: Proxy::None,
        }
    }
}

impl ConnectOptions {
    /// Create a ConnectionObject object with default values.
    pub fn new() -> ConnectOptions {
        Self::default()
    }

    /// Update mqtt protocol level.
    pub fn set_protocol_level(&mut self, protocol_level: ProtocolLevel) -> &mut Self {
        self.protocol_level = protocol_level;
        self
    }

    /// Get current mqtt protocol level.
    pub fn protocol_level(&self) -> ProtocolLevel {
        self.protocol_level
    }

    /// Update connection type.
    pub fn set_connect_type(&mut self, connect_type: ConnectType) -> &mut Self {
        self.connect_type = connect_type;
        self
    }

    /// Get current connection type.
    pub fn connect_type(&self) -> &ConnectType {
        &self.connect_type
    }

    /// Update client id.
    pub fn set_client_id(&mut self, client_id: &str) -> &mut Self {
        self.client_id = client_id.to_string();
        self
    }

    /// Get current client id.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Update connection timeout duration.
    pub fn set_connect_timeout(&mut self, connect_timeout: Duration) -> &mut Self {
        self.connect_timeout = connect_timeout;
        self
    }

    /// Get current connection timeout duration.
    pub fn connect_timeout(&self) -> &Duration {
        &self.connect_timeout
    }

    /// Update keep alive value of network connection.
    pub fn set_keepalive(&mut self, keep_alive: Duration) -> &mut Self {
        self.keep_alive = keep_alive;
        self
    }

    /// Get current value of network keep alive.
    pub fn keep_alive(&self) -> &Duration {
        &self.keep_alive
    }

    /// Update network proxy settings.
    pub fn set_proxy(&mut self, proxy: Proxy) -> &mut Self {
        self.proxy = proxy;
        self
    }

    /// Get current proxy value.
    pub fn proxy(&self) -> &Proxy {
        &self.proxy
    }

    // TODO(Shaohua): Add authentication options
}
