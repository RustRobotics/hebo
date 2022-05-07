// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use codec::{ProtocolLevel, QoS};
use std::fmt;
use std::future::Future;

use crate::connect_options::*;
use crate::error::Error;
use crate::{ClientInnerV3, ClientInnerV4, ClientInnerV5, ClientStatus};

type FutureConnectCb = dyn Fn(&mut Client) -> dyn Future<Output = ()>;

/// Asynchronous mqtt client.
pub struct Client {
    inner: Inner,
    connect_cb: Option<Box<FutureConnectCb>>,
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
    pub fn new(connect_options: ConnectOptions) -> Client {
        let inner = match connect_options.protocol_level() {
            ProtocolLevel::V3 => Inner::V3(ClientInnerV3::new(connect_options)),
            ProtocolLevel::V4 => Inner::V4(ClientInnerV4::new(connect_options)),
            ProtocolLevel::V5 => Inner::V5(ClientInnerV5::new(connect_options)),
        };
        Client {
            inner,
            connect_cb: None,
        }
    }

    pub fn set_connect_callback(&mut self, callback: Box<FutureConnectCb>) {
        self.connect_cb = Some(callback);
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

    pub async fn connect(&mut self) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.connect().await,
            Inner::V4(inner) => inner.connect().await,
            Inner::V5(inner) => inner.connect().await,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        match &mut self.inner {
            Inner::V3(inner) => inner.run_loop().await,
            Inner::V4(inner) => inner.run_loop().await,
            Inner::V5(inner) => inner.run_loop().await,
        }
    }

    pub async fn publish(&mut self, topic: &str, qos: QoS, payload: &[u8]) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.publish(topic, qos, payload).await,
            Inner::V4(inner) => inner.publish(topic, qos, payload).await,
            Inner::V5(inner) => inner.publish(topic, qos, payload).await,
        }
    }

    pub async fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Error> {
        match &mut self.inner {
            Inner::V3(inner) => inner.subscribe(topic, qos).await,
            Inner::V4(inner) => inner.subscribe(topic, qos).await,
            Inner::V5(inner) => inner.subscribe(topic, qos).await,
        }
    }
}
