// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use hebo::error::{Error, ErrorKind};
use std::fs;
use std::path::Path;

/// `ServerConfig` is used to save config string to local filesystem.
///
/// That config file will be cleanup on drop.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ServerConfig {
    filename: String,
}

impl ServerConfig {
    pub fn new(filename: &str, content: &str) -> Result<Self, Error> {
        let path = Path::new(filename);
        let parent = path.parent().unwrap();
        fs::create_dir_all(parent).map_err(|err| {
            Error::from_string(
                ErrorKind::ConfigError,
                format!(
                    "Failed to create parent directory for config: {}, err: {}",
                    filename, err
                ),
            )
        })?;
        fs::write(filename, content).map_err(|err| {
            Error::from_string(
                ErrorKind::ConfigError,
                format!("Failed to write to config file: {}, err: {}", filename, err),
            )
        })?;
        Ok(Self {
            filename: filename.to_string(),
        })
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }
}

impl Drop for ServerConfig {
    fn drop(&mut self) {
        if let Err(err) = std::fs::remove_file(&self.filename) {
            eprintln!("Failed to remove file: {}, err: {:?}", self.filename, err);
        }
    }
}
