use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;

pub enum JustifyMode {
    /// No justification, end of justification of preceding text.
    None,
    WordFill,
    WordSpace,
    LetterSpace,
    Hyphen,
    /// Flush to line home position margin.
    FlushHome,
    /// Centre between line home position and line limit position margins.
    Center,
    /// Flush to line limit position margin.
    FlushLimit,
    ItalianHyphen,
}

impl Display for JustifyMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            JustifyMode::None => "0",
            JustifyMode::WordFill => "1",
            JustifyMode::WordSpace => "2",
            JustifyMode::LetterSpace => "3",
            JustifyMode::Hyphen => "4",
            JustifyMode::FlushHome => "5",
            JustifyMode::Center => "6",
            JustifyMode::FlushLimit => "7",
            JustifyMode::ItalianHyphen => "8",
        })
    }
}

/// # JFY - Justify
///
/// JFY is used to indicate the beginning of a string of graphic characters in the presentation component that
/// are to be justified according to the layout specified by the parameter values.
///
/// The end of the string to be justified is indicated by the next occurrence of JFY in the data stream.
pub fn justify(modes: &[JustifyMode]) -> ControlSequence {
    let str_modes: Vec<String> = modes.iter()
        .map(|mode| mode.to_string())
        .collect() ;

    let str_ref_modes: Vec<&str> = str_modes.iter()
        .map(AsRef::as_ref)
        .collect();

    ControlSequence::new(&str_ref_modes, " F")
}


/// # SPH - Set page home
///
/// If the DEVICE COMPONENT SELECT MODE is set to PRESENTATION, SPH is used to establish at
/// line position n in the active page (the page that contains the active presentation position) and subsequent
/// pages in the presentation component the position to which the active presentation position will be moved
/// by subsequent occurrences of FORM FEED (FF) in the data stream; where n equals the value of Pn. In
/// the case of a device without data component, it is also the position ahead of which no implicit movement
/// of the active presentation position shall occur.
///
/// If the DEVICE COMPONENT SELECT MODE is set to DATA, SPH is used to establish at line position
/// n in the active page (the page that contains the active data position) and subsequent pages in the data
/// component the position to which the active data position will be moved by subsequent occurrences of
/// FORM FEED (FF) in the data stream; where n equals the value of Pn. It is also the position ahead of
/// which no implicit movement of the active presentation position shall occur.
///
/// The established position is called the page home position and remains in effect until the next occurrence
/// of SPH in the data stream.
pub fn set_page_home(c: usize) -> ControlSequence {
    ControlSequence::new(&[&c.to_string()], " i")
}