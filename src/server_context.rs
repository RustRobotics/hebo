// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use super::commands::{ConnectionCommand, ServerCommand};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
pub struct ServerContext {
    pipelines: Vec<Pipeline>,
}

impl ServerContext {
    pub fn new() -> ServerContext {
        ServerContext {
            pipelines: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Pipeline {
    sender: Sender<ServerCommand>,
    receiver: Receiver<ConnectionCommand>,
    topics: Vec<String>,
}

impl Pipeline {
    pub fn new(sender: Sender<ServerCommand>, receiver: Receiver<ConnectionCommand>) -> Pipeline {
        Pipeline {
            sender,
            receiver,
            topics: Vec::new(),
        }
    }
}
