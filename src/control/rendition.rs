use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;

/// # ICH - Insert character
/// 
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, ICH is used to
/// prepare the insertion of n characters, by putting into the erased state the active presentation position and,
/// depending on the setting of the CHARACTER EDITING MODE (HEM), the n-1 preceding or following
/// character positions in the presentation component, where n equals the value of Pn. The previous contents
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
/// positions in the data component, where n equals the value of Pn. The previous contents of the active data
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
/// EDITING MODE (VEM), the n-1 preceding or following lines, where n equals the value of Pn. The
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
/// the n-1 preceding or following lines, where n equals the value of Pn. The previous contents of the active
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
/// removed from the presentation component, where n equals the value of Pn. The resulting gap is closed
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
/// component, where n equals the value of Pn. The resulting gap is closed by shifting the contents of the
/// adjacent character positions towards the active data position. At the other end of the shifted part, n
/// character positions are put into the erased state.
pub fn delete_char(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "P")
}

pub fn delete_line(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "M")
}

pub fn erase_char(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "X")
}

pub fn previous_page(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "V")
}

pub fn next_page(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "U")
}
pub fn modify_size(height: usize, width: usize) -> ControlSequence {
    ControlSequence::new(&[&height.to_string(), &width.to_string()], " B")
}
pub fn select_size(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " C")
}

#[derive(Copy, Clone)]
pub enum Expansion {
    Normal,
    Expanded,
    Condensed,
}

impl Display for Expansion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Normal => "0",
            Self::Expanded => "1",
            Self::Condensed => "2"
        })
    }
}

pub fn expand_or_condense(expansion: Expansion) -> ControlSequence {
    ControlSequence::new(&[&expansion.to_string()], " Z")
}

#[derive(Copy, Clone)]
pub enum Combination {
    Two,
    Start,
    End,
}

impl Display for Combination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Two => "0",
            Self::Start => "1",
            Self::End => "2"
        })
    }
}

pub fn character_combination(combination: Combination) -> ControlSequence {
    ControlSequence::new(&[&combination.to_string()], " ^")
}

pub enum Font {
    Primary,
    Alternative1,
    Alternative2,
    Alternative3,
    Alternative4,
    Alternative5,
    Alternative6,
    Alternative7,
    Alternative8,
    Alternative9,
}

impl Display for Font {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Font::Primary => "0",
            Font::Alternative1 => "1",
            Font::Alternative2 => "2",
            Font::Alternative3 => "3",
            Font::Alternative4 => "4",
            Font::Alternative5 => "5",
            Font::Alternative6 => "6",
            Font::Alternative7 => "7",
            Font::Alternative8 => "8",
            Font::Alternative9 => "9"
        })
    }
}

pub fn select_font(font: Font) -> ControlSequence {
    ControlSequence::new(&[&font.to_string()], " D")
}

#[derive(Clone)]
pub struct SelectGraphic {
    modes: Vec<String>,
}
impl SelectGraphic {
    pub fn new() -> Self { Self { modes: vec![] } }
    pub fn default(&mut self) -> &mut Self { self.add("0") }
    pub fn bold(&mut self) -> &mut Self { self.add("1") }
    pub fn faint(&mut self) -> &mut Self { self.add("2") }
    pub fn italic(&mut self) -> &mut Self { self.add("3") }
    pub fn underline(&mut self) -> &mut Self { self.add("4") }
    pub fn slow_blink(&mut self) -> &mut Self { self.add("5") }
    pub fn fast_blink(&mut self) -> &mut Self { self.add("6") }
    pub fn negative(&mut self) -> &mut Self { self.add("7") }
    pub fn conceal(&mut self) -> &mut Self { self.add("8") }
    pub fn cross(&mut self) -> &mut Self { self.add("9") }
    pub fn primary_font(&mut self) -> &mut Self { self.add("10") }
    pub fn alter1_font(&mut self) -> &mut Self { self.add("11") }
    pub fn alter2_font(&mut self) -> &mut Self { self.add("12") }
    pub fn alter3_font(&mut self) -> &mut Self { self.add("13") }
    pub fn alter4_font(&mut self) -> &mut Self { self.add("14") }
    pub fn alter5_font(&mut self) -> &mut Self { self.add("15") }
    pub fn alter6_font(&mut self) -> &mut Self { self.add("16") }
    pub fn alter7_font(&mut self) -> &mut Self { self.add("17") }
    pub fn alter8_font(&mut self) -> &mut Self { self.add("18") }
    pub fn alter9_font(&mut self) -> &mut Self { self.add("19") }
    pub fn gothic_font(&mut self) -> &mut Self { self.add("20") }
    pub fn double_underline(&mut self) -> &mut Self { self.add("21") }
    pub fn not_bold_or_faint(&mut self) -> &mut Self { self.add("22") }
    pub fn not_italic(&mut self) -> &mut Self { self.add("23") }
    pub fn not_underline(&mut self) -> &mut Self { self.add("24") }
    pub fn not_blink(&mut self) -> &mut Self { self.add("25") }
    pub fn not_negative(&mut self) -> &mut Self { self.add("27") }
    pub fn not_conceal(&mut self) -> &mut Self { self.add("28") }
    pub fn not_cross(&mut self) -> &mut Self { self.add("29") }
    pub fn fg_black(&mut self) -> &mut Self { self.add("30") }
    pub fn fg_red(&mut self) -> &mut Self { self.add("31") }
    pub fn fg_green(&mut self) -> &mut Self { self.add("32") }
    pub fn fg_yellow(&mut self) -> &mut Self { self.add("33") }
    pub fn fg_blue(&mut self) -> &mut Self { self.add("34") }
    pub fn fg_magenta(&mut self) -> &mut Self { self.add("35") }
    pub fn fg_cyan(&mut self) -> &mut Self { self.add("36") }
    pub fn fg_gray(&mut self) -> &mut Self { self.add("37") }
    pub fn fg_color(&mut self) -> &mut Self { self.add("38") }
    pub fn fg_default(&mut self) -> &mut Self { self.add("39") }
    pub fn bg_black(&mut self) -> &mut Self { self.add("40") }
    pub fn bg_red(&mut self) -> &mut Self { self.add("41") }
    pub fn bg_green(&mut self) -> &mut Self { self.add("42") }
    pub fn bg_yellow(&mut self) -> &mut Self { self.add("43") }
    pub fn bg_blue(&mut self) -> &mut Self { self.add("44") }
    pub fn bg_magenta(&mut self) -> &mut Self { self.add("45") }
    pub fn bg_cyan(&mut self) -> &mut Self { self.add("46") }
    pub fn bg_gray(&mut self) -> &mut Self { self.add("47") }
    pub fn bg_color(&mut self) -> &mut Self { self.add("48") }
    pub fn bg_default(&mut self) -> &mut Self { self.add("49") }
    pub fn frame(&mut self) -> &mut Self { self.add("51") }
    pub fn encircle(&mut self) -> &mut Self { self.add("52") }
    pub fn overline(&mut self) -> &mut Self { self.add("53") }
    pub fn not_frame_not_encircle(&mut self) -> &mut Self { self.add("54") }
    pub fn not_overline(&mut self) -> &mut Self { self.add("55") }
    pub fn ideogram_underline(&mut self) -> &mut Self { self.add("60") }
    pub fn ideogram_double_underline(&mut self) -> &mut Self { self.add("61") }
    pub fn ideogram_overline(&mut self) -> &mut Self { self.add("62") }
    pub fn ideogram_double_overline(&mut self) -> &mut Self { self.add("63") }
    pub fn ideogram_stress_marking(&mut self) -> &mut Self { self.add("64") }
    pub fn ideogram_cancel(&mut self) -> &mut Self { self.add("65") }
    pub fn get(&self) -> ControlSequence {
        ControlSequence::new(&self.modes.iter().map(|s| s.as_str()).collect::<Vec<_>>(), "m")
    }
    fn add(&mut self, s: &str) -> &mut Self {
        self.modes.push(s.to_string());
        self
    }
}

pub fn select() -> SelectGraphic {
    SelectGraphic::new()
}

impl Display for SelectGraphic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

pub fn format_str(str: &str, format: &SelectGraphic) -> String {
    format!("{}{}{}", format, str, select().default())
}