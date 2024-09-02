use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;

/// # CBT - Cursor backward tabulation
///
/// CBT causes the active presentation position to be moved to the character position corresponding to the
/// `n`-th preceding character tabulation stop in the presentation component, according to the character path.
pub fn cursor_backward(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "Z")
}

/// # CBT - Cursor forward tabulation
///
/// CHT causes the active presentation position to be moved to the character position corresponding to the
/// `n`-th following character tabulation stop in the presentation component, according to the character path.
pub fn cursor_forward(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "I")
}

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

/// # CTC - Cursor tabulation control
///
/// CTC causes one or more tabulation stops to be set or cleared in the presentation component.
pub fn cursor_control(tabulation_control: TabulationControl) -> ControlSequence {
    ControlSequence::new(&[&tabulation_control.to_string()], "W")
}

/// # CVT - Cursor line tabulation
///
/// CVT causes the active presentation position to be moved to the corresponding character position of the
/// line corresponding to the `n`-th following line tabulation stop in the presentation component.
pub fn line_tabulation(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "Y")
}