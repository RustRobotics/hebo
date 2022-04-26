// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use hebo::error::{Error, ErrorKind};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct Server {
    config_file: String,
    exec_file: PathBuf,
    handler: JoinHandle<()>,
}

impl Server {
    pub fn start(config_file: &str) -> Result<Self, Error> {
        let config_file = config_file.to_string();
        let config_file_clone = config_file.clone();
        let exec_file = Self::get_exec_file()?;
        let exec_file_clone = exec_file.clone();

        let handler = thread::spawn(move || {
            let output = Command::new(exec_file_clone)
                .args(&["-c", &config_file_clone])
                .output()
                .expect("Failed to run hebo server");
            assert!(output.status.success());
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        });
        Ok(Self {
            config_file,
            exec_file,
            handler,
        })
    }

    pub fn terminate(self) {
        let ret = Command::new(&self.exec_file)
            .args(["-c", &self.config_file, "-s"])
            .spawn();
        assert!(ret.is_ok());
        let ret = self.handler.join();
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
