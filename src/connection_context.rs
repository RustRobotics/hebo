// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::commands::{ConnectionCommand, ServerCommand};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
pub struct ConnectionContext {
    remote_address: String,
    sender: Sender<ConnectionCommand>,
    receiver: Receiver<ServerCommand>,
}

impl ConnectionContext {
    pub fn new(
        addr: &str,
        sender: Sender<ConnectionCommand>,
        receiver: Receiver<ServerCommand>,
    ) -> ConnectionContext {
        ConnectionContext {
            remote_address: addr.to_string(),
            sender,
            receiver,
        }
    }
}
