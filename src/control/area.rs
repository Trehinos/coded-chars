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

pub fn dimension_text(l: usize, c:usize) -> ControlSequence {
    ControlSequence::new(&[&l.to_string(), &c.to_string()], " T")
}

pub fn erase(area_position: AreaPosition) -> ControlSequence {
    ControlSequence::new(&[&area_position.to_string()], "O")
}
pub fn erase_in_page(area_position: AreaPosition) -> ControlSequence {
    ControlSequence::new(&[&area_position.to_string()], "J")
}
pub fn erase_in_line(area_position: AreaPosition) -> ControlSequence {
    ControlSequence::new(&[&area_position.to_string()], "K")
}
pub fn erase_in_field(area_position: AreaPosition) -> ControlSequence {
    ControlSequence::new(&[&area_position.to_string()], "N")
}

// todo PFS

// todo DAQ