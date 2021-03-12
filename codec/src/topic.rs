// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Default, Clone)]
pub struct Topic {
    topic: String,
    parts: Vec<TopicPart>,
}

#[derive(Debug)]
pub enum TopicError {
    DecodeError,
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
                _ => {}
            }
        }
        return true;
    }

    pub fn str(&self) -> &str {
        &self.topic
    }
}

// TODO(Shaohua): Impl internal reference to `topic` String.
#[derive(Debug, Clone)]
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
                    return Err(TopicError::DecodeError);
                } else if TopicPart::is_internal(s) {
                    return Ok(TopicPart::Internal(s.to_string()));
                } else {
                    return Ok(TopicPart::Normal(s.to_string()));
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
        assert!(t_any.is_match(t_sys.str()));
    }
}
