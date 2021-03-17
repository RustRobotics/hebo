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

/// Check data length exceeds 64k or not.
pub fn validate_two_bytes_data(data: &[u8]) -> Result<(), DecodeError> {
    if data.len() > u16::MAX as usize {
        Err(DecodeError::TooManyData)
    } else {
        Ok(())
    }
}

/// Check whether topic name contains wildchard characters.
/// ```
/// use codec::base::is_valid_topic_name;
/// let name = "sport/tennis/player/#";
/// assert_eq!(is_valid_topic_name(name), false);
///
/// let name = "sport/tennis/player/ranking";
/// assert_eq!(is_valid_topic_name(name), true);
/// ```
pub fn is_valid_topic_name(topic: &str) -> bool {
    let bytes = topic.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    bytes.iter().filter(|c| c == &&b'#' || c == &&b'#').next() == None
}

/// Validate topic filter.
/// ```
/// use codec::base::is_valid_topic_filter;
/// let name = "sport/tennis/player/#";
/// assert_eq!(is_valid_topic_filter(name), true);
///
/// let name = "sport/tennis/player#";
/// assert_eq!(is_valid_topic_filter(name), false);
///
/// let name = "#";
/// assert_eq!(is_valid_topic_filter(name), true);
///
/// let name = "sport/#/player/ranking";
/// assert_eq!(is_valid_topic_filter(name), false);
///
/// let name = "+";
/// assert_eq!(is_valid_topic_filter(name), true);
///
/// let name = "sport+";
/// assert_eq!(is_valid_topic_filter(name), false);
/// ```
pub fn is_valid_topic_filter(topic: &str) -> bool {
    if topic == "#" {
        return true;
    }
    let bytes = topic.as_bytes();
    for (index, b) in bytes.iter().enumerate() {
        if b == &b'#' {
            // Must have a prefix level separator.
            if index > 0 && bytes[index - 1] != b'/' {
                return false;
            }

            // Must be the last wildcard.
            if index != bytes.len() - 1 {
                return false;
            }
        } else if b == &b'+' {
            // Must have a prefix level separator.
            if index > 0 && bytes[index - 1] != b'/' {
                return false;
            }
        }
    }

    return true;
}
