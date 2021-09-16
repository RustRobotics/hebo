// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Handles commands and new connections

use codec::SubscribePattern;
use tokio::sync::mpsc;

use super::Listener;
use super::CHANNEL_CAPACITY;
use crate::commands::ListenerToDispatcherCmd;
use crate::session::Session;
use crate::stream::Stream;

impl Listener {
    pub async fn run_loop(&mut self) -> ! {
        // Take ownership of mpsc receiver or else tokio select will raise error.
        let mut session_receiver = self
            .session_receiver
            .take()
            .expect("Invalid session receiver");

        let mut dispatcher_receiver = self
            .dispatcher_receiver
            .take()
            .expect("Invalid dispatcher receiver");
        let mut auth_receiver = self.auth_receiver.take().expect("Invalid auth receiver");
        let mut acl_receiver = self.acl_receiver.take().expect("Invalid acl receiver");

        loop {
            tokio::select! {
                Ok(stream) = self.accept() => {
                    self.new_connection(stream).await;
                },

                Some(cmd) = session_receiver.recv() => {
                    if let Err(err) = self.handle_session_cmd(cmd).await {
                        log::error!("handle session cmd failed: {:?}", err);
                    }
                },

                Some(cmd) = dispatcher_receiver.recv() => {
                    self.handle_dispatcher_cmd(cmd).await;
                }

                Some(cmd) = auth_receiver.recv() => {
                    if let Err(err) = self.handle_auth_cmd(cmd).await {
                        log::error!("handle auth cmd failed: {:?}", err);
                    }
                }

                Some(cmd) = acl_receiver.recv() => {
                    if let Err(err) = self.handle_acl_cmd(cmd).await {
                        log::error!("handle acl cmd failed: {:?}", err);
                    }
                }
            }
        }
    }

    async fn new_connection(&mut self, stream: Stream) {
        let (sender, receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let session_id = self.next_session_id();
        self.session_senders.insert(session_id, sender);
        let session = Session::new(session_id, stream, self.session_sender.clone(), receiver);
        tokio::spawn(session.run_loop());

        if let Err(err) = self
            .dispatcher_sender
            .send(ListenerToDispatcherCmd::SessionAdded(self.id))
            .await
        {
            log::error!("Failed to send NewSession cmd: {:?}", err);
        }
    }
}
