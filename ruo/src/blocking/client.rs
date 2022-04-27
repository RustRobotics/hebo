// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::ProtocolLevel;
use codec::QoS;
use std::fmt;

use crate::connect_options::ConnectOptions;
use crate::error::Error;

/// Mqtt connection status.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClientStatus {
    Initialized,
    Connecting,
    Connected,
    ConnectFailed,
    Disconnecting,
    Disconnected,
}

/// Synchronize mqtt client.
pub struct Client {
    protocol_level: ProtocolLevel,
    connect_options: ConnectOptions,
    status: ClientStatus,
    inner: Inner,
}

enum Inner {
    V3(ClientInnerV3),
    V4(ClientInnerV4),
    V5(ClientInnerV5),
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("connect_options", &self.connect_options)
            .field("status", &self.status)
            .finish()
    }
}

impl Client {
    /// Create a new mqtt client.
    ///
    /// No packet is sent to server before calling [`connect()`].
    pub fn new(connect_options: ConnectOptions, protocol_level: ProtocolLevel) -> Self {
        let inner = match protocol_level {
            ProtocolLevel::V31 => Inner::V3(ClientInnerV3::new(connect_options.clone())),
            ProtocolLevel::V311 => Inner::V4(ClientInnerV4::new(connect_options.clone())),
            ProtocolLevel::V5 => Inner::V5(ClientInnerV5::new(connect_options.clone())),
        };
        Self {
            protocol_level,
            connect_options,
            status: ClientStatus::Initialized,
            inner,
        }
    }

    /// Get mqtt connection options.
    pub fn connect_option(&self) -> &ConnectOptions {
        &self.connect_options
    }

    /// Get current status.
    pub fn status(&self) -> ClientStatus {
        self.status
    }

    /// Connect to server.
    pub fn connect(&mut self) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Disconnected);
        self.status = ClientStatus::Connecting;
        //TODO(Shaohua):
        Ok(())
    }

    pub fn publish(&mut self, topic: &str, qos: QoS, data: &[u8]) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        //TODO(Shaohua):
        Ok(())
    }

    pub fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        //TODO(Shaohua):
        Ok(())
    }

    pub fn unsubscribe(&mut self, topic: &str) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        //TODO(Shaohua):
        Ok(())
    }

    pub fn wait_for_messages(&mut self) -> Result<Vec<u8>, Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        todo!()
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        assert_eq!(self.status, ClientStatus::Connected);
        self.status = ClientStatus::Disconnecting;
        //TODO(Shaohua):
        self.status = ClientStatus::Disconnected;
        Ok(())
    }
}
