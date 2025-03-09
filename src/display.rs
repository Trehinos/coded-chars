//! This module provides control function that change the display.

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;

/// # PP - Preceding page
///
/// PP causes the n-th preceding page in the presentation component to be displayed, where n equals the
/// value of `n`. The effect of this control function on the active presentation position is not defined by this
/// Standard.
pub fn previous_page(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "V")
}

/// # NP - Next page
///
/// NP causes the n-th following page in the presentation component to be displayed, where n equals the
/// value of `n`. The effect of this control function on the active presentation position is not defined by this Standard.
pub fn next_page(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "U")
}


/// Use this function to call the control functions `SD`, `SL`, `ST` and `SR`.
pub fn scroll(n: usize, scroll_direction: ScrollDirection) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], &scroll_direction.to_string())
}

#[derive(Copy, Clone, Debug)]
pub enum ScrollDirection {
    /// # SD - Scroll down
    ///
    /// SD causes the data in the presentation component to be moved by n line positions if the line orientation
    /// is horizontal, or by `n` character positions if the line orientation is vertical, such that the data appear to
    /// move down.
    ///
    /// The active presentation position is not affected by this control function.
    Down,

    /// # SL - Scroll left
    ///
    /// SL causes the data in the presentation component to be moved by n character positions if the line
    /// orientation is horizontal, or by `n` line positions if the line orientation is vertical, such that the data appear
    /// to move to the left.
    ///
    /// The active presentation position is not affected by this control function.
    Left,

    /// # SR - Scroll right
    ///
    /// SR causes the data in the presentation component to be moved by n character positions if the line
    /// orientation is horizontal, or by `n` line positions if the line orientation is vertical, such that the data appear
    /// to move to the right.
    ///
    /// The active presentation position is not affected by this control function.
    Right,

    /// # SU - Scroll up
    ///
    /// SU causes the data in the presentation component to be moved by n line positions if the line orientation
    /// is horizontal, or by `n` character positions if the line orientation is vertical, such that the data appear to
    /// move up.
    ///
    /// The active presentation position is not affected by this control function.
    Up,
}

impl Display for ScrollDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ScrollDirection::Down => "T",
            ScrollDirection::Left => " @",
            ScrollDirection::Right => " A",
            ScrollDirection::Up => "S",
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::introducers::CSI;
    use super::*;

    #[test]
    fn test_previous_page() {
        let sequence = previous_page(4);
        assert_eq!(sequence.to_string(), format!("{}4V", CSI));
    }

    #[test]
    fn test_next_page() {
        let sequence = next_page(3);
        assert_eq!(sequence.to_string(), format!("{}3U", CSI));
    }

    #[test]
    fn test_scroll_down() {
        let sequence = scroll(5, ScrollDirection::Down);
        assert_eq!(sequence.to_string(), format!("{}5T", CSI));
    }

    #[test]
    fn test_scroll_left() {
        let sequence = scroll(2, ScrollDirection::Left);
        assert_eq!(sequence.to_string(), format!("{}2 @", CSI));
    }

    #[test]
    fn test_scroll_right() {
        let sequence = scroll(7, ScrollDirection::Right);
        assert_eq!(sequence.to_string(), format!("{}7 A", CSI));
    }

    #[test]
    fn test_scroll_up() {
        let sequence = scroll(1, ScrollDirection::Up);
        assert_eq!(sequence.to_string(), format!("{}1S", CSI));
    }

    #[test]
    fn test_scroll_direction_display() {
        assert_eq!(ScrollDirection::Down.to_string(), "T");
        assert_eq!(ScrollDirection::Left.to_string(), " @");
        assert_eq!(ScrollDirection::Right.to_string(), " A");
        assert_eq!(ScrollDirection::Up.to_string(), "S");
    }
}