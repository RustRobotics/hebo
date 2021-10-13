// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use subprocess::unix::PopenExt;
use subprocess::{Popen, PopenConfig};

#[derive(Debug)]
pub enum Error {
    PopenError(subprocess::PopenError),
    IoError(std::io::Error),
}

#[derive(Debug)]
pub struct Server {
    p: Popen,
}

impl Server {
    pub fn start() -> Result<Self, Error> {
        let p = Popen::create(&["./target/release/hebo", "-c"], PopenConfig::default())
            .map_err(|err| Error::PopenError(err))?;
        Ok(Self { p })
    }

    pub fn terminate(&mut self) -> Result<(), Error> {
        self.p.terminate().map_err(|err| Error::IoError(err))
    }
}
