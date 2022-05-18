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
pub struct Password {
    salt: Salt,
    password_hash: Hash,
    valid: bool,
}

impl Password {
    #[must_use]
    pub fn hash(&self) -> &[u8] {
        &self.password_hash.0
    }

    #[must_use]
    pub fn salt(&self) -> &[u8] {
        &self.salt.0
    }

    #[must_use]
    pub fn valid(&self) -> bool {
        self.valid
    }

    pub fn parse_raw_text(s: &str) -> Result<Option<(&str, Self)>, Error> {
        if s.is_empty() {
            return Ok(None);
        }
        // Ignore comment lines.
        if s.starts_with('#') {
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
        if username.is_empty() {
            return Err(Error::from_string(
                ErrorKind::FormatError,
                format!("Username is empty in entry: {:?}", s),
            ));
        }
        let password = parts[1];
        let password = Self::generate(password.as_bytes())?;

        Ok(Some((username, password)))
    }

    /// Parse password entry from string.
    ///
    /// Returns (username, Password) pair if success.
    pub fn parse(s: &str) -> Result<Option<(&str, Self)>, Error> {
        if s.is_empty() {
            return Ok(None);
        }
        // Ignore comment lines.
        if s.starts_with('#') {
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
        if username.is_empty() {
            return Err(Error::from_string(
                ErrorKind::FormatError,
                format!("Username is empty in entry: {:?}", s),
            ));
        }
        let password = Self::parse_password(parts[1])?;

        Ok(Some((username, password)))
    }

    fn parse_password(s: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = s.split('$').collect();
        let err = Err(Error::from_string(
            ErrorKind::FormatError,
            format!("Invalid password : {:?}", s),
        ));
        if parts.len() != 4 {
            return err;
        }
        if let Ok(hash_type) = parts[1].parse::<i32>() {
            if hash_type != PW_SHA512 {
                return err;
            }
        } else {
            return err;
        }

        let salt = base64::decode(parts[2])?;
        let salt = Salt::from_slice(&salt);
        let password_hash = base64::decode(parts[3])?;
        let password_hash = Hash::from_slice(&password_hash);
        Ok(Self {
            salt,
            password_hash,
            valid: true,
        })
    }

    /// Generate password entry.
    #[must_use]
    pub fn dump(&self, username: &str) -> String {
        if self.valid {
            let salt = base64::encode(self.salt.0);
            let hash = base64::encode(self.password_hash.0);
            format!("{}:${}${}${}", username, PW_SHA512, salt, hash)
        } else {
            format!("{}:", username)
        }
    }

    pub fn generate(password: &[u8]) -> Result<Self, Error> {
        let salt = Salt(rand::thread_rng().gen());
        if password.is_empty() {
            return Ok(Self {
                salt,
                password_hash: Hash::new(),
                valid: false,
            });
        }
        let mut h = Hasher::new(MessageDigest::sha512())?;
        h.update(password)?;
        h.update(&salt.0)?;
        let res = h.finish()?;
        assert_eq!(res.as_ref().len(), HASH_LEN);
        let password_hash = Hash::from_slice(res.as_ref());
        Ok(Self {
            salt,
            password_hash,
            valid: true,
        })
    }

    pub fn update(&mut self, password: &[u8]) -> Result<(), Error> {
        let mut h = Hasher::new(MessageDigest::sha512())?;
        h.update(password)?;
        h.update(&self.salt.0)?;
        let res = h.finish()?;
        assert_eq!(res.as_ref().len(), HASH_LEN);
        self.password_hash.0.copy_from_slice(res.as_ref());
        Ok(())
    }

    pub fn is_match(&self, password: &[u8]) -> Result<bool, Error> {
        let mut h = Hasher::new(MessageDigest::sha512())?;
        h.update(password)?;
        h.update(&self.salt.0)?;
        let res = h.finish()?;
        Ok(self.password_hash.0 == res.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dump() {
        let p = Password::generate(b"password").unwrap();
        let output = p.dump("username");
        assert_eq!(output.len(), 117);
    }

    #[test]
    fn test_generate() {
        let p = Password::generate(b"password");
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.hash().len(), HASH_LEN);
    }

    #[test]
    fn test_update() {
        let mut p = Password::generate(b"password").unwrap();
        assert!(p.update(b"new-password").is_ok());
    }

    #[test]
    fn test_is_match() {
        let p = Password::generate(b"password").unwrap();
        assert!(p.is_match(b"password").unwrap());
    }
}
