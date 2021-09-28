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

/// Generate random client id in valid characters.
pub fn random_client_id() -> Result<String, StringError> {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(14..22);
    String::from_utf8(
        rng.sample_iter(&Alphanumeric)
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
    // The character data in a UTF-8 encoded string MUST be well-formed UTF-8 as
    // defined by the Unicode specification [Unicode] and restated in RFC 3629
    // [RFC3629]. In particular this data MUST NOT include encodings of code points
    // between U+D800 and U+DFFF. If a Server or Client receives a Control Packet
    // containing ill-formed UTF-8 it MUST close the Network Connection. [MQTT-1.5.3-1]
    //
    // A UTF-8 encoded string MUST NOT include an encoding of the null character
    // U+0000. If a receiver (Server or Client) receives a Control Packet containing
    // U+0000 it MUST close the Network Connection. [MQTT-1.5.3-2]
    //
    // A UTF-8 encoded sequence 0xEF 0xBB 0xBF is always to be interpreted to
    // mean U+FEFF ("ZERO WIDTH NO-BREAK SPACE") wherever it appears in a
    // string and MUST NOT be skipped over or stripped off by a packet receiver. [MQTT-1.5.3-3]
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

/// ClientId is based on rules below:
///
/// - The ClientId MUST be a UTF-8 encoded string as defined in Section 1.5.3 [MQTT-3.1.3-4].
///
/// - The Server MUST allow ClientIds which are between 1 and 23 UTF-8 encoded bytes in length, and that
///   contain only the characters
///   "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" [MQTT-3.1.3-5].
pub fn validate_client_id(id: &str) -> Result<(), StringError> {
    if id.is_empty() || id.len() > 23 {
        return Err(StringError::InvalidLength);
    }
    for byte in id.bytes() {
        if !((b'0'..=b'9').contains(&byte)
            || (b'a'..=b'z').contains(&byte)
            || (b'A'..=b'Z').contains(&byte))
        {
            return Err(StringError::InvalidChar);
        }
    }
    Ok(())
}
