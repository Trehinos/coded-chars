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
    use crate::control::rendition::select;

    #[test]
    fn test() {
        println!("Hello {}{}{} !", select().fg_red().bold().underline(), "World", select().default())
    }

}
