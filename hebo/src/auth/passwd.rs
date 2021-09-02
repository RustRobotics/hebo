// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use openssl::hash::{Hasher, MessageDigest};
use rand::Rng;

use crate::error::{Error, ErrorKind};

pub const SALT_LEN: usize = 12;
pub const HASH_LEN: usize = 64;
pub const PW_SHA512: i32 = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Salt([u8; SALT_LEN]);

impl Salt {
    fn from_slice(s: &[u8]) -> Self {
        let mut v = [0; SALT_LEN];
        v.copy_from_slice(s);
        Self(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hash([u8; HASH_LEN]);

impl Hash {
    fn new() -> Self {
        Self([0; HASH_LEN])
    }
    fn from_slice(s: &[u8]) -> Self {
        let mut v = [0; HASH_LEN];
        v.copy_from_slice(s);
        Self(v)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Passwd {
    salt: Salt,
    passwd_hash: Hash,
    valid: bool,
}

impl Passwd {
    pub fn hash(&self) -> &[u8] {
        &self.passwd_hash.0
    }

    pub fn salt(&self) -> &[u8] {
        &self.salt.0
    }

    pub fn valid(&self) -> bool {
        self.valid
    }

    pub fn parse_raw_text(s: &str) -> Result<Option<(&str, Self)>, Error> {
        if s.is_empty() {
            return Ok(None);
        }
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::from_string(
                ErrorKind::FormatError,
                format!("Invalid password entry: {:?}", s),
            ));
        }
        let username = parts[0];
        let passwd = parts[1];
        let passwd = Self::generate(passwd.as_bytes())?;

        Ok(Some((username, passwd)))
    }

    /// Parse password entry from string.
    ///
    /// Returns (username, Password) pair if success.
    pub fn parse(s: &str) -> Result<Option<(&str, Self)>, Error> {
        if s.is_empty() {
            return Ok(None);
        }

        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::from_string(
                ErrorKind::FormatError,
                format!("Invalid password entry: {:?}", s),
            ));
        }
        let username = parts[0];
        let passwd = Self::parse_passwd(parts[1])?;

        Ok(Some((username, passwd)))
    }

    fn parse_passwd(s: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = s.split('$').collect();
        if parts.len() != 3 {
            return Err(Error::from_string(
                ErrorKind::FormatError,
                format!("Invalid password : {:?}", s),
            ));
        }
        let salt = Salt::from_slice(parts[1].as_bytes());
        let passwd_hash = Hash::from_slice(parts[2].as_bytes());
        Ok(Self {
            salt,
            passwd_hash,
            valid: true,
        })
    }

    /// Generate password entry.
    pub fn dump(&self, username: &str) -> String {
        if self.valid {
            let salt = base64::encode(self.salt.0);
            let hash = base64::encode(self.passwd_hash.0);
            format!("{}:${}${}${}", username, PW_SHA512, salt, hash)
        } else {
            format!("{}:", username)
        }
    }

    pub fn generate(passwd: &[u8]) -> Result<Self, Error> {
        let salt = Salt(rand::thread_rng().gen());
        if passwd.is_empty() {
            return Ok(Self {
                salt,
                passwd_hash: Hash::new(),
                valid: false,
            });
        }
        let mut h = Hasher::new(MessageDigest::sha512())?;
        h.update(passwd)?;
        h.update(&salt.0)?;
        let res = h.finish()?;
        assert_eq!(res.as_ref().len(), HASH_LEN);
        let passwd_hash = Hash::from_slice(res.as_ref());
        Ok(Self {
            salt,
            passwd_hash,
            valid: true,
        })
    }

    pub fn update(&mut self, passwd: &[u8]) -> Result<(), Error> {
        let mut h = Hasher::new(MessageDigest::sha512())?;
        h.update(passwd)?;
        h.update(&self.salt.0)?;
        let res = h.finish()?;
        assert_eq!(res.as_ref().len(), HASH_LEN);
        self.passwd_hash.0.copy_from_slice(res.as_ref());
        Ok(())
    }

    pub fn is_match(&self, passwd: &[u8]) -> Result<bool, Error> {
        let mut h = Hasher::new(MessageDigest::sha512())?;
        h.update(passwd)?;
        h.update(&self.salt.0)?;
        let res = h.finish()?;
        Ok(self.passwd_hash.0 == res.as_ref())
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
