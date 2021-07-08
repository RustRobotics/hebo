// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{ListenerToStorageCmd, StorageToListenerCmd};
use crate::sys_message::SysMessage;

#[derive(Debug)]
pub struct Storage {
    sys_message: SysMessage,

    listener_receiver: Receiver<ListenerToStorageCmd>,
    listener_senders: Vec<Sender<StorageToListenerCmd>>,
}

impl Storage {
    pub fn new(
        listener_receiver: Receiver<ListenerToStorageCmd>,
        listener_senders: Vec<Sender<StorageToListenerCmd>>,
    ) -> Self {
        let sys_message = SysMessage::new();
        Storage {
            sys_message,
            listener_receiver,
            listener_senders,
        }
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.listener_receiver.recv() => {
                    self.handle_listener_cmd(cmd).await;
                },
            }
        }
    }

    async fn handle_listener_cmd(&mut self, cmd: ListenerToStorageCmd) {
        log::info!("handle_listener_cmd: {:?}", cmd);
        match cmd {
            ListenerToStorageCmd::Publish(packet) => {
                for s in &self.listener_senders {
                    let cmd = StorageToListenerCmd::Publish(packet.clone());
                    if let Err(err) = s.send(cmd).await {
                        log::error!("Storage::handle_listener_cmd() send failed: {:?}", err);
                    }
                }
            }
        }
    }
}
