//! This module provides all rendition control sequences of ecma-48.

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;

/// # ICH - Insert character
/// 
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, ICH is used to
/// prepare the insertion of n characters, by putting into the erased state the active presentation position and,
/// depending on the setting of the CHARACTER EDITING MODE (HEM), the n-1 preceding or following
/// character positions in the presentation component, where n equals the value of `n`. The previous contents
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
/// positions in the data component, where n equals the value of `n`. The previous contents of the active data
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
/// EDITING MODE (VEM), the n-1 preceding or following lines, where n equals the value of `n`. The
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
/// the n-1 preceding or following lines, where n equals the value of `n`. The previous contents of the active
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
/// removed from the presentation component, where n equals the value of `n`. The resulting gap is closed
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
/// component, where n equals the value of `n`. The resulting gap is closed by shifting the contents of the
/// adjacent character positions towards the active data position. At the other end of the shifted part, n
/// character positions are put into the erased state.
pub fn delete_char(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "P")
}


/// # DL - Delete line
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, DL causes the
/// contents of the active line (the line that contains the active presentation position) and, depending on the
/// setting of the LINE EDITING MODE (VEM), the contents of the n-1 preceding or following lines to be
/// removed from the presentation component, where n equals the value of `n`. The resulting gap is closed
/// by shifting the contents of a number of adjacent lines towards the active line. At the other end of the
/// shifted part, n lines are put into the erased state.
///
/// The active presentation position is moved to the line home position in the active line. The line home
/// position is established by the parameter value of SET LINE HOME (SLH). If the TABULATION STOP
/// MODE (TSM) is set to SINGLE, character tabulation stops are cleared in the lines that are put into the
/// erased state.
///
/// The extent of the shifted part is established by SELECT EDITING EXTENT (SEE).
///
/// Any occurrences of the start or end of a selected area, the start or end of a qualified area, or a tabulation
/// stop in the shifted part, are also shifted.
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, DL causes the contents of the
/// active line (the line that contains the active data position) and, depending on the setting of the LINE
/// EDITING MODE (VEM), the contents of the n-1 preceding or following lines to be removed from the
/// data component, where n equals the value of `n`. The resulting gap is closed by shifting the contents of a
/// number of adjacent lines towards the active line. At the other end of the shifted part, n lines are put into
/// the erased state. The active data position is moved to the line home position in the active line. The line
/// home position is established by the parameter value of SET LINE HOME (SLH).
pub fn delete_line(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "M")
}

/// # ECH - Erase character
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, ECH causes the
/// active presentation position and the n-1 following character positions in the presentation component to
/// be put into the erased state, where n equals the value of `n`.
///
/// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, ECH causes the active data
/// position and the n-1 following character positions in the data component to be put into the erased state,
/// where n equals the value of `n`.
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions
/// of unprotected areas only, depends on the setting of the ERASURE MODE (ERM).
pub fn erase_char(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "X")
}

/// # PP - Preceding page
///
/// PP causes the n-th preceding page in the presentation component to be displayed, where n equals the
/// value of `n`. The effect of this control function on the active presentation position is not defined by this
/// Standard.
pub fn previous_page(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "V")
}

/// # NP - Next page
///
/// NP causes the n-th following page in the presentation component to be displayed, where n equals the
/// value of `n`. The effect of this control function on the active presentation position is not defined by this Standard.
pub fn next_page(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "U")
}

/// # GSM - Graphic size modification
///
/// GSM is used to modify for subsequent text the height and/or the width of all primary and alternative
/// fonts identified by FONT SELECTION (FNT) and established by GRAPHIC SIZE SELECTION (GSS).
///
/// The established values remain in effect until the next occurrence of GSM or GSS in the data steam.
///
/// `height` and `width` are percentage of values established by GSS ([select_size]).
pub fn modify_size(height: usize, width: usize) -> ControlSequence {
    ControlSequence::new(&[&height.to_string(), &width.to_string()], " B")
}

/// # GSS - Graphic size selection
///
/// GSS is used to establish for subsequent text the height and the width of all primary and alternative fonts
/// identified by FONT SELECTION (FNT). The established values remain in effect until the next
/// occurrence of GSS in the data stream.
///
/// `n` specifies the height, the width is implicitly defined by the height.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT
/// SIZE UNIT (SSU).
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

/// # PEC - Presentation expand or contract
///
/// PEC is used to establish the spacing and the extent of the graphic characters for subsequent text. The
/// spacing is specified in the line as multiples of the spacing established by the most recent occurrence of
/// SET CHARACTER SPACING (SCS) or of SELECT CHARACTER SPACING (SHS) or of SPACING
/// INCREMENT (SPI) in the data stream. The extent of the characters is implicitly established by these
/// control functions. The established spacing and the extent remain in effect until the next occurrence of
/// PEC, of SCS, of SHS or of SPI in the data stream.
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

/// # GCC - Graphic character combination
///
/// GCC is used to indicate that two or more graphic characters are to be imaged as one single graphic
/// symbol. GCC with a parameter value of 0 indicates that the following two graphic characters are to be
/// imaged as one single graphic symbol; GCC with a parameter value of 1 and GCC with a parameter value
/// of 2 indicate respectively the beginning and the end of a string of graphic characters which are to be
/// imaged as one single graphic symbol.
///
/// ### Note
/// GCC does not explicitly specify the relative sizes or placements of the component parts of a composite
/// graphic symbol. In the simplest case, two components may be "half-width" and side-by-side. For
/// example, in Japanese text a pair of characters may be presented side-by-side, and occupy the space of a
/// normal-size Kanji character.
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

/// # FNT - Font selection
///
/// FNT is used to identify the character font to be selected as primary or alternative font by subsequent
/// occurrences of SELECT GRAPHIC RENDITION (SGR) in the data stream.
pub fn select_font(font: Font) -> ControlSequence {
    ControlSequence::new(&[&font.to_string(), "0"], " D")
}

pub enum TextDelimiter {
    End,
    BeginPrincipal,
    BeginSupplementary,
    BeginSupplementaryPhoneticJapanese,
    BeginSupplementaryPhoneticChinese,
    EndPhonetic,
}

impl Display for TextDelimiter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TextDelimiter::End => "0",
            TextDelimiter::BeginPrincipal => "1",
            TextDelimiter::BeginSupplementary => "2",
            TextDelimiter::BeginSupplementaryPhoneticJapanese => "3",
            TextDelimiter::BeginSupplementaryPhoneticChinese => "4",
            TextDelimiter::EndPhonetic => "5",
        })
    }
}

/// # PTX - Parallel texts
/// 
/// PTX is used to delimit strings of graphic characters that are communicated one after another in the data
/// stream but that are intended to be presented in parallel with one another, usually in adjacent lines.
/// 
/// PTX with a parameter value of 1 indicates the beginning of the string of principal text intended to be
/// presented in parallel with one or more strings of supplementary text.
///
/// PTX with a parameter value of 2, 3 or 4 indicates the beginning of a string of supplementary text that is
/// intended to be presented in parallel with either a string of principal text or the immediately preceding
/// string of supplementary text, if any; at the same time it indicates the end of the preceding string of
/// principal text or of the immediately preceding string of supplementary text, if any. The end of a string of
/// supplementary text is indicated by a subsequent occurrence of PTX with a parameter value other than 1.
/// PTX with a parameter value of 0 indicates the end of the strings of text intended to be presented in
/// parallel with one another.
///
///### NOTE
/// PTX does not explicitly specify the relative placement of the strings of principal and supplementary
/// parallel texts, or the relative sizes of graphic characters in the strings of parallel text. A string of
/// supplementary text is normally presented in a line adjacent to the line containing the string of principal
/// text, or adjacent to the line containing the immediately preceding string of supplementary text, if any.
/// The first graphic character of the string of principal text and the first graphic character of a string of
/// supplementary text are normally presented in the same position of their respective lines. However, a
/// string of supplementary text longer (when presented) than the associated string of principal text may be
/// centred on that string. In the case of long strings of text, such as paragraphs in different languages, the
/// strings may be presented in successive lines in parallel columns, with their beginnings aligned with one
/// another and the shorter of the paragraphs followed by an appropriate amount of "white space".
///
/// Japanese phonetic annotation typically consists of a few half-size or smaller Kana characters which
/// indicate the pronunciation or interpretation of one or more Kanji characters and are presented above
/// those Kanji characters if the character path is horizontal, or to the right of them if the character path is
/// vertical.
///
/// Chinese phonetic annotation typically consists of a few Pinyin characters which indicate the
/// pronunciation of one or more Hanzi characters and are presented above those Hanzi characters.
/// Alternatively, the Pinyin characters may be presented in the same line as the Hanzi characters and
/// following the respective Hanzi characters. The Pinyin characters will then be presented within enclosing
/// pairs of parentheses
pub fn parallel_texts(text_delimiter: TextDelimiter) -> ControlSequence {
    ControlSequence::new(&[&text_delimiter.to_string()], "\\")
}


#[derive(Clone)]
pub struct GraphicSelection {
    modes: Vec<String>,
}
impl GraphicSelection {
    pub fn new() -> Self { Self { modes: vec![] } }

    /// Default rendition (implementation-defined), cancels the effect of any preceding occurrence of SGR in
    /// the data stream regardless of the setting of the GRAPHIC RENDITION COMBINATION MODE (GRCM).
    pub fn default(&mut self) -> &mut Self { self.add("0") }
    
    /// Bold or increased intensity
    pub fn bold(&mut self) -> &mut Self { self.add("1") }
    
    /// Faint, decreased intensity or second color
    pub fn faint(&mut self) -> &mut Self { self.add("2") }
    pub fn italic(&mut self) -> &mut Self { self.add("3") }
    pub fn underline(&mut self) -> &mut Self { self.add("4") }
    
    /// Slowly blinking (less than 150/minute)
    pub fn slow_blink(&mut self) -> &mut Self { self.add("5") }
    
    /// Rapidly blinking (150/minute or more)
    pub fn fast_blink(&mut self) -> &mut Self { self.add("6") }
    pub fn negative(&mut self) -> &mut Self { self.add("7") }
    pub fn conceal(&mut self) -> &mut Self { self.add("8") }
    
    /// Crossed-out (characters still legible but marked as to be deleted)
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
    
    /// Normal color or normal intensity
    pub fn not_bold_or_faint(&mut self) -> &mut Self { self.add("22") }
    
    /// Not italicized, not gothic font
    pub fn not_italic(&mut self) -> &mut Self { self.add("23") }
    
    /// Not underline (neither singly or doubly)
    pub fn not_underline(&mut self) -> &mut Self { self.add("24") }
    
    /// Steady (not blinking)
    pub fn not_blink(&mut self) -> &mut Self { self.add("25") }
    
    /// Positive image
    pub fn not_negative(&mut self) -> &mut Self { self.add("27") }
    
    /// Revealed characters
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

/// # SGR - Select graphic rendition
///
/// SGR is used to establish one or more graphic rendition aspects for subsequent text. The established
/// aspects remain in effect until the next occurrence of SGR in the data stream, depending on the setting of
/// the GRAPHIC RENDITION COMBINATION MODE (GRCM).
///
/// ### Example
/// ```
/// use coded_chars::control::rendition::select_graphic;
///
/// // Direct format
/// println!("Hello {}{}{} !", select_graphic().fg_red().bold().underline(), "World", select_graphic().default());
/// ```
pub fn select_graphic() -> GraphicSelection {
    GraphicSelection::new()
}

impl Display for GraphicSelection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

/// Format a string with the specified `SGR` sequence.
///
/// The string is terminated with the sequence `\x1b[0m` to reset the style.
///
/// ### Example
/// ```
/// use coded_chars::control::rendition::{format_str, select_graphic};
///
/// let formatted = format_str(
///     "World",
///     select_graphic().fg_red().bold().underline()
///  );
/// println!("Hello {} !", formatted);
/// ```
pub fn format_str(str: &str, format: &GraphicSelection) -> String {
    format!("{}{}{}", format, str, select_graphic().default())
}