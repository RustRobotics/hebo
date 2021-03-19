// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use super::error::DecodeError;

/// Generate random string.
pub fn random_string(len: usize) -> String {
    String::from_utf8(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}

/// Check data length exceeds 64k or not.
#[inline]
pub fn validate_two_bytes_data(data: &[u8]) -> Result<(), DecodeError> {
    if data.len() > u16::MAX as usize {
        Err(DecodeError::TooManyData)
    } else {
        Ok(())
    }
}

/// Check string characters and length.
pub fn validate_utf8_string(s: &str) -> Result<(), DecodeError> {
    if s.len() > u16::MAX as usize {
        return Err(DecodeError::TooManyData);
    }

    for c in s.chars() {
        // Ignore control characters
        // No need to check chars between 0xd800 and 0xfffd as they are invalid coded point and not allowed.
        if (c >= '\u{0000}' && c <= '\u{001f}') || (c >= '\u{007f}' && c <= '\u{009f}') {
            return Err(DecodeError::InvalidString);
        }
    }

    // Empty string is valid.
    Ok(())
}

/// Convert range of bytes to valid UTF-8 string.
pub fn to_utf8_string(buf: &[u8]) -> Result<String, DecodeError> {
    let s = String::from_utf8(buf.to_vec())?;
    validate_utf8_string(&s)?;
    Ok(s)
}
