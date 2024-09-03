//! All 7-bits and 8-bits shifts. 

use crate::escape::{escape, EscapeSequence};

/// # Shift in
///
/// SI is used for code extension purposes. It causes the meanings of the bit combinations following it in the
/// data stream to be changed.  
/// The use of SI is defined in Standard ECMA-35.
///
/// ### Note
///
/// SI is used in 7-bit environments only; in 8-bit environments LOCKING-SHIFT ZERO (LS0) is used
/// instead.
pub const SI: char = '\x0F';
pub const LS0: char = SI;

/// # Shift out
///
/// SO is used for code extension purposes. It causes the meanings of the bit combinations following it in
/// the data stream to be changed.  
/// The use of SO is defined in Standard ECMA-35.
///
/// ### Note
///
/// SO is used in 7-bit environments only; in 8-bit environments LOCKING-SHIFT ONE (LS1) is used
/// instead.
pub const SO: char = '\x0E';
pub const LS1: char = SO;

/// Locking-shift 1R
pub const LS1R: EscapeSequence = escape('~');

/// Locking-shift 2
pub const LS2: EscapeSequence = escape('n');

/// Locking-shift 2R
pub const LS2R: EscapeSequence = escape('}');

/// Locking-shift 3
pub const LS3: EscapeSequence = escape('o');

/// Locking-shift 3R
pub const LS3R: EscapeSequence = escape('|');

/// Single shift 2
pub const SS2: EscapeSequence = escape('N');

/// Single shift 3
pub const SS3: EscapeSequence = escape('O');