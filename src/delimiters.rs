//! Various delimiters.

use crate::escape::{escape, EscapeSequence};

/// Application program command
pub const APC: EscapeSequence = escape('_');

/// Coding method delimiter
pub const CMD: EscapeSequence = escape('d');

/// Device control string
pub const DCS: EscapeSequence = escape('P');

/// Operating system command
pub const OSC: EscapeSequence = escape(']');

/// Private message
pub const PM: EscapeSequence = escape('^');

/// Start of string
pub const SOS: EscapeSequence = escape('X');

/// String terminator
pub const ST: EscapeSequence = escape('\\');