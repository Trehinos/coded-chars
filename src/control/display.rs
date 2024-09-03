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
