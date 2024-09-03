//! This module helps move and set the cursor position.

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;
use crate::format::TabulationControl;

/// # CTC - Cursor tabulation control
///
/// CTC causes one or more tabulation stops to be set or cleared in the presentation component.
pub fn tabulation_control(tabulation_control: TabulationControl) -> ControlSequence {
    ControlSequence::new(&[&tabulation_control.to_string()], "W")
}

/// # CPR - Active position report
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, CPR is used to
/// report the active presentation position of the sending device as residing in the presentation component at
/// the `l`-th line position according to the line progression and at the `c`-th character.
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, CPR is used to report the
/// active data position of the sending device as residing in the data component at the `l`-th line position
/// according to the line progression and at the `c`-th character position according to the character
/// progression.
///
/// CPR may be solicited by a DEVICE STATUS REPORT (DSR) or be sent unsolicited.
pub fn position_report(l: usize, c: usize) -> ControlSequence {
    ControlSequence::new(&[&l.to_string(), &c.to_string()], "H")
}

/// # CUP - Cursor position
///
/// CUP causes the active presentation position to be moved in the presentation component to the n-th line
/// position according to the line progression and to the m-th character position according to the character
/// path, where n equals the value of `l` and m equals the value of `c`.
pub fn set_position(l: usize, c: usize) -> ControlSequence {
    ControlSequence::new(&[&l.to_string(), &c.to_string()], "H")
}

/// A struct representing the cursor directions.
///
/// To use with the function [move_cursor].
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    /// # CUU - Cursor up
    ///
    /// CUU causes the active presentation position to be moved upwards in the presentation component by n
    /// line positions if the character path is horizontal, or by n character positions if the character path is vertical.
    Up,

    /// # CUD - Cursor down
    ///
    /// CUD causes the active presentation position to be moved downwards in the presentation component by n
    /// line positions if the character path is horizontal, or by n character positions if the character path is  vertical.
    Down,

    /// # CUF - Cursor forward
    ///
    /// CUF causes the active presentation position to be moved rightwards in the presentation component by n
    /// character positions if the character path is horizontal, or by n line positions if the character path is  vertical.
    Forward,

    /// # CUB - Cursor backward
    ///
    /// CUB causes the active presentation position to be moved leftwards in the presentation component by n
    /// character positions if the character path is horizontal, or by n line positions if the character path is vertical.
    Backward,

    /// # CNL - Cursor next line
    ///
    /// CNL causes the active presentation position to be moved to the first character position of the n-th following line in the presentation component.
    NextLine,

    /// # CPL - Cursor preceding line
    ///
    /// CPL causes the active presentation position to be moved to the first character position of the n-th preceding line in the presentation component.
    PreviousLine,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => "A",
            Direction::Down => "B",
            Direction::Forward => "C",
            Direction::Backward => "D",
            Direction::NextLine => "E",
            Direction::PreviousLine => "F",
        })
    }
}

/// # Move cursor
/// Moves the cursor in a direction relative to its current position.
///
/// This function can perform the following cursor control functions :
///
/// - CUU - Cursor up with [Direction::Up],
/// - CUD - Cursor down with [Direction::Down],
/// - CUF - Cursor forward with [Direction::Forward],
/// - CUB - Cursor backward with [Direction::Backward],
/// - CNL - Cursor next line with [Direction::NextLine],
/// - CPL - Cursor preceding line with [Direction::PreviousLine],
pub fn move_cursor(direction: Direction, n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], &direction.to_string())
}

/// # CBT - Cursor backward tabulation
///
/// CBT causes the active presentation position to be moved to the character position corresponding to the
/// `n`-th preceding character tabulation stop in the presentation component, according to the character path.
pub fn tabulation_backward(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "Z")
}

/// # CHT - Cursor forward tabulation
///
/// CHT causes the active presentation position to be moved to the character position corresponding to the
/// `n`-th following character tabulation stop in the presentation component, according to the character path.
pub fn tabulation_forward(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "I")
}

/// # CVT - Cursor line tabulation
///
/// CVT causes the active presentation position to be moved to the corresponding character position of the
/// line corresponding to the `n`-th following line tabulation stop in the presentation component.
pub fn line_tabulation(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "Y")
}
