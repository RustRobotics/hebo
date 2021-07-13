// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{DispatcherToListenerCmd, ListenerToDispatcherCmd};
use crate::sys_message::SysMessage;

/// Dispatcher is a message router.
#[derive(Debug)]
pub struct Dispatcher {
    sys_message: SysMessage,

    listener_receiver: Receiver<ListenerToDispatcherCmd>,
    listener_senders: Vec<Sender<DispatcherToListenerCmd>>,
}

impl Dispatcher {
    pub fn new(
        listener_receiver: Receiver<ListenerToDispatcherCmd>,
        listener_senders: Vec<Sender<DispatcherToListenerCmd>>,
    ) -> Self {
        let sys_message = SysMessage::new();
        Dispatcher {
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

    async fn handle_listener_cmd(&mut self, cmd: ListenerToDispatcherCmd) {
        log::info!("handle_listener_cmd: {:?}", cmd);
        match cmd {
            ListenerToDispatcherCmd::Publish(packet) => {
                for s in &self.listener_senders {
                    let cmd = DispatcherToListenerCmd::Publish(packet.clone());
                    if let Err(err) = s.send(cmd).await {
                        log::error!("Dispatcher::handle_listener_cmd() send failed: {:?}", err);
                    }
                }
            }
        }
    }
}
