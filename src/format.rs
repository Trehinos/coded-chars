//! These control functions change the format.

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;
use crate::escape::{escape, EscapeSequence};

/// # Backspace
///
/// BS causes the active data position to be moved one character position in the data component in the
/// direction opposite to that of the implicit movement.
///
/// The direction of the implicit movement depends on the parameter value of SELECT IMPLICIT
/// MOVEMENT DIRECTION (SIMD).
pub const BS: char = '\x08';

/// # Horizontal tabulation
///
/// HT causes the active presentation position to be moved to the following character tabulation stop in the
/// presentation component.
///
/// In addition, if that following character tabulation stop has been set by TABULATION ALIGN CENTRE
/// (TAC), TABULATION ALIGN LEADING EDGE (TALE), TABULATION ALIGN TRAILING EDGE
/// (TATE) or TABULATION CENTRED ON CHARACTER (TCC), HT indicates the beginning of a string
/// of text which is to be positioned within a line according to the properties of that tabulation stop. The end
/// of the string is indicated by the next occurrence of HT or CARRIAGE RETURN (CR) or NEXT LINE
/// (NEL) in the data stream.
pub const HT: char = '\x09';

/// # Line feed
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, LF causes the
/// active presentation position to be moved to the corresponding character position of the following line in
/// the presentation component.
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, LF causes the active data
/// position to be moved to the corresponding character position of the following line in the data
/// component
pub const LF: char = '\x0A';

/// # Vertical tabulation
///
/// VT causes the active presentation position to be moved in the presentation component to the
/// corresponding character position on the line at which the following line tabulation stop is set.
pub const VT: char = '\x0B';

/// # Form feed
///
/// FF causes the active presentation position to be moved to the corresponding character position of the
/// line at the page home position of the next form or page in the presentation component. The page home
/// position is established by the parameter value of SET PAGE HOME (SPH).
pub const FF: char = '\x0C';

/// # Carriage return
///
/// The effect of CR depends on the setting of the DEVICE COMPONENT SELECT MODE (DCSM) and
/// on the parameter value of SELECT IMPLICIT MOVEMENT DIRECTION (SIMD).
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION and with the
/// parameter value of SIMD equal to 0, CR causes the active presentation position to be moved to the line
/// home position of the same line in the presentation component. The line home position is established by
/// the parameter value of SET LINE HOME (SLH).
///
/// With a parameter value of SIMD equal to 1, CR causes the active presentation position to be moved to
/// the line limit position of the same line in the presentation component. The line limit position is
/// established by the parameter value of SET LINE LIMIT (SLL).
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA and with a parameter value of
/// SIMD equal to 0, CR causes the active data position to be moved to the line home position of the same
/// line in the data component. The line home position is established by the parameter value of SET LINE
/// HOME (SLH).
///
/// With a parameter value of SIMD equal to 1, CR causes the active data position to be moved to the line
/// limit position of the same line in the data component. The line limit position is established by the
/// parameter value of SET LINE LIMIT (SLL).
pub const CR: char = '\x0D';

/// # Character tabulation with justification
///
/// HTJ causes the contents of the active field (the field in the presentation component that contains the
/// active presentation position) to be shifted forward so that it ends at the character position preceding the
/// following character tabulation stop. The active presentation position is moved to that following character
/// tabulation stop. The character positions which precede the beginning of the shifted string are put into the
/// erased state.
pub const HTJ: EscapeSequence = escape('I');

/// # Character tabulation set
///
/// HTS causes a character tabulation stop to be set at the active presentation position in the presentation component.
///
/// The number of lines affected depends on the setting of the TABULATION STOP MODE (TSM).
pub const HTS: EscapeSequence = escape('H');

/// # Next line
///
/// The effect of NEL depends on the setting of the DEVICE COMPONENT SELECT MODE (DCSM) and
/// on the parameter value of SELECT IMPLICIT MOVEMENT DIRECTION (SIMD).
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION and with a
/// parameter value of SIMD equal to 0, NEL causes the active presentation position to be moved to the line
/// home position of the following line in the presentation component. The line home position is established
/// by the parameter value of SET LINE HOME (SLH).
///
/// With a parameter value of SIMD equal to 1, NEL causes the active presentation position to be moved to
/// the line limit position of the following line in the presentation component. The line limit position is
/// established by the parameter value of SET LINE LIMIT (SLL).
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA and with a parameter value of
/// SIMD equal to 0, NEL causes the active data position to be moved to the line home position of the
/// following line in the data component. The line home position is established by the parameter value of
/// SET LINE HOME (SLH).
///
/// With a parameter value of SIMD equal to 1, NEL causes the active data position to be moved to the line
/// limit position of the following line in the data component. The line limit position is established by the
/// parameter value of SET LINE LIMIT (SLL).
pub const NEL: EscapeSequence = escape('E');

/// # Partial line forward
///
/// PLD causes the active presentation position to be moved in the presentation component to the
/// corresponding position of an imaginary line with a partial offset in the direction of the line progression.
/// This offset should be sufficient either to image following characters as subscripts until the first
/// following occurrence of PARTIAL LINE BACKWARD (PLU) in the data stream, or, if preceding
/// characters were imaged as superscripts, to restore imaging of following characters to the active line (the
/// line that contains the active presentation position).
///
/// Any interactions between PLD and format effectors other than PLU are not defined by this Standard.
pub const PLD: EscapeSequence = escape('K');

/// # Partial line backward
///
/// PLU causes the active presentation position to be moved in the presentation component to the
/// corresponding position of an imaginary line with a partial offset in the direction opposite to that of the
/// line progression. This offset should be sufficient either to image following characters as superscripts
/// until the first following occurrence of PARTIAL LINE FORWARD (PLD) in the data stream, or, if
/// preceding characters were imaged as subscripts, to restore imaging of following characters to the active
/// line (the line that contains the active presentation position).
///
/// Any interactions between PLU and format effectors other than PLD are not defined by this Standard.
pub const PLU: EscapeSequence = escape('L');

/// Reserve line feed
pub const RI: EscapeSequence = escape('M');

/// # Line tabulation set
///
/// VTS causes a line tabulation stop to be set at the active line (the line that contains the active presentation position).
pub const VTS: EscapeSequence = escape('J');


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