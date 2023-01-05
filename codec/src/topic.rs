// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use byteorder::{BigEndian, WriteBytesExt};
use std::hash::{Hash, Hasher};
use std::io::Write;

use crate::QoS;
use crate::{ByteArray, DecodeError, DecodePacket, EncodeError, EncodePacket};

// TODO(Shaohua): Simplify topic structs.
#[derive(Debug, Default, Clone, Eq, PartialOrd, Ord)]
pub struct Topic {
    topic: String,
    parts: Vec<TopicPart>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq)]
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

impl Hash for Topic {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.topic.hash(state);
    }
}

impl Topic {
    // TODO(Shaohua): Replace with `std::str::FromStr` trait.

    /// Parse topic from string slice.
    ///
    /// # Errors
    ///
    /// Returns error if string contains invalid chars or too large.
    pub fn parse(s: &str) -> Result<Self, TopicError> {
        let parts = Self::parse_parts(s)?;
        Ok(Self {
            topic: s.to_string(),
            parts,
        })
    }

    fn parse_parts(s: &str) -> Result<Vec<TopicPart>, TopicError> {
        s.split('/').map(TopicPart::parse).collect()
    }

    /// Returns true if this topic matches string slice.
    #[must_use]
    pub fn is_match(&self, s: &str) -> bool {
        for (index, part) in s.split('/').into_iter().enumerate() {
            match self.parts.get(index) {
                None | Some(TopicPart::Empty) => return false,
                Some(TopicPart::Normal(ref s_part) | TopicPart::Internal(ref s_part)) => {
                    if s_part != part {
                        return false;
                    }
                }
                Some(TopicPart::SingleWildcard) => {
                    // Continue
                }
                Some(TopicPart::MultiWildcard) => return true,
            }
        }
        true
    }

    /// Used as a string slice.
    #[must_use]
    pub const fn topic(&self) -> &String {
        &self.topic
    }

    /// Get topic length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.topic.len()
    }

    /// Returns true if topic is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.topic.is_empty()
    }

    /// Used as byte slice.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.topic.as_bytes()
    }
}

/// Validate topic filter.
///
/// Rules are defined in `MQTT chapter-4.7 Topic Name and Filters`
///
/// # Errors
///
/// Returns error if topic string contains invalid chars or too large.
///
/// # Examples
///
/// ```
/// use hebo_codec::topic;
/// let name = "sport/tennis/player/#";
/// assert!(topic::validate_sub_topic(name).is_ok());
///
/// let name = "sport/tennis/player#";
/// assert!(topic::validate_sub_topic(name).is_err());
///
/// let name = "#";
/// assert!(topic::validate_sub_topic(name).is_ok());
///
/// let name = "sport/#/player/ranking";
/// assert!(topic::validate_sub_topic(name).is_err());
///
/// let name = "+";
/// assert!(topic::validate_sub_topic(name).is_ok());
///
/// let name = "sport+";
/// assert!(topic::validate_sub_topic(name).is_err());
/// ```
#[allow(clippy::module_name_repetitions)]
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
///
/// # Errors
///
/// Returns error if topic string contains invalid characters or too large.
///
/// # Examples
///
/// ```
/// use hebo_codec::topic;
/// let name = "sport/tennis/player/#";
/// assert!(topic::validate_pub_topic(name).is_err());
///
/// let name = "sport/tennis/player/ranking";
/// assert!(topic::validate_pub_topic(name).is_ok());
/// ```
#[allow(clippy::module_name_repetitions)]
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
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TopicPart {
    /// Special internal part, like `$SYS`.
    /// Topics start will `$` char will be traited as internal topic, even so
    /// only `$SYS` is used currently.
    Internal(String),

    /// Normal part.
    Normal(String),

    /// Empty part.
    #[default]
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

    /// Returns true if topic is used in broker inner only.
    #[must_use]
    fn is_internal(s: &str) -> bool {
        s.starts_with('$')
    }

    /// Parse topic parts.
    ///
    /// # Errors
    ///
    /// Returns error if string slice contains invalid chars.
    fn parse(s: &str) -> Result<Self, TopicError> {
        match s {
            "" => Ok(Self::Empty),
            "+" => Ok(Self::SingleWildcard),
            "#" => Ok(Self::MultiWildcard),
            _ => {
                if Self::has_wildcard(s) {
                    Err(TopicError::ContainsWildChar)
                } else if Self::is_internal(s) {
                    Ok(Self::Internal(s.to_string()))
                } else {
                    Ok(Self::Normal(s.to_string()))
                }
            }
        }
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
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn parse(topic: &str, qos: QoS) -> Result<Self, TopicError> {
        let topic = Topic::parse(topic)?;
        Ok(Self { topic, qos })
    }

    /// Create a new subscription topic pattern.
    #[must_use]
    #[inline]
    pub const fn new(topic: Topic, qos: QoS) -> Self {
        Self { topic, qos }
    }

    /// Get topic value.
    #[must_use]
    #[inline]
    pub const fn topic(&self) -> &Topic {
        &self.topic
    }

    /// Get current `QoS` value.
    #[must_use]
    #[inline]
    pub const fn qos(&self) -> QoS {
        self.qos
    }
}

/// Topic used in publish packet.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PubTopic(String);

impl PubTopic {
    /// Create a new publish topic.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` is invalid.
    pub fn new(topic: &str) -> Result<Self, TopicError> {
        validate_pub_topic(topic)?;
        Ok(Self(topic.to_string()))
    }

    /// Get byte length in packet.
    #[must_use]
    pub fn bytes(&self) -> usize {
        2 + self.0.len()
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
        validate_pub_topic(&s)?;
        Ok(Self(s))
    }
}

impl EncodePacket for PubTopic {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        #[allow(clippy::cast_possible_truncation)]
        let len = self.0.len() as u16;
        buf.write_u16::<BigEndian>(len)?;
        buf.write_all(self.0.as_bytes())?;
        Ok(self.bytes())
    }
}

/// Topic pattern used in subscribe/unsubscribe packet.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubTopic(String);

impl SubTopic {
    /// Create a new subscription topic.
    ///
    /// # Errors
    ///
    /// Returns error if `topic` pattern is invalid.
    pub fn new(topic: &str) -> Result<Self, TopicError> {
        validate_sub_topic(topic)?;
        Ok(Self(topic.to_string()))
    }

    /// Get byte length in packet.
    #[must_use]
    pub fn bytes(&self) -> usize {
        2 + self.0.len()
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
        validate_sub_topic(&s)?;
        Ok(Self(s))
    }
}

impl EncodePacket for SubTopic {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<usize, EncodeError> {
        #[allow(clippy::cast_possible_truncation)]
        let len = self.0.len() as u16;
        buf.write_u16::<BigEndian>(len)?;
        buf.write_all(self.0.as_bytes())?;
        Ok(self.bytes())
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
        //let t_sys = t_sys.unwrap();
        //let t_any = Topic::parse("#").unwrap();
        // FIXME(Shaohua):
        //assert!(t_any.is_match(t_sys.str()));

        let t_dev = Topic::parse("dev/#").unwrap();
        assert!(t_dev.is_match("dev/cpu/0"));
    }
}
