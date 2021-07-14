// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{CacheToDispatcherCmd, DispatcherToCacheCmd};
use crate::error::Error;

/// Key-value store.
#[derive(Debug)]
pub struct Cache {
    sender: Sender<CacheToDispatcherCmd>,
    receiver: Option<Receiver<DispatcherToCacheCmd>>,
    sys_message: SysMessageCache,

    listeners: HashMap<u32, ListenerCache>,
}

#[derive(Debug, Default)]
pub struct ListenerCache {
    pub id: u32,
    pub address: String,

    pub sessions: u64,
}

#[derive(Debug, Default)]
pub struct SysMessageCache {
    pub messages_sent: u64,
    pub messages_received: u64,

    pub bytes_sent: u64,
    pub bytes_received: u64,

    pub retained_messages_count: u64,
    pub retained_messages_bytes: u64,

    pub publish_messages_dropped: u64,
    pub publish_messages_sent: u64,
    pub publish_messages_received: u64,

    pub publish_bytes_dropped: u64,
    pub publish_bytes_sent: u64,
    pub publish_bytess_received: u64,
}

impl Cache {
    pub fn new(
        sender: Sender<CacheToDispatcherCmd>,
        receiver: Receiver<DispatcherToCacheCmd>,
    ) -> Self {
        Cache {
            sender,
            receiver: Some(receiver),
            sys_message: SysMessageCache::default(),
            listeners: HashMap::new(),
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        let mut dispatcher_receiver = self.receiver.take().expect("Invalid dispatcher receiver");
        loop {
            if let Some(cmd) = dispatcher_receiver.recv().await {
                self.handle_dispatcher_cmd(cmd).await;
            }
        }
    }

    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToCacheCmd) {
        log::info!("cmd: {:?}", cmd);
        match cmd {
            DispatcherToCacheCmd::ListenerAdded(id, address) => {
                log::info!("listener id: {}, addr: {:?}", id, address);
            }
            DispatcherToCacheCmd::PublishPacketSent(count, bytes) => {
                log::info!("count {}, bytes: {:?}", count, bytes);
            }
            _ => {
                // Do nothing
            }
        }
    }
}
