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
pub fn dimension_text(l: usize, c:usize) -> ControlSequence {
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

// todo PFS

// todo DAQ