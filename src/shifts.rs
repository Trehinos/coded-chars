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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_si_constant() {
        assert_eq!(SI, '\x0F');
    }

    #[test]
    fn test_ls0_constant() {
        assert_eq!(LS0, SI);
    }

    #[test]
    fn test_so_constant() {
        assert_eq!(SO, '\x0E');
    }

    #[test]
    fn test_ls1_constant() {
        assert_eq!(LS1, SO);
    }

    #[test]
    fn test_ls1r_escape_sequence() {
        assert_eq!(LS1R, escape('~'));
    }

    #[test]
    fn test_ls2_escape_sequence() {
        assert_eq!(LS2, escape('n'));
    }

    #[test]
    fn test_ls2r_escape_sequence() {
        assert_eq!(LS2R, escape('}'));
    }

    #[test]
    fn test_ls3_escape_sequence() {
        assert_eq!(LS3, escape('o'));
    }

    #[test]
    fn test_ls3r_escape_sequence() {
        assert_eq!(LS3R, escape('|'));
    }

    #[test]
    fn test_ss2_escape_sequence() {
        assert_eq!(SS2, escape('N'));
    }

    #[test]
    fn test_ss3_escape_sequence() {
        assert_eq!(SS3, escape('O'));
    }
}
