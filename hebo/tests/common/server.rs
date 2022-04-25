// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use hebo::error::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::Command;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use subprocess::Exec;

pub struct Server {
    config_file: String,
    exec_file: PathBuf,
    _handler: JoinHandle<()>,
}

impl Server {
    pub fn start(config_file: &str) -> Result<Self, Error> {
        let config_file = config_file.to_string();
        let config_file_clone = config_file.clone();
        let exec_file = Self::get_exec_file()?;
        let exec_file_clone = exec_file.clone();

        let _handler = thread::spawn(move || {
            let exec = Exec::cmd(exec_file_clone)
                .args(&["-c", &config_file_clone])
                .detached();
            if let Err(err) = exec.join() {
                eprintln!("Failed to run server program, err: {:?}", err);
            }
        });
        Ok(Self {
            config_file,
            exec_file,
            _handler,
        })
    }

    pub fn terminate(&mut self) {
        let ret = Command::new(&self.exec_file)
            .args(["-c", &self.config_file, "-s"])
            .spawn();
        assert!(ret.is_ok());
        thread::sleep(Duration::from_secs(1));
    }

    fn get_exec_file() -> Result<PathBuf, Error> {
        const IN_CURR_DIR: &str = "./target/debug/hebo";
        const IN_PARENT_DIR: &str = "../target/debug/hebo";
        let path = PathBuf::from(IN_CURR_DIR);
        if path.exists() {
            return Ok(path);
        }
        let path = PathBuf::from(IN_PARENT_DIR);
        if path.exists() {
            return Ok(path);
        }
        return Err(Error::new(
            ErrorKind::IoError,
            "Make sure hebo binary is compiled!",
        ));
    }
}
