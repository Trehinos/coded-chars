//! This crate implements ECMA standards for coded characters in rust.
//!
//! Various constructions are provided to easily add control character and sequences inside text.
//!
//! This crate is compatible with ANSI terminals.
//!
//! ### Standards implemented
//! - [ecma-35](https://ecma-international.org/publications-and-standards/standards/ecma-35/) - Character Code Structure and Extension Techniques
//! - [ecma-43](https://ecma-international.org/publications-and-standards/standards/ecma-43/) - 8-Bit Coded Character Set Structure and Rules
//! - [ecma-48](https://ecma-international.org/publications-and-standards/standards/ecma-48/) - Control Functions for Coded Character Sets
//!
//! ### Example : format text in an ANSI terminal
//! ```
//! use coded_chars::control::rendition::{format_str, next_page, select};
//!
//! // Direct format
//! println!("Hello {}{}{} !", select().fg_red().bold().underline(), "World", select().default());
//!
//! // Clear screen
//! next_page(1);
//!
//! // Using format_str
//! let formatted = format_str(
//!     "World",
//!     select().fg_red().bold().underline()
//!  );
//! println!("Hello {} !", formatted);
//! ```

pub mod characters;
pub mod escape;
pub mod control;


#[cfg(test)]
mod tests {
    use crate::control::rendition::{format_str, next_page, select};

    #[test]
    fn test() {

        // Direct format
        println!("Hello {}{}{} !", select().fg_red().bold().underline(), "World", select().default());

        // Clear screen
        next_page(1);

        // Using format_str
        let formatted = format_str(
            "World",
            select().fg_red().bold().underline()
        );
        println!("Hello {} !", formatted);
    }

}
