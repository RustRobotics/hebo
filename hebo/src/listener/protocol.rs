// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::fmt;
use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::net::UnixListener;
use tokio_rustls::TlsAcceptor;

/// Each Listener binds to a specific port
pub enum Protocol {
    Mqtt(TcpListener),
    Mqtts(TcpListener, TlsAcceptor),
    Ws(TcpListener),
    Wss(TcpListener, TlsAcceptor),
    #[cfg(unix)]
    Uds(UnixListener),
    Quic(quinn::Endpoint),
}

impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Mqtt(..) => "Mqtt",
            Self::Mqtts(..) => "Mqtts",
            Self::Ws(..) => "Ws",
            Self::Wss(..) => "Wss",
            #[cfg(unix)]
            Self::Uds(..) => "Uds",
            Self::Quic(..) => "Quic",
        };
        write!(f, "{msg}")
    }
}
