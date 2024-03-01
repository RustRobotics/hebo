// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::fmt;
use std::io::Write;

use crate::{
    utils::validate_utf8_string, utils::StringError, ByteArray, DecodeError, DecodePacket,
    EncodeError, EncodePacket,
};

/// Text fields within the MQTT Control Packets described later are encoded as UTF-8 strings.
/// UTF-8 [RFC3629] is an efficient encoding of Unicode [Unicode] characters that
/// optimizes the encoding of ASCII characters in support of text-based communications.
///
/// String Data is represented by a Two Byte Integer length which indicates
/// the number of data bytes, followed by characters.
///
/// Thus, the length of String Data is limited to the range of 0 to 65,535 Bytes.
///
/// ```text
/// +-------------------+
/// | String Length     |
/// |                   |
/// +-------------------+
/// | String            |
/// |                   |
/// +-------------------+
/// ```
///
/// The character data in a UTF-8 Encoded String MUST be well-formed UTF-8 as defined
/// by the Unicode specification [Unicode] and restated in RFC 3629 [RFC3629].
/// In particular, the character data MUST NOT include encodings of code points
/// between U+D800 and U+DFFF [MQTT-1.5.4-1].
///
/// If the Client or Server receives an MQTT Control Packet containing ill-formed UTF-8
/// it is a Malformed Packet.
///
/// A UTF-8 Encoded String MUST NOT include an encoding of the null character U+0000. [MQTT-1.5.4-2].
///
/// If a receiver (Server or Client) receives an MQTT Control Packet containing U+0000
/// it is a Malformed Packet.
///
/// The data SHOULD NOT include encodings of the Unicode [Unicode] code points listed below.
/// If a receiver (Server or Client) receives an MQTT Control Packet containing any of them
/// it MAY treat it as a Malformed Packet. These are the Disallowed Unicode code points.
///
/// - U+0001..U+001F control characters
/// - U+007F..U+009F control characters
/// - Code points defined in the Unicode specification [Unicode] to be non-characters (for example U+0FFFF)
///
/// A UTF-8 encoded sequence 0xEF 0xBB 0xBF is always interpreted as U+FEFF ("ZERO WIDTH NO-BREAK SPACE")
/// wherever it appears in a string and MUST NOT be skipped over or stripped off
/// by a packet receiver [MQTT-1.5.4-3].
///
/// [RFC3629]: https://datatracker.ietf.org/doc/html/rfc3629
/// [Unicode]: https://unicode.org/standard/standard.html
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StringData(String);

impl StringData {
    /// Create an empty string data.
    #[must_use]
    pub const fn new() -> Self {
        Self(String::new())
    }

    /// Convert string slice into string data.
    ///
    /// # Errors
    ///
    /// Returns error if string slice is too large.
    pub fn from(s: &str) -> Result<Self, StringError> {
        validate_utf8_string(s)?;
        Ok(Self(s.to_string()))
    }

    /// Get byte length in packet.
    #[must_use]
    pub fn bytes(&self) -> usize {
        2 + self.0.len()
    }

    /// Returns true if string data is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clear string.
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl fmt::Display for StringData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for StringData {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl DecodePacket for StringData {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let len = ba.read_u16()?;
        let s = ba.read_string(len as usize)?;
        Ok(Self(s))
    }
}

impl EncodePacket for StringData {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        #[allow(clippy::cast_possible_truncation)]
        let len = self.0.len() as u16;
        buf.write_u16::<BigEndian>(len)?;
        buf.write_all(self.0.as_bytes())?;
        Ok(self.bytes())
    }
}
