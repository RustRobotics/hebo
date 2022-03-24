// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use super::Error;

#[derive(Debug)]
pub struct ServerConfig {
    filename: String,
}

impl ServerConfig {
    pub fn new(filename: &str, content: &str) -> Result<Self, Error> {
        let path = Path::new(filename);
        let parent = path.parent().unwrap();
        println!("parent: {:?}", parent);
        fs::create_dir_all(parent)?;
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
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
            eprintln!("Failed to remove file: {}", self.filename);
        }
    }
}
