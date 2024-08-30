use std::fmt::{Display, Formatter};
use crate::escape;

#[derive(Clone)]
pub struct ControlSequence {
    arguments: Vec<String>,
    end: String,
}

impl ControlSequence {
    pub fn new(from: &[&str], end: &str) -> Self {
        ControlSequence { arguments: from.iter().map(|s| s.to_string()).collect::<Vec<_>>(), end: end.to_string() }
    }
}

impl Display for ControlSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", escape::CSI, self.arguments.join(";"), self.end)
    }
}

pub mod rendition;
pub mod mode;
pub mod cursor;
pub mod area;
pub mod device {}
pub mod tabulation {}
pub mod view {}