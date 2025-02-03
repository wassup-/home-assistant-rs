#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ObjectId(String);

impl FromStr for ObjectId {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut object_id = String::with_capacity(s.len());

        for ch in s.chars() {
            if ch.is_whitespace() || !ch.is_ascii() {
                object_id.push('-');
            } else {
                object_id.push(ch);
            }
        }

        object_id.shrink_to_fit();
        Ok(ObjectId(object_id))
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

use std::{convert::Infallible, fmt, str::FromStr};
