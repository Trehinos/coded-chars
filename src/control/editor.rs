//! Helps edit text.

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;

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

/// # ICH - Insert character
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, ICH is used to
/// prepare the insertion of n characters, by putting into the erased state the active presentation position and,
/// depending on the setting of the CHARACTER EDITING MODE (HEM), the n-1 preceding or following
/// character positions in the presentation component, where n equals the value of `n`. The previous contents
/// of the active presentation position and an adjacent string of character positions are shifted away from the
/// active presentation position. The contents of n character positions at the other end of the shifted part are
/// removed. The active presentation position is moved to the line home position in the active line. The line
/// home position is established by the parameter value of SET LINE HOME (SLH).
///
/// The extent of the shifted part is established by SELECT EDITING EXTENT (SEE).
///
/// The effect of ICH on the start or end of a selected area, the start or end of a qualified area, or a
/// tabulation stop in the shifted part, is not defined by this Standard.
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, ICH is used to prepare the
/// insertion of n characters, by putting into the erased state the active data position and, depending on the
/// setting of the CHARACTER EDITING MODE (HEM), the n-1 preceding or following character
/// positions in the data component, where n equals the value of `n`. The previous contents of the active data
/// position and an adjacent string of character positions are shifted away from the active data position. The
/// contents of n character positions at the other end of the shifted part are removed. The active data
/// position is moved to the line home position in the active line. The line home position is established by
/// the parameter value of SET LINE HOME (SLH).
pub fn insert_char(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "@")
}

/// # IL - Insert line
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, IL is used to
/// prepare the insertion of n lines, by putting into the erased state in the presentation component the active
/// line (the line that contains the active presentation position) and, depending on the setting of the LINE
/// EDITING MODE (VEM), the n-1 preceding or following lines, where n equals the value of `n`. The
/// previous contents of the active line and of adjacent lines are shifted away from the active line. The
/// contents of n lines at the other end of the shifted part are removed. The active presentation position is
/// moved to the line home position in the active line. The line home position is established by the
/// parameter value of SET LINE HOME (SLH).
///
/// The extent of the shifted part is established by SELECT EDITING EXTENT (SEE).
///
/// Any occurrences of the start or end of a selected area, the start or end of a qualified area, or a tabulation
/// stop in the shifted part, are also shifted.
///
/// If the TABULATION STOP MODE (TSM) is set to SINGLE, character tabulation stops are cleared in
/// the lines that are put into the erased state.
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, IL is used to prepare the
/// insertion of n lines, by putting into the erased state in the data component the active line (the line that
/// contains the active data position) and, depending on the setting of the LINE EDITING MODE (VEM),
/// the n-1 preceding or following lines, where n equals the value of `n`. The previous contents of the active
/// line and of adjacent lines are shifted away from the active line. The contents of n lines at the other end
/// of the shifted part are removed. The active data position is moved to the line home position in the active
/// line. The line home position is established by the parameter value of SET LINE HOME (SLH).
pub fn insert_line(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "K")
}

/// # DCH - Delete character
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, DCH causes the
/// contents of the active presentation position and, depending on the setting of the CHARACTER
/// EDITING MODE (HEM), the contents of the n-1 preceding or following character positions to be
/// removed from the presentation component, where n equals the value of `n`. The resulting gap is closed
/// by shifting the contents of the adjacent character positions towards the active presentation position. At
/// the other end of the shifted part, n character positions are put into the erased state.
///
/// The extent of the shifted part is established by SELECT EDITING EXTENT (SEE).
///
/// The effect of DCH on the start or end of a selected area, the start or end of a qualified area, or a
/// tabulation stop in the shifted part is not defined by this Standard.
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, DCH causes the contents of
/// the active data position and, depending on the setting of the CHARACTER EDITING MODE (HEM),
/// the contents of the n-1 preceding or following character positions to be removed from the data
/// component, where n equals the value of `n`. The resulting gap is closed by shifting the contents of the
/// adjacent character positions towards the active data position. At the other end of the shifted part, n
/// character positions are put into the erased state.
pub fn delete_char(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "P")
}


/// # DL - Delete line
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, DL causes the
/// contents of the active line (the line that contains the active presentation position) and, depending on the
/// setting of the LINE EDITING MODE (VEM), the contents of the n-1 preceding or following lines to be
/// removed from the presentation component, where n equals the value of `n`. The resulting gap is closed
/// by shifting the contents of a number of adjacent lines towards the active line. At the other end of the
/// shifted part, n lines are put into the erased state.
///
/// The active presentation position is moved to the line home position in the active line. The line home
/// position is established by the parameter value of SET LINE HOME (SLH). If the TABULATION STOP
/// MODE (TSM) is set to SINGLE, character tabulation stops are cleared in the lines that are put into the
/// erased state.
///
/// The extent of the shifted part is established by SELECT EDITING EXTENT (SEE).
///
/// Any occurrences of the start or end of a selected area, the start or end of a qualified area, or a tabulation
/// stop in the shifted part, are also shifted.
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, DL causes the contents of the
/// active line (the line that contains the active data position) and, depending on the setting of the LINE
/// EDITING MODE (VEM), the contents of the n-1 preceding or following lines to be removed from the
/// data component, where n equals the value of `n`. The resulting gap is closed by shifting the contents of a
/// number of adjacent lines towards the active line. At the other end of the shifted part, n lines are put into
/// the erased state. The active data position is moved to the line home position in the active line. The line
/// home position is established by the parameter value of SET LINE HOME (SLH).
pub fn delete_line(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "M")
}

/// # ECH - Erase character
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, ECH causes the
/// active presentation position and the n-1 following character positions in the presentation component to
/// be put into the erased state, where n equals the value of `n`.
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, ECH causes the active data
/// position and the n-1 following character positions in the data component to be put into the erased state,
/// where n equals the value of `n`.
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions
/// of unprotected areas only, depends on the setting of the ERASURE MODE (ERM).
pub fn erase_char(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "X")
}

/// # EA - Erase in area
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, EA causes some or
/// all character positions in the active qualified area (the qualified area in the presentation component
/// which contains the active presentation position) to be put into the erased state, depending on the
/// parameter values:
/// - 0 the active presentation position and the character positions up to the end of the qualified area are put
/// into the erased state
/// - 1 the character positions from the beginning of the qualified area up to and including the active
/// presentation position are put into the erased state
/// - 2 all character positions of the qualified area are put into the erased state
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, EA causes some or all
/// character positions in the active qualified area (the qualified area in the data component which contains
/// the active data position) to be put into the erased state, depending on the parameter values:
/// - 0 the active data position and the character positions up to the end of the qualified area are put into the
/// erased state
/// - 1 the character positions from the beginning of the qualified area up to and including the active data
/// position are put into the erased state
/// - 2 all character positions of the qualified area are put into the erased state
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions
/// of unprotected areas only, depends on the setting of the ERASURE MODE (ERM).
pub fn erase(area_position: AreaPosition) -> ControlSequence {
    ControlSequence::new(&[&area_position.to_string()], "O")
}

#[derive(Copy, Clone, Debug)]
pub enum AreaPosition {
    AfterCursor,
    BeforeCursor,
    Whole,
}

impl Display for AreaPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::AfterCursor => "0",
            Self::BeforeCursor => "1",
            Self::Whole => "2"
        })
    }
}


/// # ED - Erase in page
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, ED causes some or
/// all character positions of the active page (the page which contains the active presentation position in the
/// presentation component) to be put into the erased state, depending on the parameter values:
/// - 0 the active presentation position and the character positions up to the end of the page are put into the
/// erased state
/// - 1 the character positions from the beginning of the page up to and including the active presentation
/// position are put into the erased state
/// - 2 all character positions of the page are put into the erased state
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, ED causes some or all
/// character positions of the active page (the page which contains the active data position in the data
/// component) to be put into the erased state, depending on the parameter values:
/// - 0 the active data position and the character positions up to the end of the page are put into the erased
/// state
/// - 1 the character positions from the beginning of the page up to and including the active data position are
/// put into the erased state
/// - 2 all character positions of the page are put into the erased state
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions
/// of unprotected areas only, depends on the setting of the ERASURE MODE (ERM).
pub fn erase_in_page(area_position: AreaPosition) -> ControlSequence {
    ControlSequence::new(&[&area_position.to_string()], "J")
}

/// # EF - Erase in field
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, EF causes some or
/// all character positions of the active field (the field which contains the active presentation position in the
/// presentation component) to be put into the erased state, depending on the parameter values:
/// - 0 the active presentation position and the character positions up to the end of the field are put into the
/// erased state
/// - 1 the character positions from the beginning of the field up to and including the active presentation
/// position are put into the erased state
/// - 2 all character positions of the field are put into the erased state
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, EF causes some or all
/// character positions of the active field (the field which contains the active data position in the data
/// component) to be put into the erased state, depending on the parameter values:
/// - 0 the active data position and the character positions up to the end of the field are put into the erased
/// state
/// - 1 the character positions from the beginning of the field up to and including the active data position are
/// put into the erased state
/// - 2 all character positions of the field are put into the erased state
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions
/// of unprotected areas only, depends on the setting of the ERASURE MODE (ERM)
pub fn erase_in_field(area_position: AreaPosition) -> ControlSequence {
    ControlSequence::new(&[&area_position.to_string()], "N")
}

/// # EL - Erase in line
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, EL causes some or
/// all character positions of the active line (the line which contains the active presentation position in the
/// presentation component) to be put into the erased state, depending on the parameter values:
/// - 0 the active presentation position and the character positions up to the end of the line are put into the
/// erased state
/// - 1 the character positions from the beginning of the line up to and including the active presentation
/// position are put into the erased state
/// - 2 all character positions of the line are put into the erased state
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, EL causes some or all
/// character positions of the active line (the line which contains the active data position in the data
/// component) to be put into the erased state, depending on the parameter values:
/// - 0 the active data position and the character positions up to the end of the line are put into the erased
/// state
/// - 1 the character positions from the beginning of the line up to and including the active data position are
/// put into the erased state
/// - 2 all character positions of the line are put into the erased state
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions
/// of unprotected areas only, depends on the setting of the ERASURE MODE (ERM).
pub fn erase_in_line(area_position: AreaPosition) -> ControlSequence {
    ControlSequence::new(&[&area_position.to_string()], "K")
}

/// # SEE - Select editing extent
///
/// SEE is used to establish the editing extent for subsequent character or line insertion or deletion. The
/// established extent remains in effect until the next occurrence of SEE in the data stream.
pub fn select_extent(editing_extent: EditingExtent) -> ControlSequence {
    ControlSequence::new(&[&editing_extent.to_string()], "Q")
}

#[derive(Copy, Clone, Debug)]
pub enum EditingExtent {
    Page,
    Line,
    Field,
    QualifiedArea,
    Relevant,
}

impl Display for EditingExtent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            EditingExtent::Page => "0",
            EditingExtent::Line => "1",
            EditingExtent::Field => "2",
            EditingExtent::QualifiedArea => "3",
            EditingExtent::Relevant => "4",
        })
    }
}