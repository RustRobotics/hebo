// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::process::Command;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use subprocess::{Exec, Popen, PopenConfig, PopenError};

use super::Error;

#[derive(Debug)]
pub struct Server {
    config: String,
    handler: JoinHandle<()>,
}

const PROGRAM: &str = "./target/debug/hebo";

impl Server {
    pub fn start(config: &str) -> Result<Self, Error> {
        let config_str = config.to_string();
        let handler = thread::spawn(move || {
            let exec = Exec::cmd(PROGRAM).args(&["-c", &config_str]).detached();
            if let Err(err) = exec.join() {
                eprintln!("Failed to run server program");
            }
        });
        Ok(Self {
            handler,
            config: config.to_string(),
        })
    }

    pub fn terminate(&mut self) {
        Command::new(PROGRAM)
            .args(["-c", &self.config, "-s"])
            .spawn();
        thread::sleep(Duration::from_secs(1));
    }
}
