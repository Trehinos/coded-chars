//! This module provides tabulation-related control functions.

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

/// # STAB - Selective tabulation
///
/// STAB causes subsequent text in the presentation component to be aligned according to the position and
/// the properties of a tabulation stop which is selected from a list according to the value of the parameter.
///
/// The use of this control function and means of specifying a list of tabulation stops to be referenced by the
/// control function are specified in other standards, for example ISO 8613-6.
pub fn select_tabulation(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " ^")
}

/// # TAC - Tabulation aligned centred
///
/// TAC causes a character tabulation stop calling for centring to be set at character position n in the active
/// line (the line that contains the active presentation position) and lines of subsequent text in the
/// presentation component, where n equals the value of Pn. TAC causes the replacement of any tabulation
/// stop previously set at that character position, but does not affect other tabulation stops.
///
/// A text string centred upon a tabulation stop set by TAC will be positioned so that the (trailing edge of
/// the) first graphic character and the (leading edge of the) last graphic character are at approximately equal
/// distances from the tabulation stop.
pub fn align_center(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " b")
}

/// # TALE - Tabulation aligned leading edge
///
/// TALE causes a character tabulation stop calling for leading edge alignment to be set at character
/// position n in the active line (the line that contains the active presentation position) and lines of
/// subsequent text in the presentation component, where n equals the value of Pn. TALE causes the
/// replacement of any tabulation stop previously set at that character position, but does not affect other
/// tabulation stops.
///
/// A text string aligned with a tabulation stop set by TALE will be positioned so that the (leading edge of
/// the) last graphic character of the string is placed at the tabulation stop.
pub fn align_leading(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " a")
}

/// # TATE - Tabulation aligned trailing edge
///
/// TATE causes a character tabulation stop calling for trailing edge alignment to be set at character
/// position n in the active line (the line that contains the active presentation position) and lines of
/// subsequent text in the presentation component, where n equals the value of Pn. TATE causes the
/// replacement of any tabulation stop previously set at that character position, but does not affect other
/// tabulation stops.
///
/// A text string aligned with a tabulation stop set by TATE will be positioned so that the (trailing edge of
/// the) first graphic character of the string is placed at the tabulation stop.
pub fn align_trailing(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " `")
}


/// # TBC - Tabulation clear
/// 
/// TBC causes one or more tabulation stops in the presentation component to be cleared.
pub fn clear(tabulation_control: TabulationControl) -> ControlSequence {
    ControlSequence::new(&[&tabulation_control.to_string()], "g")
}

/// # TCC - Tabulation centred on character
/// 
/// TCC causes a character tabulation stop calling for alignment of a target graphic character to be set at
/// character position `l` in the active line (the line that contains the active presentation position) and lines of
/// subsequent text in the presentation component, and the target character
/// about which centring is to be performed is specified by `ascii`. TCC causes the replacement of any
/// tabulation stop previously set at that character position, but does not affect other tabulation stops.
///
/// The positioning of a text string aligned with a tabulation stop set by TCC will be determined by the first
/// occurrence in the string of the target graphic character; that character will be centred upon the tabulation
/// stop. If the target character does not occur within the string, then the trailing edge of the first character
/// of the string will be positioned at the tabulation stop.
///
/// The value of `ascii` indicates the code table position (binary value) of the target character in the currently
/// invoked code. For a 7-bit code, the permissible range of values is 32 to 127; for an 8-bit code, the
/// permissible range of values is 32 to 127 and 160 to 255.
pub fn center_on_char(l: usize, ascii: usize) -> ControlSequence {
    ControlSequence::new(&[&l.to_string(), &ascii.to_string()], " c")
}

/// # TSR - Tabulation stop remove
/// 
/// TSR causes any character tabulation stop at character position n in the active line (the line that contains
/// the active presentation position) and lines of subsequent text in the presentation component to be
/// cleared, but does not affect other tabulation stops.
pub fn remove_stop(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " d")
}