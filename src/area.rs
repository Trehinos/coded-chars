//! Qualify and delimit areas.

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;
use crate::escape::{escape, EscapeSequence};

/// # DAQ - Define area qualification
///
/// DAQ is used to indicate that the active presentation position in the presentation component is the first
/// character position of a qualified area. The last character position of the qualified area is the character
/// position in the presentation component immediately preceding the first character position of the
/// following qualified area.
///
/// This control function operates independently of the setting of the TABULATION STOP MODE (TSM).
/// The character tabulation stop set by parameter value 7 applies to the active line only
///
/// ### Note
/// The control functions for area definition (DAQ, EPA, ESA, SPA, SSA) should not be used within an SRS
/// string or an SDS string.
pub fn area_qualification(qualification: Qualification) -> ControlSequence {
    ControlSequence::new(&[&qualification.to_string()], "o")
}

#[derive(Copy, Clone, Debug)]
pub enum Qualification {
    UnprotectNoGuard,
    ProtectGuard,
    Character,
    Numeric,
    Alphabet,
    AlignLast,
    FillZero,
    SetTabStop,
    Protect,
    FillSpace,
    AlignFirst,
    Reverse,
}

impl Display for Qualification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Qualification::UnprotectNoGuard => "0",
            Qualification::ProtectGuard => "1",
            Qualification::Character => "2",
            Qualification::Numeric => "3",
            Qualification::Alphabet => "4",
            Qualification::AlignLast => "5",
            Qualification::FillZero => "6",
            Qualification::SetTabStop => "7",
            Qualification::Protect => "8",
            Qualification::FillSpace => "9",
            Qualification::AlignFirst => "10",
            Qualification::Reverse => "11",
        })
    }
}

/// # Start of selected area
///
/// SSA is used to indicate that the active presentation position is the first of a string of character positions
/// in the presentation component, the contents of which are eligible to be transmitted in the form of a data
/// stream or transferred to an auxiliary input/output device.
///
/// The end of this string is indicated by END OF SELECTED AREA (ESA). The string of characters
/// actually transmitted or transferred depends on the setting of the GUARDED AREA TRANSFER MODE
/// (GATM) and on any guarded areas established by DEFINE AREA QUALIFICATION (DAQ), or by
/// START OF GUARDED AREA (SPA) and END OF GUARDED AREA (EPA).
///
/// ### Note
///
/// The control functions for area definition (DAQ, EPA, ESA, SPA, SSA) should not be used within an SRS
/// string or an SDS string.
pub const SSA: EscapeSequence = escape('F');

/// # End of selected area
///
/// ESA is used to indicate that the active presentation position is the last of a string of character positions
/// in the presentation component, the contents of which are eligible to be transmitted in the form of a data
/// stream or transferred to an auxiliary input/output device. The beginning of this string is indicated by
/// START OF SELECTED AREA (SSA).
///
/// ### Note
///
/// The control function for area definition (DAQ, EPA, ESA, SPA, SSA) should not be used within an SRS
/// string or an SDS string.
pub const ESA: EscapeSequence = escape('G');

/// Start of protected area
pub const SPA: EscapeSequence = escape('V');

/// End of protected area
pub const EPA: EscapeSequence = escape('W');
