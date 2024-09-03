//! This module provides control characters not sorted elsewhere....
//!
//! Complementary information in ISO 1745.

pub mod separator {
    /// Unit separator
    pub const US: char = '\x1F';
    
    /// Record separator
    pub const RS: char = '\x1E';
    
    /// Group separator
    pub const GS: char = '\x1D';
    
    /// File separator
    pub const FS: char = '\x1C';
}

/// # Null character
/// 
/// NUL is used for media-fill or time-fill. NUL characters may be inserted into, or removed from, a data
/// stream without affecting the information content of that stream, but such action may affect the
/// information layout and/or the control of equipment.
pub const NUL:char = '\x00';

/// # Bell
/// 
/// BEL is used when there is a need to call for attention; it may control alarm or attention devices.
pub const BEL:char = '\x07';

/// # Cancel
/// 
/// CAN is used to indicate that the data preceding it in the data stream is in error. As a result, this data
/// shall be ignored. The specific meaning of this control function shall be defined for each application
/// and/or between sender and recipient.
pub const CAN:char = '\x18';

/// # End of medium
/// 
/// EM is used to identify the physical end of a medium, or the end of the used portion of a medium, or the
/// end of the wanted portion of data recorded on a medium.
pub const EM:char = '\x19';

/// # Substitute
/// 
/// SUB is used in the place of a character that has been found to be invalid or in error. SUB is intended to
/// be introduced by automatic means.
pub const SUB:char = '\x1A';

/// Space
pub const SPC:char = '\x20';

/// Delete
pub const DEL:char = '\x7f';