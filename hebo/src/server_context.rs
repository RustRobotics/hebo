// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::runtime::Runtime;

use crate::config::Config;
use crate::error::Error;
use crate::listener;

/// ServerContext manages lifetime of Storage and Listeners.
/// All kernel signals are handled here.
#[derive(Debug)]
pub struct ServerContext {
    config: Config,
}

impl ServerContext {
    pub fn new(config: Config) -> ServerContext {
        let (connection_tx, connection_rx) = mpsc::channel(10);
        ServerContext {
            config,
            connection_rx,
            connection_tx,
            current_connection_id: 0,
            sys_message: SysMessage::new(),
        }
    }

    pub async fn run_loop(&mut self, runtime: Runtime) -> Result<(), Error> {
        let mut handles = Vec::new();

        for l in &self.config.listeners.clone() {
            let handle = runtime.spawn(async {
                let listener = listener::Listener::bind(l)
                    .await
                    .expect(&format!("Failed to listen at {:?}", l));
                listener.run_loop().await;
            });
            handles.push(handle);
        }

        //runtime.spawn(async {
        //    if let Some(cmd) = self.connection_rx.recv() {
        //        self.route_cmd(cmd).await;
        //    }
        //});
        //for handle in handles {
        //handle.await;
        //}
        Ok(())
    }
}
