// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    // Connections
    pub mqtt: String,
    pub mqtts: String,
    pub ws: String,
    pub wss: String,

    // Security
    pub enable_anonymous: bool,

    // Storage
    pub persistent_storage: bool,
    pub db_path: String,

    // Log
    pub console_log: bool,
    pub log_file: Option<PathBuf>,
}
