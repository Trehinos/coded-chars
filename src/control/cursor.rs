use crate::control::ControlSequence;

/// # CUP - Cursor position
///
/// CUP causes the active presentation position to be moved in the presentation component to the n-th line
/// position according to the line progression and to the m-th character position according to the character
/// path, where n equals the value of `l` and m equals the value of `c`.
pub fn set_position(l: usize, c: usize) -> ControlSequence {
    ControlSequence::new(&[&l.to_string(), &c.to_string()], "H")
}