// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use openssl::hash::{Hasher, MessageDigest};
use rand::Rng;

use crate::error::Error;

pub const SALT_LEN: usize = 12;
pub const HASH_LEN: usize = 64;
pub const PW_SHA512: i32 = 6;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Salt([u8; SALT_LEN]);

#[derive(Debug, Clone, PartialEq)]
pub struct Passwd {
    // TODO(Shaohua): Replace with [u8; 64].
    passwd_hash: Vec<u8>,
    salt: Salt,
}

impl Passwd {
    pub fn hash(&self) -> &[u8] {
        &self.passwd_hash
    }

    pub fn salt(&self) -> &[u8] {
        &self.salt.0
    }

    /// Parse password entry from string.
    pub fn parse(_s: &str) -> Option<Self> {
        unimplemented!()
    }

    /// Generate password entry.
    pub fn dump(&self, username: &str) -> String {
        let salt = base64::encode(self.salt.0);
        let hash = base64::encode(&self.passwd_hash);
        format!("{}:${}${}${}", username, PW_SHA512, salt, hash)
    }

    pub fn generate(passwd: &[u8]) -> Result<Self, Error> {
        let salt = Salt(rand::thread_rng().gen());
        let mut h = Hasher::new(MessageDigest::sha512())?;
        h.update(passwd)?;
        h.update(&salt.0)?;
        let res = h.finish()?;
        Ok(Self {
            passwd_hash: res.to_vec(),
            salt,
        })
    }

    pub fn update(&mut self, passwd: &[u8]) -> Result<(), Error> {
        let mut h = Hasher::new(MessageDigest::sha512())?;
        h.update(passwd)?;
        h.update(&self.salt.0)?;
        let res = h.finish()?;
        self.passwd_hash = res.to_vec();
        Ok(())
    }

    pub fn is_match(&self, passwd: &[u8]) -> Result<bool, Error> {
        let mut h = Hasher::new(MessageDigest::sha512())?;
        h.update(passwd)?;
        h.update(&self.salt.0)?;
        let res = h.finish()?;
        Ok(self.passwd_hash == res.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dump() {
        let p = Passwd::generate(b"password").unwrap();
        let output = p.dump("username");
        assert_eq!(output.len(), 117);
    }

    #[test]
    fn test_generate() {
        let p = Passwd::generate(b"password");
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.hash().len(), HASH_LEN);
    }

    #[test]
    fn test_update() {
        let mut p = Passwd::generate(b"password").unwrap();
        assert!(p.update(b"new-password").is_ok());
    }

    #[test]
    fn test_is_match() {
        let p = Passwd::generate(b"password").unwrap();
        assert!(p.is_match(b"password").unwrap());
    }
}
