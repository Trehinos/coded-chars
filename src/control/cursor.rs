//! This module helps move and set the cursor position.

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;

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