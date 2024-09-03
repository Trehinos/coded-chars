//! This module provides escape sequences not sorted elsewhere...
//!
//! The [EscapeSequence] struct is [Display]able.

use std::fmt::{Display, Formatter};
use crate::introducers::ESC;

#[derive(Copy, Clone)]
pub struct EscapeSequence(char);

impl EscapeSequence {
    pub const fn new(with: char) -> Self { Self(with) }
}

impl Display for EscapeSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", ESC, self.0)
    }
}

pub const fn escape(c:char) -> EscapeSequence { EscapeSequence::new(c) }

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
