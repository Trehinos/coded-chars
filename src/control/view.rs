use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;
use crate::control::rendition::{CharacterPath, PathEffect};

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

    /// # SR - Scroll right
    ///
    /// SR causes the data in the presentation component to be moved by n character positions if the line
    /// orientation is horizontal, or by `n` line positions if the line orientation is vertical, such that the data appear
    /// to move to the right.
    ///
    /// The active presentation position is not affected by this control function.
    Right,

    /// # SU - Scroll up
    /// 
    /// SU causes the data in the presentation component to be moved by n line positions if the line orientation
    /// is horizontal, or by `n` character positions if the line orientation is vertical, such that the data appear to
    /// move up.
    /// 
    /// The active presentation position is not affected by this control function.
    Up,
}

impl Display for ScrollDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ScrollDirection::Down => "T",
            ScrollDirection::Left => " @",
            ScrollDirection::Right => " A",
            ScrollDirection::Up => "S",
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

pub enum LineOrientation {
    Horizontal,
    Vertical,
}

fn spd_ps1(line_orientation: LineOrientation, line_progression: CharacterPath, character_path: CharacterPath) -> usize {
    match line_orientation {
        LineOrientation::Horizontal => {
            match line_progression {
                CharacterPath::LeftToRight => {
                    match character_path {
                        CharacterPath::LeftToRight => 0,
                        CharacterPath::RightToLeft => 3,
                    }
                }
                CharacterPath::RightToLeft => {
                    match character_path {
                        CharacterPath::LeftToRight => 6,
                        CharacterPath::RightToLeft => 5,
                    }
                }
            }
        }
        LineOrientation::Vertical => {
            match line_progression {
                CharacterPath::LeftToRight => {
                    match character_path {
                        CharacterPath::LeftToRight => 2,
                        CharacterPath::RightToLeft => 4,
                    }
                }
                CharacterPath::RightToLeft => {
                    match character_path {
                        CharacterPath::LeftToRight => 1,
                        CharacterPath::RightToLeft => 7,
                    }
                }
            }
        }
    }
}

/// # SPD - Select presentation directions
///
/// SPD is used to select the line orientation, the line progression, and the character path in the presentation
/// component. It is also used to update the content of the presentation component and the content of the
/// data component. This takes effect immediately.
pub fn select_directions(
    line_orientation: LineOrientation,
    line_progression: CharacterPath,
    character_path: CharacterPath,
    path_effect: PathEffect,
) -> ControlSequence {
    ControlSequence::new(&[&spd_ps1(line_orientation, line_progression, character_path).to_string(), &path_effect.to_string()], " S")
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
pub fn page_home(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " i")
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
pub fn page_limit(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " j")
}

/// # SSW - Set space width
/// 
/// SSW is used to establish for subsequent text the character escapement associated with the character
/// SPACE. The established escapement remains in effect until the next occurrence of SSW in the data
/// stream or until it is reset to the default value by a subsequent occurrence of CARRIAGE RETURN/LINE
/// FEED (CR/LF), CARRIAGE RETURN/FORM FEED (CR/FF), or of NEXT LINE (NEL) in the data stream.
///
/// `n` specifies the escapement.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT
/// SIZE UNIT (SSU).
///
/// The default character escapement of SPACE is specified by the most recent occurrence of SET
/// CHARACTER SPACING (SCS) or of SELECT CHARACTER SPACING (SHS) or of SELECT
/// SPACING INCREMENT (SPI) in the data stream if the current font has constant spacing, or is specified
/// by the nominal width of the character SPACE in the current font if that font has proportional spacing.
pub fn space_width(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " [")
}