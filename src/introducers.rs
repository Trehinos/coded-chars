//! Control sequence and other introducers.

use crate::escape::{escape, EscapeSequence};

/// Control sequence identifier
pub const CSI: EscapeSequence = escape('[');

/// # Escape
///
/// ESC is used for code extension purposes. It causes the meanings of a limited number of bit combinations
/// following it in the data stream to be changed.
///
/// The use of ESC is defined in Standard ECMA-35.
pub const ESC:char = '\x1B';

/// Single character introducer
pub const SCI: EscapeSequence = escape('Z');