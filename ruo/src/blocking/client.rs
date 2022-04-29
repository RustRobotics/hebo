// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::ProtocolLevel;
use codec::QoS;
use std::fmt;

use super::{ClientInnerV3, ClientInnerV4, ClientInnerV5, ClientStatus, PublishMessage};
use crate::connect_options::ConnectOptions;
use crate::error::Error;

/// Synchronize mqtt client.
pub struct Client {
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
    /// No packet is sent to server before calling [`Self::connect()`].
    pub fn new(connect_options: ConnectOptions) -> Self {
        let inner = match connect_options.protocol_level() {
            ProtocolLevel::V3 => Inner::V3(ClientInnerV3::new(connect_options)),
            ProtocolLevel::V4 => Inner::V4(ClientInnerV4::new(connect_options)),
            ProtocolLevel::V5 => Inner::V5(ClientInnerV5::new(connect_options)),
        };
        Self { inner }
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
    pub fn connect(&mut self) -> Result<bool, Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.connect(),
            Inner::V4(inner) => inner.connect(),
            Inner::V5(inner) => inner.connect(),
        }
    }

    /// Publish packet.
    pub fn publish(&mut self, topic: &str, qos: QoS, data: &[u8]) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.publish(topic, qos, data),
            Inner::V4(inner) => inner.publish(topic, qos, data),
            Inner::V5(inner) => inner.publish(topic, qos, data),
        }
    }

    /// Subscribe to topic.
    pub fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.subscribe(topic, qos),
            Inner::V4(inner) => inner.subscribe(topic, qos),
            Inner::V5(inner) => inner.subscribe(topic, qos),
        }
    }

    /// Unsubscribe specific topic or topic pattern.
    pub fn unsubscribe(&mut self, topic: &str) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.unsubscribe(topic),
            Inner::V4(inner) => inner.unsubscribe(topic),
            Inner::V5(inner) => inner.unsubscribe(topic),
        }
    }

    /// Send ping packet to server explicitly.
    pub fn ping(&mut self) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.ping(),
            Inner::V4(inner) => inner.ping(),
            Inner::V5(inner) => inner.ping(),
        }
    }

    pub fn wait_for_message(&mut self) -> Result<Option<PublishMessage>, Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.wait_for_packet(),
            Inner::V4(inner) => inner.wait_for_packet(),
            Inner::V5(inner) => inner.wait_for_packet(),
        }
    }

    /// Disconnect from server.
    pub fn disconnect(&mut self) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.disconnect(),
            Inner::V4(inner) => inner.disconnect(),
            Inner::V5(inner) => inner.disconnect(),
        }
    }
}
