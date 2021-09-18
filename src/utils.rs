// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Generate random string.
pub fn random_string(len: usize) -> Result<String, StringError> {
    String::from_utf8(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .collect::<Vec<u8>>(),
    )
    .map_err(|_err| StringError::InvalidRandomString)
}

#[derive(Debug, PartialEq)]
// Invalid UTF-8 string.
pub enum StringError {
    TooManyData,

    InvalidLength,

    InvalidChar,

    /// Server or client shall DISCONNECT immediately.
    SeriousError,

    InvalidRandomString,
}

impl From<std::string::FromUtf8Error> for StringError {
    fn from(_e: std::string::FromUtf8Error) -> StringError {
        StringError::SeriousError
    }
}

/// Check data length exceeds 64k or not.
#[inline]
pub fn validate_two_bytes_data(data: &[u8]) -> Result<(), StringError> {
    if data.len() > u16::MAX as usize {
        Err(StringError::TooManyData)
    } else {
        Ok(())
    }
}

/// Check string characters and length.
pub fn validate_utf8_string(s: &str) -> Result<(), StringError> {
    if s.len() > u16::MAX as usize {
        return Err(StringError::TooManyData);
    }

    for c in s.chars() {
        // Check control characters
        if c == '\u{0000}' {
            return Err(StringError::SeriousError);
        }

        // Not need to Check chars between 0xd800 and 0xfffd as they are invalid coded point and not allowed.
        //if c >= '\u{d800}' && c <= '\u{fffd}' {
        //    return Err(StringError::InvalidStringSerious);
        //}

        if ('\u{0001}'..='\u{001f}').contains(&c) || ('\u{007f}'..='\u{009f}').contains(&c) {
            return Err(StringError::InvalidChar);
        }
    }

    // Empty string is valid.
    Ok(())
}

/// Convert range of bytes to valid UTF-8 string.
pub fn to_utf8_string(buf: &[u8]) -> Result<String, StringError> {
    let s = String::from_utf8(buf.to_vec())?;
    validate_utf8_string(&s)?;
    Ok(s)
}
