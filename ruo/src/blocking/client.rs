// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::ProtocolLevel;
use codec::QoS;
use std::fmt;

use super::{ClientInnerV3, ClientInnerV4, ClientInnerV5, ClientStatus};
use crate::connect_options::ConnectOptions;
use crate::error::Error;

/// Synchronize mqtt client.
pub struct Client {
    protocol_level: ProtocolLevel,
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
            .field("connect_options", self.connect_options())
            .field("status", &self.status())
            .finish()
    }
}

impl Client {
    /// Create a new mqtt client.
    ///
    /// No packet is sent to server before calling [`connect()`].
    pub fn new(
        connect_options: ConnectOptions,
        protocol_level: ProtocolLevel,
    ) -> Result<Self, Error> {
        let inner = match protocol_level {
            ProtocolLevel::V31 => {
                let inner = ClientInnerV3::new(connect_options)?;
                Inner::V3(inner)
            }
            ProtocolLevel::V311 => {
                let inner = ClientInnerV4::new(connect_options)?;
                Inner::V4(inner)
            }
            ProtocolLevel::V5 => {
                let inner = ClientInnerV5::new(connect_options)?;
                Inner::V5(inner)
            }
        };
        Ok(Self {
            protocol_level,
            inner,
        })
    }

    /// Get mqtt connection options.
    pub fn connect_options(&self) -> &ConnectOptions {
        match &self.inner {
            Inner::V3(inner) => inner.connect_options(),
            Inner::V4(inner) => inner.connect_options(),
            Inner::V5(inner) => inner.connect_options(),
        }
    }

    /// Get current status.
    pub fn status(&self) -> ClientStatus {
        match &self.inner {
            Inner::V3(inner) => inner.status(),
            Inner::V4(inner) => inner.status(),
            Inner::V5(inner) => inner.status(),
        }
    }

    /// Connect to server.
    pub fn connect(&mut self) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.connect(),
            Inner::V4(inner) => inner.connect(),
            Inner::V5(inner) => inner.connect(),
        }
    }

    pub fn publish(&mut self, topic: &str, qos: QoS, data: &[u8]) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.publish(topic, qos, data),
            Inner::V4(inner) => inner.publish(topic, qos, data),
            Inner::V5(inner) => inner.publish(topic, qos, data),
        }
    }

    pub fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.subscribe(topic, qos),
            Inner::V4(inner) => inner.subscribe(topic, qos),
            Inner::V5(inner) => inner.subscribe(topic, qos),
        }
    }

    pub fn unsubscribe(&mut self, topic: &str) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.unsubscribe(topic),
            Inner::V4(inner) => inner.unsubscribe(topic),
            Inner::V5(inner) => inner.unsubscribe(topic),
        }
    }

    pub fn wait_for_messages(&mut self) -> Result<Vec<u8>, Error> {
        todo!()
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.disconnect(),
            Inner::V4(inner) => inner.disconnect(),
            Inner::V5(inner) => inner.disconnect(),
        }
    }
}
