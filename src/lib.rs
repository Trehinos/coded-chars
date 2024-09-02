
//! This crate implements the ECMA-48 standard for coded characters in rust.
//!
//! Various constructions are provided to easily add control character and sequences inside text.
//!
//! This crate is compatible with "ANSI terminals".
//!
//! # Standard implemented
//! - [ecma-48](https://ecma-international.org/publications-and-standards/standards/ecma-48/) - Control Functions for Coded Character Sets
//!
//! > Implementation status : **80%**
//!
//! ## An example : format a text printed in an ECMA/ANSI terminal
//! ```
//! use coded_chars::control::clear_screen;
//! use coded_chars::control::cursor::set_position;
//! use coded_chars::control::rendition::{format_str, select_graphic};
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
//! - CBT : [control::tabulation::cursor_backward]
//! - CCH : [escape::CCH]
//! - CHT : [control::tabulation::cursor_forward]
//! - CMD : [escape::CMD]
//! - CNL : [control::cursor::Direction::NextLine] (see [control::cursor::move_cursor])
//! - CPL : [control::cursor::Direction::PreviousLine] (see [control::cursor::move_cursor])
//! - CPR : [control::cursor::position_report]
//! - CR : [characters::format::CR]
//! - CSI : [escape::CSI] (see [control::ControlSequence])
//! - CTC : [control::tabulation::cursor_control]
//! - CUB : [control::cursor::Direction::Backward] (see [control::cursor::move_cursor])
//! - CUD : [control::cursor::Direction::Down] (see [control::cursor::move_cursor])
//! - CUF : [control::cursor::Direction::Forward] (see [control::cursor::move_cursor])
//! - CUP : [control::cursor::set_position]
//! - CUU : [control::cursor::Direction::Up] (see [control::cursor::move_cursor])
//! - CVT : [control::tabulation::line_tabulation]
//! - DA : [control::device::attributes]
//! - DAQ : [control::area::define_qualification]
//! - DCH : [control::rendition::delete_char]
//! - DCS : [escape::DCS]
//! - DC1 : [characters::device::DC1]
//! - DC2 : [characters::device::DC2]
//! - DC3 : [characters::device::DC3]
//! - DC4 : [characters::device::DC4]
//! - DL : [control::rendition::delete_line]
//! - DLE : [characters::transmission::DLE]
//! - DMI : [escape::DMI]
//! - DSR : [control::device::report_status] & [control::device::StatusReport]
//! - DTA : [control::area::dimension_text]
//! - EA : [control::area::erase]
//! - ECH : [control::rendition::erase_char]
//! - ED : [control::area::erase_in_page]
//! - EF : [control::area::erase_in_field]
//! - EL : [control::area::erase_in_line]
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
//! - FNT : [control::rendition::select_font] & [control::rendition::Font]
//! - GCC : [control::rendition::character_combination] & [control::rendition::Combination]
//! - GSM : [control::rendition::modify_size]
//! - GSS : [control::rendition::select_size]
//! - HPA : [control::cursor::character_absolute]
//! - HPB : [control::cursor::character_backward]
//! - HPR : [control::cursor::character_forward]
//! - HT : [characters::format::HT]
//! - HTJ : [escape::HTJ]
//! - HTS : [escape::HTS]
//! - HVP : [control::cursor::character_and_line_position]
//! - ICH : [control::rendition::insert_char]
//! - IDCS : [control::device::identify_control_string] & [control::device::ControlString]
//! - IGS : [control::device::identify_graphic_sub]
//! - IL : [control::rendition::insert_line]
//! - INT : [escape::INT]
//! - IS1 : [characters::separator::US]
//! - IS2 : [characters::separator::RS]
//! - IS3 : [characters::separator::GS]
//! - IS4 : [characters::separator::FS]
//! - JFY : [control::view::justify] & [control::view::JustifyMode]
//! - LF : [characters::format::LF]
//! - LS0 : [characters::shift::LS0]
//! - LS1 : [characters::shift::LS1]
//! - LS1R : [escape::LS1R]
//! - LS2 : [escape::LS2]
//! - LS2R : [escape::LS2R]
//! - LS3 : [escape::LS3]
//! - LS3R : [escape::LS3R]
//! - MC : [control::device::media_copy] & [control::device::CopyStatus]
//! - MW : [escape::MW]
//! - NAK : [characters::transmission::NAK]
//! - NBH : [escape::NBH]
//! - NEL : [escape::NEL]
//! - NP : [control::rendition::next_page]
//! - NUL : [characters::NUL]
//! - OSC : [escape::OSC]
//! - PEC : [control::rendition::expand_or_condense] & [control::rendition::Expansion]
//! - PFS : [control::area::select_page_format] & [control::area::PageFormat]
//! - PLD : [escape::PLD]
//! - PLU : [escape::PLU]
//! - PM : [escape::PM]
//! - PP : [control::rendition::previous_page]
//! - PPA : [control::cursor::page_position]
//! - PPB : [control::cursor::page_backward]
//! - PPR : [control::cursor::page_forward]
//! - PTX : [control::rendition::parallel_texts]
//! - PU1 : [escape::PU1]
//! - PU2 : [escape::PU2]
//! - QUAD : [control::rendition::quad] & [control::rendition::Layout]
//! - REP : [control::rendition::repeat]
//! - RI : [escape::RI]
//! - RIS : [escape::RIS]
//! - RM : [control::mode::Mode::reset]
//! - SACS : [control::rendition::character_separation]
//! - SAPV : [control::rendition::select_alternative] & [control::rendition::PresentationVariant]
//! - SCI : [escape::SCI]
//! - SCO : [control::rendition::character_orientation]
//! - SCP : [control::rendition::character_path]
//! - SD : [control::view::ScrollDirection::Down] (see [control::view::scroll])
//! - SDS : todo
//! - SEE : todo
//! - SEF : todo
//! - SGR : [control::rendition::select_graphic] & [control::rendition::GraphicSelection]
//! - SHS : todo
//! - SI : [characters::shift::SI]
//! - SIMD : todo
//! - SL : todo
//! - SLH : todo
//! - SLL : todo
//! - SLS : todo
//! - SM : [control::mode::Mode::set]
//! - SO : [characters::shift::SO]
//! - SOH : [characters::transmission::SOH]
//! - SOS : [escape::SOS]
//! - SPA : [escape::SPA]
//! - SPD : todo
//! - SPH : [control::view::set_page_home]
//! - SPI : todo
//! - SPL : todo
//! - SPQR : todo
//! - SR : todo
//! - SRCS : todo
//! - SRS : todo
//! - SSA : [escape::SSA]
//! - SSU : todo
//! - SSW : todo
//! - SS2 : [escape::SS2]
//! - SS3 : [escape::SS3]
//! - ST : [escape::ST]
//! - STAB : todo
//! - STS : [escape::STS]
//! - STX : [characters::transmission::STX]
//! - SU : todo
//! - SUB : [characters::SUB]
//! - SVS : todo
//! - SYN : [characters::transmission::SYN]
//! - TAC : todo
//! - TALE : todo
//! - TATE : todo
//! - TBC : todo
//! - TCC : todo
//! - TSR : todo
//! - TSS : todo
//! - VPA : todo
//! - VPB : todo
//! - VPR : todo
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
        use crate::control::rendition::{format_str, select_graphic};

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
