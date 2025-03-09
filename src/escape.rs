//! This module provides escape sequences not sorted elsewhere...
//!
//! The [EscapeSequence] struct is [Display]able.

use crate::introducers::ESC;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct EscapeSequence(char);

impl EscapeSequence {
    pub const fn new(with: char) -> Self {
        Self(with)
    }
}

impl Display for EscapeSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", ESC, self.0)
    }
}

pub const fn escape(c: char) -> EscapeSequence {
    EscapeSequence::new(c)
}

/// Padding character
pub const PAD: EscapeSequence = escape('@');

/// High octet preset
pub const HOP: EscapeSequence = escape('A');

/// Index
pub const IND: EscapeSequence = escape('D');

/// Private use 1
pub const PU1: EscapeSequence = escape('Q');

/// Private use 2
pub const PU2: EscapeSequence = escape('R');

/// Set transmit state
pub const STS: EscapeSequence = escape('S');

/// Cancel character
pub const CCH: EscapeSequence = escape('T');

/// Message waiting
pub const MW: EscapeSequence = escape('U');

/// Single graphic character introducer
pub const SGC: EscapeSequence = escape('Y');

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_sequence_new() {
        let seq = EscapeSequence::new('A');
        assert_eq!(seq.0, 'A');
    }

    #[test]
    fn test_escape_sequence_display() {
        let seq = EscapeSequence::new('A');
        let formatted = format!("{}", seq);
        assert_eq!(formatted, format!("{}A", ESC));
    }

    #[test]
    fn test_escape_function() {
        let seq = escape('B');
        assert_eq!(seq.0, 'B');
    }

    #[test]
    fn test_constants_display() {
        assert_eq!(format!("{}", PAD), format!("{}@", ESC));
        assert_eq!(format!("{}", HOP), format!("{}A", ESC));
        assert_eq!(format!("{}", IND), format!("{}D", ESC));
        assert_eq!(format!("{}", PU1), format!("{}Q", ESC));
        assert_eq!(format!("{}", PU2), format!("{}R", ESC));
        assert_eq!(format!("{}", STS), format!("{}S", ESC));
        assert_eq!(format!("{}", CCH), format!("{}T", ESC));
        assert_eq!(format!("{}", MW), format!("{}U", ESC));
        assert_eq!(format!("{}", SGC), format!("{}Y", ESC));
    }
}
