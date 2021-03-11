// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Default, Clone)]
pub struct Topic {
    pub topic: String,
    pub parts: Vec<TopicPart>,
}

pub enum TopicError {
    EncodeError,
}

impl Topic {
    pub fn parse(s: &str) -> Result<Topic, TopicError> {
        Ok(Topic {
            topic: s.to_string(),
            parts: vec![],
        })
    }

    pub fn is_match(&self, s: &str) -> bool {
        return false;
    }
}

#[derive(Debug, Clone)]
pub enum TopicPart {
    /// Special internal part, like `$SYS`.
    /// Topics start will `$` char will be traited as internal topic, even so
    /// only `$SYS` is used currently.
    Internal(String),

    /// Normal part.
    Str(String),

    /// Empty part.
    Empty,

    /// `*` char, to match any remaining parts.
    StarWildcard,

    /// `+` char, to match right part.
    PlusWildcard,
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
    fn test_topic_match() {}
}
