// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::commands::StorageCommand;
use crate::sys_message::SysMessage;

#[derive(Debug)]
pub struct Storage {
    sys_message: SysMessage,

    tx: Sender<StorageCommand>,
    rx: Receiver<StorageCommand>,
}

impl Storage {
    pub fn new() -> Self {
        let sys_message = SysMessage::new();
        let (tx, rx) = mpsc::channel(10);
        Storage {
            sys_message,
            tx,
            rx,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.rx.recv() => {
                    log::info!("cmd: {:?}", cmd);
                },
            }
        }
    }
}
