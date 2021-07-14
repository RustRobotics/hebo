// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{CacheToDispatcherCmd, DispatcherToCacheCmd};
use crate::error::Error;

/// Key-value store.
#[derive(Debug)]
pub struct Cache {
    sender: Sender<CacheToDispatcherCmd>,
    receiver: Option<Receiver<DispatcherToCacheCmd>>,
    sys_message: SysMessageCache,
}

#[derive(Debug, Default)]
pub struct SysMessageCache {
    connections: usize,
    messages_sent: u64,
    messages_recv: u64,
    messages_queued: u64,
    message_bytes_queued: u64,
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
    }
}
