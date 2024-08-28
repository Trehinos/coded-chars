//! This crate implements [ecma-48](https://ecma-international.org/publications-and-standards/standards/ecma-48/) for rust.
//!
//! Various constructions are provided to easily add control character and sequences inside text.
//!
//! ### Example
//! ```
//! use ecma_48::control::rendition::select;
//!
//! println!("Hello {}{}{} !", select().fg_red().bold().underline(), "World", select().default())
//! ```

pub mod characters;
pub mod escape;
pub mod control;


#[cfg(test)]
mod tests {
    use crate::control::rendition::{format_str, select};

    #[test]
    fn test() {
        let formatted = format_str(
            "World", 
            select().fg_black().bg_yellow().bold().underline()
        );
        println!("Hello {} !", formatted);
    }

}
