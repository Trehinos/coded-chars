
//! This crate implements the ECMA-48 standard for coded characters in rust.
//!
//! Various constructions are provided to easily add control character and sequences inside text.
//!
//! This crate is compatible with "ANSI terminals".
//!
//! # Standard implemented
//! - [ecma-48](https://ecma-international.org/publications-and-standards/standards/ecma-48/) - Control Functions for Coded Character Sets
//!
//! ## An example : format a text printed in an ECMA/ANSI terminal
//! ```
//! use coded_chars::clear_screen;
//! use coded_chars::cursor::set_position;
//! use coded_chars::presentation::{format_str, select_graphic};
//!
//! // Direct format
//! println!("Hello {}World{} !", select_graphic().fg_red().bold().underline(), select_graphic().default());
//!
//! // Clear screen
//! clear_screen();
//!
//! // Using format_str
//! let formatted = format_str(
//!     "World",
//!     select_graphic().fg_red().bold().underline()
//!  );
//! println!("Hello {} !", formatted);
//!
//! set_position(5, 1).exec();
//! println!("This line is printed on the fifth line.");
//! ```
//!
//! ## All ECMA-48 control functions
//!
//! - ACK : [transmission::ACK]
//! - APC : [delimiters::APC]
//! - BEL : [characters::BEL]
//! - BPH : [presentation::BPH]
//! - CAN : [characters::CAN]
//! - CBT : [cursor::tabulation_backward]
//! - CCH : [escape::CCH]
//! - CHT : [cursor::tabulation_forward]
//! - CMD : [delimiters::CMD]
//! - CNL : [cursor::Direction::NextLine] (see [cursor::move_cursor])
//! - CPL : [cursor::Direction::PreviousLine] (see [cursor::move_cursor])
//! - CPR : [cursor::position_report]
//! - CR : [format::CR]
//! - CSI : [introducers::CSI] (see [control::ControlSequence])
//! - CTC : [cursor::tabulation_control]
//! - CUB : [cursor::Direction::Backward] (see [cursor::move_cursor])
//! - CUD : [cursor::Direction::Down] (see [cursor::move_cursor])
//! - CUF : [cursor::Direction::Forward] (see [cursor::move_cursor])
//! - CUP : [cursor::set_position]
//! - CUU : [cursor::Direction::Up] (see [cursor::move_cursor])
//! - CVT : [cursor::line_tabulation]
//! - DA : [device::attributes]
//! - DAQ : [area::area_qualification]
//! - DCH : [editor::delete_char]
//! - DCS : [delimiters::DCS]
//! - DC1 : [device::DC1]
//! - DC2 : [device::DC2]
//! - DC3 : [device::DC3]
//! - DC4 : [device::DC4]
//! - DL : [editor::delete_line]
//! - DLE : [transmission::DLE]
//! - DMI : [device::DMI]
//! - DSR : [device::report_status] (see [device::StatusReport])
//! - DTA : [presentation::dimension_text]
//! - EA : [editor::erase]
//! - ECH : [editor::erase_char]
//! - ED : [editor::erase_in_page]
//! - EF : [editor::erase_in_field]
//! - EL : [editor::erase_in_line]
//! - EM : [characters::EM]
//! - EMI : [device::EMI]
//! - ENQ : [transmission::ENQ]
//! - EOT : [transmission::EOT]
//! - EPA : [area::EPA]
//! - ESA : [area::ESA]
//! - ESC : [introducers::ESC]
//! - ETB : [transmission::ETB]
//! - ETX : [transmission::ETX]
//! - FF : [format::FF]
//! - FNK : [device::function_key]
//! - FNT : [presentation::select_font] (see [presentation::Font])
//! - GCC : [presentation::character_combination] (see [presentation::Combination])
//! - GSM : [presentation::modify_size]
//! - GSS : [presentation::select_size]
//! - HPA : [format::character_absolute]
//! - HPB : [format::character_backward]
//! - HPR : [format::character_forward]
//! - HT : [format::HT]
//! - HTJ : [format::HTJ]
//! - HTS : [format::HTS]
//! - HVP : [format::character_and_line_position]
//! - ICH : [editor::insert_char]
//! - IDCS : [device::identify_control_string] (see [device::ControlString])
//! - IGS : [device::identify_graphic_sub]
//! - IL : [editor::insert_line]
//! - INT : [device::INT]
//! - IS1 : [characters::separator::US]
//! - IS2 : [characters::separator::RS]
//! - IS3 : [characters::separator::GS]
//! - IS4 : [characters::separator::FS]
//! - JFY : [presentation::justify] (see [presentation::JustifyMode])
//! - LF : [format::LF]
//! - LS0 : [shifts::LS0]
//! - LS1 : [shifts::LS1]
//! - LS1R : [shifts::LS1R]
//! - LS2 : [shifts::LS2]
//! - LS2R : [shifts::LS2R]
//! - LS3 : [shifts::LS3]
//! - LS3R : [shifts::LS3R]
//! - MC : [device::media_copy] (see [device::CopyStatus])
//! - MW : [escape::MW]
//! - NAK : [transmission::NAK]
//! - NBH : [presentation::NBH]
//! - NEL : [format::NEL]
//! - NP : [display::next_page]
//! - NUL : [characters::NUL]
//! - OSC : [delimiters::OSC]
//! - PEC : [presentation::expand_or_condense] (see [presentation::Expansion])
//! - PFS : [presentation::select_page_format] (see [presentation::PageFormat])
//! - PLD : [format::PLD]
//! - PLU : [format::PLU]
//! - PM : [delimiters::PM]
//! - PP : [display::previous_page]
//! - PPA : [format::page_position]
//! - PPB : [format::page_backward]
//! - PPR : [format::page_forward]
//! - PTX : [presentation::parallel_texts] (see [presentation::TextDelimiter])
//! - PU1 : [escape::PU1]
//! - PU2 : [escape::PU2]
//! - QUAD : [presentation::quad] (see [presentation::Layout])
//! - REP : [presentation::repeat]
//! - RI : [format::RI]
//! - RIS : [device::RIS]
//! - RM : [mode::Mode::reset]
//! - SACS : [presentation::add_separation]
//! - SAPV : [presentation::select_alternative] (see [presentation::PresentationVariant])
//! - SCI : [introducers::SCI]
//! - SCO : [presentation::character_orientation] (see [presentation::Orientation])
//! - SCP : [presentation::character_path] (see [presentation::CharacterPath] & [presentation::PathEffect])
//! - SD : [display::ScrollDirection::Down] (see [display::scroll])
//! - SDS : [presentation::directed] (see [presentation::StringDirection])
//! - SEE : [editor::select_extent] (see [editor::EditingExtent])
//! - SEF : [device::eject_and_feed]
//! - SGR : [presentation::select_graphic] (see [presentation::GraphicSelection])
//! - SHS : [presentation::select_spacing] (see [presentation::CharacterSpacing])
//! - SI : [shifts::SI]
//! - SIMD : [presentation::select_implicit] (see [presentation::MovementDirection])
//! - SL : [display::ScrollDirection::Left] (see [display::scroll])
//! - SLH : [presentation::line_home]
//! - SLL : [presentation::line_limit]
//! - SLS : [presentation::line_spacing]
//! - SM : [mode::Mode::set]
//! - SO : [shifts::SO]
//! - SOH : [transmission::SOH]
//! - SOS : [delimiters::SOS]
//! - SPA : [area::SPA]
//! - SPD : [presentation::select_directions] (see [presentation::LineOrientation], [presentation::CharacterPath] and [presentation::PathEffect])
//! - SPH : [presentation::page_home]
//! - SPI : [presentation::spacing_increment]
//! - SPL : [presentation::page_limit]
//! - SPQR : [presentation::print_quality]
//! - SR : [display::ScrollDirection::Right] (see [display::scroll])
//! - SRCS : [presentation::reduce_separation]
//! - SRS : [presentation::reversed] (see [presentation::StringReversion])
//! - SSA : [area::SSA]
//! - SSU : [presentation::select_size_unit] (see [presentation::SizeUnit])
//! - SSW : [presentation::space_width]
//! - SS2 : [shifts::SS2]
//! - SS3 : [shifts::SS3]
//! - ST : [delimiters::ST]
//! - STAB : [presentation::select_tabulation]
//! - STS : [escape::STS]
//! - STX : [transmission::STX]
//! - SU : [display::ScrollDirection::Up] (see [display::scroll])
//! - SUB : [characters::SUB]
//! - SVS : [presentation::select_line_spacing]
//! - SYN : [transmission::SYN]
//! - TAC : [presentation::align_center]
//! - TALE : [presentation::align_trailing]
//! - TATE : [presentation::align_trailing]
//! - TBC : [format::clear_tabulation]
//! - TCC : [presentation::tabulation_center_on_char]
//! - TSR : [format::remove_tabulation_stop]
//! - TSS : [presentation::specify_thin_space]
//! - VPA : [format::line_position]
//! - VPB : [format::line_backward]
//! - VPR : [format::line_forward]
//! - VT : [format::VT]
//! - VTS : [format::VTS]

use crate::cursor::set_position;
use crate::editor::{erase_in_page, AreaPosition};

pub mod characters;
pub mod transmission;
pub mod introducers;
pub mod escape;
pub mod delimiters;
pub mod shifts;
pub mod control;
pub mod mode;
pub mod cursor;
pub mod device;
pub mod display;
pub mod editor;
pub mod presentation;
pub mod format;
pub mod area;

/// The page is erased and the cursor position is set to the first line and the first column.
///
/// - The ANSI/ECMA printed function is : `ED(2),CUP(1,1)`
/// - The ANSI/ECMA printed sequence is : `\x1b[2J\x1b[1;1H`
///
/// This function is a shorthand for :
/// ```
/// use coded_chars::cursor::set_position;
/// use coded_chars::editor::{erase_in_page, AreaPosition};
/// print!(
///     "{}{}",
///     erase_in_page(AreaPosition::Whole),
///     set_position(1, 1)
/// );
/// ```
pub fn clear_screen() {
    print!("{}{}", erase_in_page(AreaPosition::Whole), set_position(1, 1));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::clear_screen;
        use crate::cursor::set_position;
        use crate::presentation::{format_str, select_graphic};

        // Direct format
        println!("Hello {}World{} !", select_graphic().fg_red().bold().underline(), select_graphic().default());

        // Clear screen
        clear_screen();

        // Using format_str
        let formatted = format_str(
            "World",
            select_graphic().fg_red().bold().underline()
        );
        println!("Hello {} !", formatted);

        set_position(5, 1).exec();
        println!("This line is printed on the fifth line.");
    }

}
