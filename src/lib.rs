//! This crate implements the ECMA-48 standard for coded characters in rust.
//!
//! Various constructions are provided to easily add control character and sequences inside text.
//!
//! This crate is compatible with "ANSI terminals".
//!
//! # Standard implemented
//! - [ecma-48](https://ecma-international.org/publications-and-standards/standards/ecma-48/) - Control Functions for Coded Character Sets
//!
//! ## An example : format a text printed in an ANSI terminal
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
//! - CBT : todo
//! - CCH : [escape::CCH]
//! - CHT : todo
//! - CMD : [escape::CMD]
//! - CNL : [control::cursor::Direction::NextLine] (see [control::cursor::move_cursor])
//! - CPL : [control::cursor::Direction::PreviousLine] (see [control::cursor::move_cursor])
//! - CPR : todo
//! - CR : [characters::format::CR]
//! - CSI : [escape::CSI] (see [control::ControlSequence])
//! - CTC : todo
//! - CUB : [control::cursor::Direction::Backward] (see [control::cursor::move_cursor])
//! - CUD : [control::cursor::Direction::Down] (see [control::cursor::move_cursor])
//! - CUF : [control::cursor::Direction::Forward] (see [control::cursor::move_cursor])
//! - CUP : [control::cursor::set_position]
//! - CUU : [control::cursor::Direction::Up] (see [control::cursor::move_cursor])
//! - CVT : todo
//! - DA : todo
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
//! - DSR : todo
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
//! - FNK : todo
//! - FNT : [control::rendition::select_font]
//! - GCC : [control::rendition::character_combination]
//! - GSM : [control::rendition::modify_size]
//! - GSS : [control::rendition::select_size]
//! - HPA : todo
//! - HPB : todo
//! - HPR : todo
//! - HT : [characters::format::HT]
//! - HTJ : [escape::HTJ]
//! - HTS : [escape::HTS]
//! - HVP : todo
//! - ICH : [control::rendition::insert_char]
//! - IDCS : todo
//! - IGS : todo
//! - IL : [control::rendition::insert_line]
//! - INT : [escape::INT]
//! - IS1 : [characters::separator::US]
//! - IS2 : [characters::separator::RS]
//! - IS3 : [characters::separator::GS]
//! - IS4 : [characters::separator::FS]
//! - JFY : todo
//! - LF : [characters::format::LF]
//! - LS0 : [characters::shift::LS0]
//! - LS1 : [characters::shift::LS1]
//! - LS1R : [escape::LS1R]
//! - LS2 : [escape::LS2]
//! - LS2R : [escape::LS2R]
//! - LS3 : [escape::LS3]
//! - LS3R : [escape::LS3R]


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
