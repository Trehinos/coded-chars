//! This module provides all escape sequences defined in ecma-48.
//!
//! The [EscapeSequence] struct is [Display]able.

use std::fmt::{Display, Formatter};
use crate::characters;

#[derive(Copy, Clone)]
pub struct EscapeSequence(char);

impl EscapeSequence {
    pub const fn new(with: char) -> Self { Self(with) }
}

impl Display for EscapeSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", characters::ESC, self.0)
    }
}

pub const fn escape(c:char) -> EscapeSequence { EscapeSequence::new(c) }

/// Padding character
pub const PAD: EscapeSequence = escape('@');

/// High octet preset
pub const HOP: EscapeSequence = escape('A');

/// # Break permitted here
///
/// BPH is used to indicate a point where a line break may occur when text is formatted. BPH may occur
/// between two graphic characters, either or both of which may be SPACE.
pub const BPH: EscapeSequence = escape('B');

/// # No break here
///
/// NBH is used to indicate a point where a line break shall not occur when text is formatted. NBH may
/// occur between two graphic characters either or both of which may be SPACE.
pub const NBH: EscapeSequence = escape('C');

/// Index
pub const IND: EscapeSequence = escape('D');

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

/// # Character tabulation set
/// 
/// HTS causes a character tabulation stop to be set at the active presentation position in the presentation component.
/// 
/// The number of lines affected depends on the setting of the TABULATION STOP MODE (TSM).
pub const HTS: EscapeSequence = escape('H');

/// # Character tabulation with justification
/// 
/// HTJ causes the contents of the active field (the field in the presentation component that contains the
/// active presentation position) to be shifted forward so that it ends at the character position preceding the
/// following character tabulation stop. The active presentation position is moved to that following character
/// tabulation stop. The character positions which precede the beginning of the shifted string are put into the
/// erased state.
pub const HTJ: EscapeSequence = escape('I');

/// # Line tabulation set
/// 
/// VTS causes a line tabulation stop to be set at the active line (the line that contains the active presentation position).
pub const VTS: EscapeSequence = escape('J');

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

/// Single shift 2
pub const SS2: EscapeSequence = escape('N');

/// Single shift 3
pub const SS3: EscapeSequence = escape('O');

/// Device control string
pub const DCS: EscapeSequence = escape('P');

/// Private use 1
pub const PU1: EscapeSequence = escape('Q');

/// Private use 2
pub const PU2: EscapeSequence = escape('R');

/// Set transmit state
pub const STS: EscapeSequence = escape('S');

/// Cancel character
pub const CCH: EscapeSequence = escape('T');

/// Message waiting
pub const MW: EscapeSequence = escape('U');

/// Start of protected area
pub const SPA: EscapeSequence = escape('V');

/// End of protected area
pub const EPA: EscapeSequence = escape('W');

/// Start of string
pub const SOS: EscapeSequence = escape('X');

/// Single graphic character introducer
pub const SGC: EscapeSequence = escape('Y');

/// Single character introducer
pub const SCI: EscapeSequence = escape('Z');

/// Control sequence identifier
pub const CSI: EscapeSequence = escape('[');

/// String terminator
pub const ST: EscapeSequence = escape('\\');

/// Operating system command
pub const OSC: EscapeSequence = escape(']');

/// Private message
pub const PM: EscapeSequence = escape('^');

/// Application program command
pub const APC: EscapeSequence = escape('_');

/// Disable manual input
pub const DMI: EscapeSequence = escape('`');

/// Interrupt
pub const INT: EscapeSequence = escape('a');

/// Enable manual input
pub const EMI: EscapeSequence = escape('b');

/// Reset to initial state
pub const RIS: EscapeSequence = escape('c');

/// Coding method delimiter
pub const CMD: EscapeSequence = escape('d');

/// Locking-shift 1R
pub const LS1R: EscapeSequence = escape('~');

/// Locking-shift 2
pub const LS2: EscapeSequence = escape('n');

/// Locking-shift 2R
pub const LS2R: EscapeSequence = escape('}');

/// Locking-shift 3
pub const LS3: EscapeSequence = escape('o');

/// Locking-shift 3R
pub const LS3R: EscapeSequence = escape('|');
