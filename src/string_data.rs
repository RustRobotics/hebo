// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StringData(String);

impl StringData {
    pub fn new(data: &str) -> Result<Self, StringError> {
        validate_utf8_string(data)?;
        Ok(Self(data.to_string()))
    }

    pub fn set_data(&mut self, data: &str) -> Result<(), StringError> {
        validate_utf8_string(data)?;
        self.0 = data.to_string();
        Ok(())
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
        buf.write_u16::<BigEndian>(self.0.len() as u16)?;
        buf.write_all(self.0.as_bytes())?;
        Ok(2 + self.0.len())
    }
}
