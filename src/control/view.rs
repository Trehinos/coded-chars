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
        .collect();

    let str_ref_modes: Vec<&str> = str_modes.iter()
        .map(AsRef::as_ref)
        .collect();

    ControlSequence::new(&str_ref_modes, " F")
}

pub enum ScrollDirection {
    /// # SD - Scroll down
    ///
    /// SD causes the data in the presentation component to be moved by n line positions if the line orientation
    /// is horizontal, or by `n` character positions if the line orientation is vertical, such that the data appear to
    /// move down.
    ///
    /// The active presentation position is not affected by this control function.
    Down,

    /// # SL - Scroll left
    ///
    /// SL causes the data in the presentation component to be moved by n character positions if the line
    /// orientation is horizontal, or by `n` line positions if the line orientation is vertical, such that the data appear
    /// to move to the left.
    ///
    /// The active presentation position is not affected by this control function.
    Left,
}

impl Display for ScrollDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ScrollDirection::Down => "T",
            ScrollDirection::Left => "@",
        })
    }
}

/// Use this function to call the control functions `SD`, `SL`, `ST` and `SR`.
pub fn scroll(n: usize, scroll_direction: ScrollDirection) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], &scroll_direction.to_string())
}

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

/// # SEE - Select editing extent
///
/// SEE is used to establish the editing extent for subsequent character or line insertion or deletion. The
/// established extent remains in effect until the next occurrence of SEE in the data stream.
pub fn select_extent(editing_extent: EditingExtent) -> ControlSequence {
    ControlSequence::new(&[&editing_extent.to_string()], "Q")
}

/// # SLH - Set line home
///
/// If the DEVICE COMPONENT SELECT MODE is set to PRESENTATION, SLH is used to establish at
/// character position n in the active line (the line that contains the active presentation position) and lines of
/// subsequent text in the presentation component the position to which the active presentation position will
/// be moved by subsequent occurrences of CARRIAGE RETURN (CR), DELETE LINE (DL), INSERT
/// LINE (IL) or NEXT LINE (NEL) in the data stream. In the case of a
/// device without data component, it is also the position ahead of which no implicit movement of the active
/// presentation position shall occur.
///
/// If the DEVICE COMPONENT SELECT MODE is set to DATA, SLH is used to establish at character
/// position n in the active line (the line that contains the active data position) and lines of subsequent text
/// in the data component the position to which the active data position will be moved by subsequent
/// occurrences of CARRIAGE RETURN (CR), DELETE LINE (DL), INSERT LINE (IL) or NEXT LINE
/// (NEL) in the data stream. It is also the position ahead of which no
/// implicit movement of the active data position shall occur.
///
/// The established position is called the line home position and remains in effect until the next occurrence
/// of SLH in the data stream.
pub fn line_home(c: usize) -> ControlSequence {
    ControlSequence::new(&[&c.to_string()], " U")
}

/// # SLL - Set line limit
///
/// If the DEVICE COMPONENT SELECT MODE is set to PRESENTATION, SLL is used to establish at
/// character position n in the active line (the line that contains the active presentation position) and lines of
/// subsequent text in the presentation component the position to which the active presentation position will
/// be moved by subsequent occurrences of CARRIAGE RETURN (CR), or NEXT LINE (NEL) in the data
/// stream if the parameter value of SELECT IMPLICIT MOVEMENT DIRECTION (SIMD) is equal to 1;
/// where n equals the value of Pn. In the case of a device without data component, it is also the position
/// beyond which no implicit movement of the active presentation position shall occur.
///
/// If the DEVICE COMPONENT SELECT MODE is set to DATA, SLL is used to establish at character
/// position n in the active line (the line that contains the active data position) and lines of subsequent text
/// in the data component the position beyond which no implicit movement of the active data position shall
/// occur. It is also the position in the data component to which the active data position will be moved by
/// subsequent occurrences of CR or NEL in the data stream, if the parameter value of SELECT IMPLICIT
/// MOVEMENT DIRECTION (SIMD) is equal to 1.
///
/// The established position is called the line limit position and remains in effect until the next occurrence
/// of SLL in the data stream.
pub fn line_limit(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " V")
}

/// # SLS - Set line spacing
///
/// SLS is used to establish the line spacing for subsequent text. The established spacing remains in effect
/// until the next occurrence of SLS or of SELECT LINE SPACING (SVS) or of SPACING INCREMENT
/// (SPI) in the data stream.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT
/// SIZE UNIT (SSU).
pub fn line_spacing(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " h")
}

/// # SPH - Set page home
///
/// If the DEVICE COMPONENT SELECT MODE is set to PRESENTATION, SPH is used to establish at
/// line position n in the active page (the page that contains the active presentation position) and subsequent
/// pages in the presentation component the position to which the active presentation position will be moved
/// by subsequent occurrences of FORM FEED (FF) in the data stream In
/// the case of a device without data component, it is also the position ahead of which no implicit movement
/// of the active presentation position shall occur.
///
/// If the DEVICE COMPONENT SELECT MODE is set to DATA, SPH is used to establish at line position
/// `n` in the active page (the page that contains the active data position) and subsequent pages in the data
/// component the position to which the active data position will be moved by subsequent occurrences of
/// FORM FEED (FF) in the data stream. It is also the position ahead of
/// which no implicit movement of the active presentation position shall occur.
///
/// The established position is called the page home position and remains in effect until the next occurrence
/// of SPH in the data stream.
pub fn page_home(c: usize) -> ControlSequence {
    ControlSequence::new(&[&c.to_string()], " i")
}

/// # SPL - Set page limit
///
/// If the DEVICE COMPONENT SELECT MODE is set to PRESENTATION, SPL is used to establish at
/// line position n in the active page (the page that contains the active presentation position) and pages of
/// subsequent text in the presentation component the position beyond which the active presentation position
/// can normally not be moved In the case of a device without data
/// component, it is also the position beyond which no implicit movement of the active presentation position
/// shall occur.
///
/// If the DEVICE COMPONENT SELECT MODE is set to DATA, SPL is used to establish at line position
/// n in the active page (the page that contains the active data position) and pages of subsequent text in the
/// data component the position beyond which no implicit movement of the active data position shall occur.
///
/// The established position is called the page limit position and remains in effect until the next occurrence
/// of SPL in the data stream.
pub fn page_limit(c: usize) -> ControlSequence {
    ControlSequence::new(&[&c.to_string()], " j")
}