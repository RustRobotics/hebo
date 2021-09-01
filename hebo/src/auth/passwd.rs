// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use openssl::hash::{Hasher, MessageDigest};
use rand::Rng;

use crate::error::Error;

pub const SALT_LEN: usize = 12;
pub const HASH_LEN: usize = 64;

#[derive(Debug, Default, Clone, Copy)]
pub struct Salt([u8; SALT_LEN]);

#[derive(Debug, Clone)]
pub struct Passwd {
    passwd_hash: Vec<u8>,
    salt: Salt,
    valid: bool,
    iterations: u32,
}

impl Passwd {
    /// Parse password entry from string.
    pub fn parse(_s: &str) -> Option<Self> {
        unimplemented!()
    }

    pub fn generate(passwd: &[u8], iterations: u32) -> Result<Self, Error> {
        let salt = Salt(rand::thread_rng().gen());
        let mut h = Hasher::new(MessageDigest::sha512())?;
        h.update(passwd)?;
        h.update(&salt.0)?;
        let res = h.finish()?;
        Ok(Self {
            passwd_hash: res.to_vec(),
            salt,
            valid: true,
            iterations,
        })
    }

    pub fn update(&mut self, _passwd: &[u8]) {
        unimplemented!();
    }

    pub fn match_passwd(&self, _s: &[u8]) -> bool {
        // TODO(Shaohua):
        false
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }
}
