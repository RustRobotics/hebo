// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::fmt;

use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket, StringData};

/// A UTF-8 String Pair consists of two UTF-8 Encoded Strings.
///
/// This data type is used to hold name-value pairs.
/// The first string serves as the name, and the second string contains the value.
///
/// Both strings MUST comply with the requirements for UTF-8 Encoded Strings [MQTT-1.5.7-1].
/// If a receiver (Client or Server) receives a string pair which does not meet
/// these requirements it is a Malformed Packet.
///
/// ```text
/// +----------------------+
/// | Key Length           |
/// |                      |
/// +----------------------+
/// | Key characters       |
/// |                      |
/// +----------------------+
/// | Value Length         |
/// |                      |
/// +----------------------+
/// | Value characters     |
/// |                      |
/// +----------------------+
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StringPairData(StringData, StringData);

impl StringPairData {
    /// Create a new string pair.
    ///
    /// # Errors
    ///
    /// Returns errors if key or value is too large.
    pub fn new(key: &str, value: &str) -> Result<Self, EncodeError> {
        let key = StringData::from(key)?;
        let value = StringData::from(value)?;
        Ok(Self(key, value))
    }

    /// Get key in pair.
    #[must_use]
    pub const fn key(&self) -> &StringData {
        &self.0
    }

    /// Get string value in pari.
    #[must_use]
    pub const fn value(&self) -> &StringData {
        &self.1
    }

    /// Get byte length in packet.
    #[must_use]
    pub fn bytes(&self) -> usize {
        self.0.bytes() + self.1.bytes()
    }
}

impl fmt::Display for StringPairData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl DecodePacket for StringPairData {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let key = StringData::decode(ba)?;
        let value = StringData::decode(ba)?;
        Ok(Self(key, value))
    }
}

impl EncodePacket for StringPairData {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        let key_len = self.0.encode(buf)?;
        let value_len = self.1.encode(buf)?;
        Ok(key_len + value_len)
    }
}
