// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{AuthToListenerCmd, ListenerId, ListenerToAuthCmd};
use crate::config::Security;

#[derive(Debug)]
pub struct AuthApp {
    security: Security,
    listener_senders: Vec<(ListenerId, Sender<AuthToListenerCmd>)>,
    listener_receiver: Receiver<ListenerToAuthCmd>,
}

impl AuthApp {
    pub fn new(
        security: Security,
        listener_senders: Vec<(ListenerId, Sender<AuthToListenerCmd>)>,
        listener_receiver: Receiver<ListenerToAuthCmd>,
    ) -> Self {
        Self {
            security,
            listener_senders,
            listener_receiver,
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

    async fn handle_listener_cmd(&mut self, cmd: ListenerToAuthCmd) {
        log::info!("AuthApp::handle_listener_cmd(), cmd: {:?}", cmd);
    }
}
