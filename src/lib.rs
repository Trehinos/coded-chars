//! This crate implements the ECMA-48 standard for coded characters in rust.
//!
//! Various constructions are provided to easily add control character and sequences inside text.
//!
//! This crate is compatible with "ANSI terminals".
//!
//! ### Standard implemented
//! - [ecma-48](https://ecma-international.org/publications-and-standards/standards/ecma-48/) - Control Functions for Coded Character Sets
//!
//! ### Example : format text in an ANSI terminal
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

pub mod characters;
pub mod escape;
pub mod control;


#[cfg(test)]
mod tests {
    use crate::control::clear_screen;
    use crate::control::rendition::{format_str, select_graphic};

    #[test]
    fn test() {

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
    }

}
