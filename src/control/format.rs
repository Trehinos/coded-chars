//! These control functions change the format.

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;

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

/// # TBC - Tabulation clear
///
/// TBC causes one or more tabulation stops in the presentation component to be cleared.
pub fn clear_tabulation(tabulation_control: TabulationControl) -> ControlSequence {
    ControlSequence::new(&[&tabulation_control.to_string()], "g")
}

#[derive(Copy, Clone, Debug)]
pub enum TabulationControl {
    /// A character tabulation stop is set at the active presentation position.
    Character,
    /// A line tabulation stop is set at the active line (the line that contains the active presentation position).
    Line,
    /// The character tabulation stop at the active presentation position is cleared.
    CharacterRemove,
    /// The line tabulation stop at the active line is cleared.
    LineRemove,
    /// All character tabulation stops in the active line are cleared.
    CharacterClearLine,
    /// All character tabulation stops are cleared.
    CharacterClearAll,
    /// All line tabulation stops are cleared.
    LineClearAll,
}

impl Display for TabulationControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TabulationControl::Character => "0",
            TabulationControl::Line => "1",
            TabulationControl::CharacterRemove => "2",
            TabulationControl::LineRemove => "3",
            TabulationControl::CharacterClearLine => "4",
            TabulationControl::CharacterClearAll => "5",
            TabulationControl::LineClearAll => "6",
        })
    }
}

/// # TSR - Tabulation stop remove
///
/// TSR causes any character tabulation stop at character position n in the active line (the line that contains
/// the active presentation position) and lines of subsequent text in the presentation component to be
/// cleared, but does not affect other tabulation stops.
pub fn remove_tabulation_stop(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " d")
}

/// # VPA - Line position absolute
///
/// VPA causes the active data position to be moved to line position n in the data component in a direction
/// parallel to the line progression.
pub fn line_position(n: usize) -> ControlSequence { ControlSequence::new(&[&n.to_string()], "d") }

/// # VPB - Line position backward
///
/// VPB causes the active data position to be moved by n line positions in the data component in a direction
/// opposite to that of the line progression.
pub fn line_backward(n: usize) -> ControlSequence { ControlSequence::new(&[&n.to_string()], "k") }

/// # VPR - Line position forward
///
/// VPR causes the active data position to be moved by n line positions in the data component in a direction
/// parallel to the line progression.
pub fn line_forward(n: usize) -> ControlSequence { ControlSequence::new(&[&n.to_string()], "e") }