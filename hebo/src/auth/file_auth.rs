// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use super::passwd::Passwd;
use crate::error::Error;

pub fn update_file_hash<P: AsRef<Path>>(password_file: P) -> Result<(), Error> {
    let fd = File::open(password_file.as_ref())?;
    let reader = BufReader::new(fd);
    for line in reader.lines() {
        let line = line?;
        match Passwd::parse_raw_text(&line) {
            Err(err) => {
                log::error!("err: {:?}, line: {}", err, line);
            }
            Ok(None) => {
                // continue
            }
            Ok(Some((username, passwd))) => {
                let hashed_line = passwd.dump(username);
                println!("{}", hashed_line);
            }
        }
    }

    Ok(())
}
