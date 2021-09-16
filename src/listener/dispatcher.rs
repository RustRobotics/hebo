// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Dispatcher cmd handlers.

use codec::PublishPacket;

use super::Listener;
use crate::commands::{DispatcherToListenerCmd, ListenerToSessionCmd};

impl Listener {
    pub(super) async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToListenerCmd) {
        match cmd {
            DispatcherToListenerCmd::Publish(packet) => self.on_dispatcher_publish(packet).await,
        }
    }

    async fn on_dispatcher_publish(&mut self, packet: PublishPacket) {
        let cmd = ListenerToSessionCmd::Publish(packet.clone());
        // TODO(Shaohua): Replace with a trie tree and a hash table.

        // TODO(Shaohua): Handle errors
        /*
        for (_, session_sender) in self.session_senders.iter_mut() {
            if topic_match(&pipeline.topics, packet.topic()) {
                if let Err(err) = pipeline.sender.send(cmd.clone()).await {
                    log::warn!(
                        "Failed to send publish packet from listener to session: {:?}",
                        err
                    );
                }
            }
        }
        */
    }
}
