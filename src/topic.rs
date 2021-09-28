// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

use crate::QoS;
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

// TODO(Shaohua): Simplify topic structs.

#[derive(Debug, Default, Clone, Eq, PartialOrd, Ord, Hash)]
pub struct Topic {
    topic: String,
    parts: Vec<TopicPart>,
}

#[derive(Debug, PartialEq)]
pub enum TopicError {
    EmptyTopic,
    TooManyData,
    InvalidChar,
    ContainsWildChar,
}

impl PartialEq for Topic {
    fn eq(&self, other: &Self) -> bool {
        self.topic.eq(&other.topic)
    }
}

impl Topic {
    // TODO(Shaohua): Replace with `std::str::FromStr` trait.
    pub fn parse(s: &str) -> Result<Topic, TopicError> {
        let parts = Topic::parse_parts(s)?;
        Ok(Topic {
            topic: s.to_string(),
            parts,
        })
    }

    fn parse_parts(s: &str) -> Result<Vec<TopicPart>, TopicError> {
        s.split('/').map(|part| TopicPart::parse(part)).collect()
    }

    pub fn is_match(&self, s: &str) -> bool {
        for (index, part) in s.split('/').into_iter().enumerate() {
            if self.parts.len() - 1 < index {
                return false;
            }
            match self.parts[index] {
                TopicPart::Empty => return false,
                TopicPart::Normal(ref s_part) => {
                    if s_part != part {
                        return false;
                    }
                }
                TopicPart::Internal(ref s_part) => {
                    if s_part != part {
                        return false;
                    }
                }
                TopicPart::SingleWildcard => {
                    // Continue
                }
                TopicPart::MultiWildcard => return true,
            }
        }
        true
    }

    pub fn topic(&self) -> &String {
        &self.topic
    }

    pub fn len(&self) -> usize {
        self.topic.len()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.topic.as_bytes()
    }
}

/// Validate topic filter.
/// Rules are defined in `MQTT chapter-4.7 Topic Name and Filters`
/// ```
/// use codec::Topic;
/// let name = "sport/tennis/player/#";
/// assert!(Topic::validate_sub_topic(name).is_ok());
///
/// let name = "sport/tennis/player#";
/// assert!(Topic::validate_sub_topic(name).is_err());
///
/// let name = "#";
/// assert!(Topic::validate_sub_topic(name).is_ok());
///
/// let name = "sport/#/player/ranking";
/// assert!(Topic::validate_sub_topic(name).is_err());
///
/// let name = "+";
/// assert!(Topic::validate_sub_topic(name).is_ok());
///
/// let name = "sport+";
/// assert!(Topic::validate_sub_topic(name).is_err());
/// ```
pub fn validate_sub_topic(topic: &str) -> Result<(), TopicError> {
    if topic.is_empty() {
        return Err(TopicError::EmptyTopic);
    }
    if topic == "#" {
        return Ok(());
    }
    let bytes = topic.as_bytes();
    for (index, b) in bytes.iter().enumerate() {
        if b == &b'#' {
            // Must have a prefix level separator.
            if index > 0 && bytes[index - 1] != b'/' {
                return Err(TopicError::InvalidChar);
            }

            // Must be the last wildcard.
            if index != bytes.len() - 1 {
                return Err(TopicError::InvalidChar);
            }
        } else if b == &b'+' {
            // Must have a prefix level separator.
            if index > 0 && bytes[index - 1] != b'/' {
                return Err(TopicError::InvalidChar);
            }
        }
    }

    Ok(())
}

/// Check whether topic name contains wildchard characters or not.
/// ```
/// use codec::Topic;
/// let name = "sport/tennis/player/#";
/// assert!(Topic::validate_pub_topic(name).is_err());
///
/// let name = "sport/tennis/player/ranking";
/// assert!(Topic::validate_pub_topic(name).is_ok());
/// ```
pub fn validate_pub_topic(topic: &str) -> Result<(), TopicError> {
    if topic.is_empty() {
        return Err(TopicError::EmptyTopic);
    }
    if topic.len() > u16::MAX as usize {
        return Err(TopicError::TooManyData);
    }

    if topic.as_bytes().iter().find(|c| c == &&b'+' || c == &&b'#') == None {
        Ok(())
    } else {
        Err(TopicError::InvalidChar)
    }
}

// TODO(Shaohua): Impl internal reference to `topic` String.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TopicPart {
    /// Special internal part, like `$SYS`.
    /// Topics start will `$` char will be traited as internal topic, even so
    /// only `$SYS` is used currently.
    Internal(String),

    /// Normal part.
    Normal(String),

    /// Empty part.
    Empty,

    /// `#` char, to match any remaining parts.
    MultiWildcard,

    /// `+` char, to match right part.
    SingleWildcard,
}

impl TopicPart {
    fn has_wildcard(s: &str) -> bool {
        s.contains(|c| c == '#' || c == '+')
    }

    fn is_internal(s: &str) -> bool {
        s.starts_with('$')
    }

    fn parse(s: &str) -> Result<Self, TopicError> {
        match s {
            "" => Ok(TopicPart::Empty),
            "+" => Ok(TopicPart::SingleWildcard),
            "#" => Ok(TopicPart::MultiWildcard),
            _ => {
                if TopicPart::has_wildcard(s) {
                    Err(TopicError::ContainsWildChar)
                } else if TopicPart::is_internal(s) {
                    Ok(TopicPart::Internal(s.to_string()))
                } else {
                    Ok(TopicPart::Normal(s.to_string()))
                }
            }
        }
    }
}

impl Default for TopicPart {
    fn default() -> Self {
        TopicPart::Empty
    }
}

/// Topic/QoS pair.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubscribePattern {
    /// Subscribed `topic` contains wildcard characters to match interested topics with patterns.
    topic: Topic,

    /// Maximum level of QoS of packet the Server can send to the Client.
    qos: QoS,
}

impl SubscribePattern {
    pub fn parse(topic: &str, qos: QoS) -> Result<Self, TopicError> {
        let topic = Topic::parse(topic)?;
        Ok(Self { topic, qos })
    }

    pub fn new(topic: Topic, qos: QoS) -> Self {
        Self { topic, qos }
    }

    pub fn topic(&self) -> &Topic {
        &self.topic
    }

    pub fn qos(&self) -> QoS {
        self.qos
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PubTopic(String);

impl PubTopic {
    pub fn new(topic: &str) -> Result<Self, TopicError> {
        validate_pub_topic(topic)?;
        Ok(Self(topic.to_string()))
    }
}

impl AsRef<str> for PubTopic {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl DecodePacket for PubTopic {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let len = ba.read_u16()?;
        let s = ba.read_string(len as usize)?;
        validate_pub_topic(&s);
        Ok(Self(s))
    }
}

impl EncodePacket for PubTopic {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.write_u16::<BigEndian>(self.0.len() as u16)?;
        buf.write_all(self.0.as_bytes())?;
        Ok(2 + self.0.len())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubTopic(String);

impl SubTopic {
    pub fn new(topic: &str) -> Result<Self, TopicError> {
        validate_sub_topic(topic)?;
        Ok(Self(topic.to_string()))
    }
}

impl AsRef<str> for SubTopic {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl DecodePacket for SubTopic {
    fn decode(ba: &mut ByteArray) -> Result<Self, DecodeError> {
        let len = ba.read_u16()?;
        let s = ba.read_string(len as usize)?;
        validate_sub_topic(&s);
        Ok(Self(s))
    }
}

impl EncodePacket for SubTopic {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        buf.write_u16::<BigEndian>(self.0.len() as u16)?;
        buf.write_all(self.0.as_bytes())?;
        Ok(2 + self.0.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let t_sys = Topic::parse("$SYS/uptime");
        assert!(t_sys.is_ok());
    }

    #[test]
    fn test_topic_match() {
        let t_sys = Topic::parse("$SYS");
        assert!(t_sys.is_ok());
        let t_sys = t_sys.unwrap();

        let t_any = Topic::parse("#").unwrap();
        // FIXME(Shaohua):
        //assert!(t_any.is_match(t_sys.str()));

        let t_dev = Topic::parse("dev/#").unwrap();
        assert!(t_dev.is_match("dev/cpu/0"));
    }
}
