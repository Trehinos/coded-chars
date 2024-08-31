

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;

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

/// # DTA - Dimension text area
///
/// DTA is used to establish the dimensions of the text area for subsequent pages.
/// The established dimensions remain in effect until the next occurrence of DTA in the data stream.
///  - `l` specifies the dimension in the direction perpendicular to the line orientation.
///  - `c` specifies the dimension in the direction parallel to the line orientation.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT
/// SIZE UNIT (SSU).
pub fn dimension_text(l: usize, c: usize) -> ControlSequence {
    ControlSequence::new(&[&l.to_string(), &c.to_string()], " T")
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

pub enum PageFormat {
    TallText,
    WideText,
    TallA4,
    WideA4,
    TallLetter,
    WideLetter,
    TallExtA4,
    WideExtA4,
    TallLegal,
    WideLegal,
    A4ShortLines,
    A4LongLines,
    B5ShortLines,
    B5LongLines,
    B4ShortLines,
    B4LongLines,
}

impl Display for PageFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PageFormat::TallText => "0",
            PageFormat::WideText => "1",
            PageFormat::TallA4 => "2",
            PageFormat::WideA4 => "3",
            PageFormat::TallLetter => "4",
            PageFormat::WideLetter => "5",
            PageFormat::TallExtA4 => "6",
            PageFormat::WideExtA4 => "7",
            PageFormat::TallLegal => "8",
            PageFormat::WideLegal => "9",
            PageFormat::A4ShortLines => "10",
            PageFormat::A4LongLines => "11",
            PageFormat::B5ShortLines => "12",
            PageFormat::B5LongLines => "13",
            PageFormat::B4ShortLines => "14",
            PageFormat::B4LongLines => "15",
        })
    }
}

/// # PFS - Page format selection
///
/// PFS is used to establish the available area for the imaging of pages of text based on paper size. The
/// pages are introduced by the subsequent occurrence of FORM FEED (FF) in the data stream.
///
/// The established image area remains in effect until the next occurrence of PFS in the data stream.
///
/// The page home position is established by the parameter value of SET PAGE HOME (SPH), the page
/// limit position is established by the parameter value of SET PAGE LIMIT (SPL).
pub fn select_page_format(page_format: PageFormat) -> ControlSequence {
    ControlSequence::new(&[&page_format.to_string()], " J")
}

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
    Reverse
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
pub fn define_qualification(qualification: Qualification) -> ControlSequence {
    ControlSequence::new(&[&qualification.to_string()], "o")
}