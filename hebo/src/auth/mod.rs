// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{AuthToListenerCmd, ListenerToAuthCmd, ServerContextToAuthCmd};
use crate::config::Security;
use crate::error::{Error, ErrorKind};
use crate::types::ListenerId;

#[allow(clippy::module_name_repetitions)]
pub mod db_auth;
#[allow(clippy::module_name_repetitions)]
pub mod file_auth;
mod listener;
pub mod pwd;
mod server;

use file_auth::FileAuth;

#[derive(Debug)]
pub struct AuthApp {
    allow_anonymous: bool,
    file_auth: Option<FileAuth>,

    listener_senders: Vec<(ListenerId, Sender<AuthToListenerCmd>)>,
    listener_receiver: Receiver<ListenerToAuthCmd>,

    server_ctx_receiver: Receiver<ServerContextToAuthCmd>,
}

impl AuthApp {
    pub fn new(
        security: Security,
        // listeners
        listener_senders: Vec<(ListenerId, Sender<AuthToListenerCmd>)>,
        listener_receiver: Receiver<ListenerToAuthCmd>,
        // server ctx module
        server_ctx_receiver: Receiver<ServerContextToAuthCmd>,
    ) -> Result<Self, Error> {
        let file_auth = if let Some(password_file) = security.password_file() {
            let file_auth = FileAuth::new(password_file).map_err(|err| {
                Error::from_string(
                    ErrorKind::ConfigError,
                    format!("Invalid password file: {:?}, err: {:?}", password_file, err),
                )
            })?;
            Some(file_auth)
        } else {
            None
        };

        Ok(Self {
            allow_anonymous: security.allow_anonymous(),
            file_auth,

            listener_senders,
            listener_receiver,

            server_ctx_receiver,
        })
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.listener_receiver.recv() => {
                    if let Err(err) = self.handle_listener_cmd(cmd).await {
                        log::error!("Failed to handle listener cmd: {:?}", err);
                    }
                },
                Some(cmd) = self.server_ctx_receiver.recv() => {
                    self.handle_server_ctx_cmd(cmd).await;
                }
            }
        }
    }
}
