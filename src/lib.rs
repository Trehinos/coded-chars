
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
//! use coded_chars::control::clear_screen;
//! use coded_chars::control::cursor::set_position;
//! use coded_chars::control::presentation::{format_str, select_graphic};
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
//! - ACK : [characters::transmission::ACK]
//! - APC : [escape::APC]
//! - BEL : [characters::BEL]
//! - BPH : [escape::BPH]
//! - CAN : [characters::CAN]
//! - CBT : [control::cursor::tabulation_backward]
//! - CCH : [escape::CCH]
//! - CHT : [control::cursor::tabulation_forward]
//! - CMD : [escape::CMD]
//! - CNL : [control::cursor::Direction::NextLine] (see [control::cursor::move_cursor])
//! - CPL : [control::cursor::Direction::PreviousLine] (see [control::cursor::move_cursor])
//! - CPR : [control::cursor::position_report]
//! - CR : [characters::format::CR]
//! - CSI : [escape::CSI] (see [control::ControlSequence])
//! - CTC : [control::cursor::tabulation_control]
//! - CUB : [control::cursor::Direction::Backward] (see [control::cursor::move_cursor])
//! - CUD : [control::cursor::Direction::Down] (see [control::cursor::move_cursor])
//! - CUF : [control::cursor::Direction::Forward] (see [control::cursor::move_cursor])
//! - CUP : [control::cursor::set_position]
//! - CUU : [control::cursor::Direction::Up] (see [control::cursor::move_cursor])
//! - CVT : [control::cursor::line_tabulation]
//! - DA : [control::device::attributes]
//! - DAQ : [control::editor::area_qualification]
//! - DCH : [control::editor::delete_char]
//! - DCS : [escape::DCS]
//! - DC1 : [characters::device::DC1]
//! - DC2 : [characters::device::DC2]
//! - DC3 : [characters::device::DC3]
//! - DC4 : [characters::device::DC4]
//! - DL : [control::editor::delete_line]
//! - DLE : [characters::transmission::DLE]
//! - DMI : [escape::DMI]
//! - DSR : [control::device::report_status] (see [control::device::StatusReport])
//! - DTA : [control::presentation::dimension_text]
//! - EA : [control::editor::erase]
//! - ECH : [control::editor::erase_char]
//! - ED : [control::editor::erase_in_page]
//! - EF : [control::editor::erase_in_field]
//! - EL : [control::editor::erase_in_line]
//! - EM : [characters::EM]
//! - EMI : [escape::EMI]
//! - ENQ : [characters::transmission::ENQ]
//! - EOT : [characters::transmission::EOT]
//! - EPA : [escape::EPA]
//! - ESA : [escape::ESA]
//! - ESC : [characters::ESC]
//! - ETB : [characters::transmission::ETB]
//! - ETX : [characters::transmission::ETX]
//! - FF : [characters::format::FF]
//! - FNK : [control::device::function_key]
//! - FNT : [control::presentation::select_font] (see [control::presentation::Font])
//! - GCC : [control::presentation::character_combination] (see [control::presentation::Combination])
//! - GSM : [control::presentation::modify_size]
//! - GSS : [control::presentation::select_size]
//! - HPA : [control::format::character_absolute]
//! - HPB : [control::format::character_backward]
//! - HPR : [control::format::character_forward]
//! - HT : [characters::format::HT]
//! - HTJ : [escape::HTJ]
//! - HTS : [escape::HTS]
//! - HVP : [control::format::character_and_line_position]
//! - ICH : [control::editor::insert_char]
//! - IDCS : [control::device::identify_control_string] (see [control::device::ControlString])
//! - IGS : [control::device::identify_graphic_sub]
//! - IL : [control::editor::insert_line]
//! - INT : [escape::INT]
//! - IS1 : [characters::separator::US]
//! - IS2 : [characters::separator::RS]
//! - IS3 : [characters::separator::GS]
//! - IS4 : [characters::separator::FS]
//! - JFY : [control::presentation::justify] (see [control::presentation::JustifyMode])
//! - LF : [characters::format::LF]
//! - LS0 : [characters::shift::LS0]
//! - LS1 : [characters::shift::LS1]
//! - LS1R : [escape::LS1R]
//! - LS2 : [escape::LS2]
//! - LS2R : [escape::LS2R]
//! - LS3 : [escape::LS3]
//! - LS3R : [escape::LS3R]
//! - MC : [control::device::media_copy] (see [control::device::CopyStatus])
//! - MW : [escape::MW]
//! - NAK : [characters::transmission::NAK]
//! - NBH : [escape::NBH]
//! - NEL : [escape::NEL]
//! - NP : [control::display::next_page]
//! - NUL : [characters::NUL]
//! - OSC : [escape::OSC]
//! - PEC : [control::presentation::expand_or_condense] (see [control::presentation::Expansion])
//! - PFS : [control::presentation::select_page_format] (see [control::presentation::PageFormat])
//! - PLD : [escape::PLD]
//! - PLU : [escape::PLU]
//! - PM : [escape::PM]
//! - PP : [control::display::previous_page]
//! - PPA : [control::format::page_position]
//! - PPB : [control::format::page_backward]
//! - PPR : [control::format::page_forward]
//! - PTX : [control::presentation::parallel_texts] (see [control::presentation::TextDelimiter])
//! - PU1 : [escape::PU1]
//! - PU2 : [escape::PU2]
//! - QUAD : [control::presentation::quad] (see [control::presentation::Layout])
//! - REP : [control::presentation::repeat]
//! - RI : [escape::RI]
//! - RIS : [escape::RIS]
//! - RM : [control::mode::Mode::reset]
//! - SACS : [control::presentation::add_separation]
//! - SAPV : [control::presentation::select_alternative] (see [control::presentation::PresentationVariant])
//! - SCI : [escape::SCI]
//! - SCO : [control::presentation::character_orientation] (see [control::presentation::Orientation])
//! - SCP : [control::presentation::character_path] (see [control::presentation::CharacterPath] & [control::presentation::PathEffect])
//! - SD : [control::display::ScrollDirection::Down] (see [control::display::scroll])
//! - SDS : [control::presentation::directed] (see [control::presentation::StringDirection])
//! - SEE : [control::editor::select_extent] (see [control::editor::EditingExtent])
//! - SEF : [control::device::eject_and_feed]
//! - SGR : [control::presentation::select_graphic] (see [control::presentation::GraphicSelection])
//! - SHS : [control::presentation::select_spacing] (see [control::presentation::CharacterSpacing])
//! - SI : [characters::shift::SI]
//! - SIMD : [control::cursor::select_implicit] (see [control::cursor::MovementDirection])
//! - SL : [control::display::ScrollDirection::Left] (see [control::display::scroll])
//! - SLH : [control::presentation::line_home]
//! - SLL : [control::presentation::line_limit]
//! - SLS : [control::presentation::line_spacing]
//! - SM : [control::mode::Mode::set]
//! - SO : [characters::shift::SO]
//! - SOH : [characters::transmission::SOH]
//! - SOS : [escape::SOS]
//! - SPA : [escape::SPA]
//! - SPD : [control::presentation::select_directions] (see [control::presentation::LineOrientation], [control::presentation::CharacterPath] and [control::presentation::PathEffect])
//! - SPH : [control::presentation::page_home]
//! - SPI : [control::presentation::spacing_increment]
//! - SPL : [control::presentation::page_limit]
//! - SPQR : [control::presentation::print_quality]
//! - SR : [control::display::ScrollDirection::Right] (see [control::display::scroll])
//! - SRCS : [control::presentation::reduce_separation]
//! - SRS : [control::presentation::reversed] (see [control::presentation::StringReversion])
//! - SSA : [escape::SSA]
//! - SSU : [control::presentation::select_size_unit] (see [control::presentation::SizeUnit])
//! - SSW : [control::presentation::space_width]
//! - SS2 : [escape::SS2]
//! - SS3 : [escape::SS3]
//! - ST : [escape::ST]
//! - STAB : [control::presentation::select_tabulation]
//! - STS : [escape::STS]
//! - STX : [characters::transmission::STX]
//! - SU : [control::display::ScrollDirection::Up] (see [control::display::scroll])
//! - SUB : [characters::SUB]
//! - SVS : [control::presentation::select_line_spacing]
//! - SYN : [characters::transmission::SYN]
//! - TAC : [control::presentation::align_center]
//! - TALE : [control::presentation::align_trailing]
//! - TATE : [control::presentation::align_trailing]
//! - TBC : [control::format::clear_tabulation]
//! - TCC : [control::presentation::tabulation_center_on_char]
//! - TSR : [control::format::remove_tabulation_stop]
//! - TSS : [control::presentation::specify_thin_space]
//! - VPA : [control::format::line_position]
//! - VPB : [control::format::line_backward]
//! - VPR : [control::format::line_forward]
//! - VT : [characters::format::VT]
//! - VTS : [escape::VTS]

pub mod characters;
pub mod escape;
pub mod control;


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::control::clear_screen;
        use crate::control::cursor::set_position;
        use crate::control::presentation::{format_str, select_graphic};

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
