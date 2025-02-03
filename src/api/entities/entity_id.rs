#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EntityId(String);

impl FromStr for EntityId {
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
        Ok(EntityId(object_id))
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

use std::{convert::Infallible, fmt, str::FromStr};
