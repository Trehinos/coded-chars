//! This module helps move and set the cursor position.

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;


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


/// # HPA - Character position absolute
///
/// HPA causes the active data position to be moved to character position `n` in the active line (the line in the
/// data component that contains the active data position).
pub fn character_absolute(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "`")
}

/// # HPB - Character position backward
///
/// HPB causes the active data position to be moved by `n` character positions in the data component in the
/// direction opposite to that of the character progression.
pub fn character_backward(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "j")
}

/// # HPR - Character position forward
///
/// HPR causes the active data position to be moved by `n` character positions in the data component in the
/// direction of the character progression.
pub fn character_forward(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "a")
}

/// # HVP - Character and line position
///
/// HVP causes the active data position to be moved in the data component to the `l`-th line position
/// according to the line progression and to the `c`-th character position according to the character
/// progression.
pub fn character_and_line_position(l: usize, c: usize) -> ControlSequence {
    ControlSequence::new(&[&l.to_string(), &c.to_string()], "f")
}

/// # PPA - Page position absolute
///
/// PPA causes the active data position to be moved in the data component to the corresponding character
/// position on the `n`-th page.
pub fn page_position(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " P")
}

/// # PPB - Page position backward
///
/// PPB causes the active data position to be moved in the data component to the corresponding character
/// position on the `n`-th preceding page.
pub fn page_backward(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " R")
}

/// # PPR - Page position forward
///
/// PPR causes the active data position to be moved in the data component to the corresponding character
/// position on the `n`-th following page.
pub fn page_forward(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " Q")
}

pub enum MovementDirection {
    Same,
    Opposite,
}

impl Display for MovementDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MovementDirection::Same => "0",
            MovementDirection::Opposite => "1",
        })
    }
}

/// # SIMD - Select implicit movement direction
/// 
/// SIMD is used to select the direction of implicit movement of the data position relative to the character
/// progression. The direction selected remains in effect until the next occurrence of SIMD.
pub fn select_implicit(movement_direction: MovementDirection) -> ControlSequence {
    ControlSequence::new(&[&movement_direction.to_string()], "^")
}