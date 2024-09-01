//! This module provides the CSI sequences.

use std::fmt::{Display, Formatter};
use crate::escape;

#[derive(Clone)]
pub struct ControlSequence {
    arguments: Vec<String>,
    end: String,
}

impl ControlSequence {
    pub fn new(from: &[&str], end: &str) -> Self {
        ControlSequence { arguments: from.iter().map(|s| s.to_string()).collect::<Vec<_>>(), end: end.to_string() }
    }
    
    pub fn exec(&self) {
        println!("{}", self);
    }
}

impl Display for ControlSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", escape::CSI, self.arguments.join(";"), self.end)
    }
}

pub mod rendition;
pub mod mode;
pub mod cursor;
pub mod area;
pub mod device {}
pub mod tabulation {}
pub mod view;

/// The page is erased and the cursor position is set to the first line and the first column.
///
/// - The ANSI/ECMA printed function is : `ED(2),CUP(1,1)`
/// - The ANSI/ECMA printed sequence is : `\x1b[2J\x1b[1;1H`
///
/// This function is a shorthand for :
/// ```
/// print!(
///     "{}{}",
///     coded_chars::control::area::erase_in_page(
///         coded_chars::control::area::AreaPosition::Whole
///     ),
///     coded_chars::control::cursor::set_position(1, 1)
/// )
/// ```
pub fn clear_screen() {
    print!("{}{}", area::erase_in_page(area::AreaPosition::Whole), cursor::set_position(1, 1))
}
