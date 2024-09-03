//! This module defines the [ControlSequence] struct which represent sequence introduced by **CSI**.

use std::fmt::{Display, Formatter};
use crate::introducers::CSI;

/// A control sequence is a string of bit combinations starting with the control function CONTROL
/// SEQUENCE INTRODUCER (CSI).
///
/// Followed by one or more bit combinations representing parameters, if
/// any, and by one or more bit combinations identifying the control function.
///
/// To "execute" a control sequence you can print it or call the method `exec` :
/// ```
///
/// use coded_chars::control::ControlSequence;
/// let sequence = ControlSequence::new(&["1", "1"], "H");
///
/// print!("{}", sequence); // Prints \x1b[1;1H
/// // or
/// sequence.exec(); // Prints \x1b[1;1H
/// ```
///
/// Almost every function from this crate return [ControlSequence]s :
///
/// ```
/// // This example is equivalent to the above example :
/// use coded_chars::cursor::set_position;
///
/// let sequence = set_position(1, 1); // Returns a ControlSequence
/// sequence.exec(); // Prints \x1b[1;1H
/// ```
#[derive(Clone)]
pub struct ControlSequence {
    arguments: Vec<String>,
    end: String,
}

impl ControlSequence {
    pub fn new(from: &[&str], end: &str) -> Self {
        ControlSequence { arguments: from.iter().map(|s| s.to_string()).collect::<Vec<_>>(), end: end.to_string() }
    }

    /// Prints the current sequence in `stdout` directly.
    pub fn exec(&self) {
        use std::io::stdout;
        use std::io::Write;
        
        print!("{}", self);
        stdout().flush().unwrap()
    }
}

impl Display for ControlSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", CSI, self.arguments.join(";"), self.end)
    }
}