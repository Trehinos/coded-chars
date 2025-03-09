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


#[cfg(test)]
mod tests {
    use crate::escape::escape;

    #[test]
    fn test_apc() {
        assert_eq!(super::APC, escape('_'));
    }

    #[test]
    fn test_cmd() {
        assert_eq!(super::CMD, escape('d'));
    }

    #[test]
    fn test_dcs() {
        assert_eq!(super::DCS, escape('P'));
    }

    #[test]
    fn test_osc() {
        assert_eq!(super::OSC, escape(']'));
    }

    #[test]
    fn test_pm() {
        assert_eq!(super::PM, escape('^'));
    }

    #[test]
    fn test_sos() {
        assert_eq!(super::SOS, escape('X'));
    }

    #[test]
    fn test_st() {
        assert_eq!(super::ST, escape('\\'));
    }
}