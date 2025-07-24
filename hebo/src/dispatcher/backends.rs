// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Backends app handlers

#![allow(clippy::unused_async)]

use codec::{v3, v5};

use super::Dispatcher;
use crate::commands::BackendsToDispatcherCmd;

impl Dispatcher {
    /// Send packet to backends.
    pub(super) async fn backends_store_packet(&mut self, _: &v3::PublishPacket) {}

    pub(super) async fn backends_store_packet_v5(&mut self, _: &v5::PublishPacket) {}

    pub(super) async fn handle_backends_cmd(&mut self, _: BackendsToDispatcherCmd) {}
}
