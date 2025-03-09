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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csi_escape_sequence() {
        let expected = EscapeSequence::new('['); // Assuming EscapeSequence has a `new` function
        assert_eq!(CSI, expected, "CSI escape sequence should match the expected value");
    }

    #[test]
    fn test_sci_escape_sequence() {
        let expected = EscapeSequence::new('Z'); // Assuming EscapeSequence has a `new` function
        assert_eq!(SCI, expected, "SCI escape sequence should match the expected value");
    }

    #[test]
    fn test_esc_character() {
        assert_eq!(ESC, '\x1B', "ESC character should be the ASCII Escape character");
    }
}
