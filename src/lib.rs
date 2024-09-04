
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
//! - Delimiters
//!     - APC : [delimiters::APC]
//!     - CMD : [delimiters::CMD]
//!     - DCS : [delimiters::DCS]
//!     - OSC : [delimiters::OSC]
//!     - PM : [delimiters::PM]
//!     - SOS : [delimiters::SOS]
//!     - ST : [delimiters::ST]
//! - Introducers
//!     - CSI : [introducers::CSI] (see [control::ControlSequence])
//!     - ESC : [introducers::ESC]
//!     - SCI : [introducers::SCI]
//! - Shifts
//!     - LS0 : [shifts::LS0]
//!     - LS1 : [shifts::LS1]
//!     - LS1R : [shifts::LS1R]
//!     - LS2 : [shifts::LS2]
//!     - LS2R : [shifts::LS2R]
//!     - LS3 : [shifts::LS3]
//!     - LS3R : [shifts::LS3R]
//!     - SI : [shifts::SI]
//!     - SO : [shifts::SO]
//!     - SS2 : [shifts::SS2]
//!     - SS3 : [shifts::SS3]
//! - Format
//!     - BS : [format::BS]
//!     - CR : [format::CR]
//!     - FF : [format::FF]
//!     - HPA : [format::character_absolute]
//!     - HPB : [format::character_backward]
//!     - HPR : [format::character_forward]
//!     - HT : [format::HT]
//!     - HTJ : [format::HTJ]
//!     - HTS : [format::HTS]
//!     - HVP : [format::character_and_line_position]
//!     - LF : [format::LF]
//!     - NEL : [format::NEL]
//!     - PLD : [format::PLD]
//!     - PLU : [format::PLU]
//!     - PPA : [format::page_position]
//!     - PPB : [format::page_backward]
//!     - PPR : [format::page_forward]
//!     - RI : [format::RI]
//!     - TBC : [format::clear_tabulation]
//!     - TSR : [format::remove_tabulation_stop]
//!     - VPA : [format::line_position]
//!     - VPB : [format::line_backward]
//!     - VPR : [format::line_forward]
//!     - VT : [format::VT]
//!     - VTS : [format::VTS]
//! - Presentation
//!     - BPH : [presentation::BPH]
//!     - DTA : [presentation::dimension_text]
//!     - FNT : [presentation::select_font] (see [presentation::Font])
//!     - GCC : [presentation::character_combination] (see [presentation::Combination])
//!     - GSM : [presentation::modify_size]
//!     - GSS : [presentation::select_size]
//!     - JFY : [presentation::justify] (see [presentation::JustifyMode])
//!     - PEC : [presentation::expand_or_condense] (see [presentation::Expansion])
//!     - PFS : [presentation::select_page_format] (see [presentation::PageFormat])
//!     - PTX : [presentation::parallel_texts] (see [presentation::TextDelimiter])
//!     - QUAD : [presentation::quad] (see [presentation::Layout])
//!     - REP : [presentation::repeat]
//!     - SACS : [presentation::add_separation]
//!     - SAPV : [presentation::select_alternative] (see [presentation::PresentationVariant])
//!     - SCO : [presentation::character_orientation] (see [presentation::Orientation])
//!     - SCP : [presentation::character_path] (see [presentation::CharacterPath] & [presentation::PathEffect])
//!     - SDS : [presentation::directed] (see [presentation::StringDirection])
//!     - SGR : [presentation::select_graphic] (see [presentation::GraphicSelection])
//!     - SHS : [presentation::select_spacing] (see [presentation::CharacterSpacing])
//!     - SIMD : [presentation::select_implicit] (see [presentation::MovementDirection])
//!     - SLH : [presentation::line_home]
//!     - SLL : [presentation::line_limit]
//!     - SLS : [presentation::line_spacing]
//!     - SPD : [presentation::select_directions] (see [presentation::LineOrientation], [presentation::CharacterPath] and [presentation::PathEffect])
//!     - SPH : [presentation::page_home]
//!     - SPI : [presentation::spacing_increment]
//!     - SPL : [presentation::page_limit]
//!     - SPQR : [presentation::print_quality]
//!     - SRCS : [presentation::reduce_separation]
//!     - SRS : [presentation::reversed] (see [presentation::StringReversion])
//!     - SSU : [presentation::select_size_unit] (see [presentation::SizeUnit])
//!     - SSW : [presentation::space_width]
//!     - STAB : [presentation::select_tabulation]
//!     - SVS : [presentation::select_line_spacing]
//!     - TAC : [presentation::align_center]
//!     - TALE : [presentation::align_trailing]
//!     - TATE : [presentation::align_trailing]
//!     - TCC : [presentation::tabulation_center_on_char]
//!     - TSS : [presentation::specify_thin_space]
//! - Editor
//!     - DCH : [editor::delete_char]
//!     - DL : [editor::delete_line]
//!     - EA : [editor::erase]
//!     - ECH : [editor::erase_char]
//!     - ED : [editor::erase_in_page]
//!     - EF : [editor::erase_in_field]
//!     - EL : [editor::erase_in_line]
//!     - ICH : [editor::insert_char]
//!     - IL : [editor::insert_line]
//!     - SEE : [editor::select_extent] (see [editor::EditingExtent])
//! - Cursor
//!     - CBT : [cursor::tabulation_backward]
//!     - CHT : [cursor::tabulation_forward]
//!     - CNL : [cursor::Direction::NextLine] (see [cursor::move_cursor])
//!     - CPL : [cursor::Direction::PreviousLine] (see [cursor::move_cursor])
//!     - CPR : [cursor::position_report]
//!     - CTC : [cursor::tabulation_control]
//!     - CUB : [cursor::Direction::Backward] (see [cursor::move_cursor])
//!     - CUD : [cursor::Direction::Down] (see [cursor::move_cursor])
//!     - CUF : [cursor::Direction::Forward] (see [cursor::move_cursor])
//!     - CUP : [cursor::set_position]
//!     - CUU : [cursor::Direction::Up] (see [cursor::move_cursor])
//!     - CVT : [cursor::line_tabulation]
//! - Display
//!     - NP : [display::next_page]
//!     - PP : [display::previous_page]
//!     - SD : [display::ScrollDirection::Down] (see [display::scroll])
//!     - SL : [display::ScrollDirection::Left] (see [display::scroll])
//!     - SR : [display::ScrollDirection::Right] (see [display::scroll])
//!     - SU : [display::ScrollDirection::Up] (see [display::scroll])
//! - Device
//!     - DA : [device::attributes]
//!     - DMI : [device::DMI]
//!     - DC1 : [device::DC1]
//!     - DC2 : [device::DC2]
//!     - DC3 : [device::DC3]
//!     - DC4 : [device::DC4]
//!     - DSR : [device::report_status] (see [device::StatusReport])
//!     - EMI : [device::EMI]
//!     - FNK : [device::function_key]
//!     - IDCS : [device::identify_control_string] (see [device::ControlString])
//!     - IGS : [device::identify_graphic_sub]
//!     - INT : [device::INT]
//!     - MC : [device::media_copy] (see [device::CopyStatus])
//!     - RIS : [device::RIS]
//!     - SEF : [device::eject_and_feed]
//! - Separators
//!     - IS1 : [characters::separator::US]
//!     - IS2 : [characters::separator::RS]
//!     - IS3 : [characters::separator::GS]
//!     - IS4 : [characters::separator::FS]
//! - Area
//!     - DAQ : [area::area_qualification]
//!     - EPA : [area::EPA]
//!     - ESA : [area::ESA]
//!     - SPA : [area::SPA]
//!     - SSA : [area::SSA]
//! - Mode
//!     - RM : [mode::Mode::reset]
//!     - SM : [mode::Mode::set]
//! - Transmission
//!     - ACK : [transmission::ACK]
//!     - DLE : [transmission::DLE]
//!     - ENQ : [transmission::ENQ]
//!     - EOT : [transmission::EOT]
//!     - ETB : [transmission::ETB]
//!     - ETX : [transmission::ETX]
//!     - NAK : [transmission::NAK]
//!     - NBH : [presentation::NBH]
//!     - SOH : [transmission::SOH]
//!     - STX : [transmission::STX]
//!     - SYN : [transmission::SYN]
//!
//! Other :
//! - BEL : [characters::BEL]
//! - CAN : [characters::CAN]
//! - CCH : [escape::CCH]
//! - EM : [characters::EM]
//! - MW : [escape::MW]
//! - NUL : [characters::NUL]
//! - PU1 : [escape::PU1]
//! - PU2 : [escape::PU2]
//! - STS : [escape::STS]
//! - SUB : [characters::SUB]

pub mod characters;
pub mod escape;
pub mod delimiters;
pub mod introducers;
pub mod transmission;
pub mod shifts;
pub mod control;
pub mod format;
pub mod presentation;
pub mod editor;
pub mod display;
pub mod device;
pub mod area;
pub mod mode;
pub mod cursor;

/// The page is erased and the cursor position is set to the first line and the first column.
///
/// - The ANSI/ECMA printed function is : `ED(2),CUP(1,1)`
/// - The ANSI/ECMA printed sequence is : `\x1b[2J\x1b[1;1H`
/// 
pub fn clear_screen() {
    use crate::cursor::set_position;
    use crate::editor::{erase_in_page, AreaPosition};
    
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
