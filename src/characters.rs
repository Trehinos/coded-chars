//! This module provides all 32 control characters (+ SP).
//!
//! Complementary information in ISO 1745.
pub mod transmission {
    /// # Start of heading
    ///
    /// SOH is used to indicate the beginning of a heading.
    pub const SOH: char = '\x01';

    /// # Start of text
    ///
    /// STX is used to indicate the beginning of a text and the end of a heading.
    pub const STX: char = '\x02';

    /// # End of text
    ///
    /// ETX is used to indicate the end of a text.
    pub const ETX: char = '\x03';

    /// # End of transmission
    ///
    /// EOT is used to indicate the conclusion of the transmission of one or more texts.
    pub const EOT: char = '\x04';

    /// # Enquiry
    ///
    /// ENQ is transmitted by a sender as a request for a response from a receiver.
    pub const ENQ: char = '\x05';

    /// # Acknowledge
    ///
    /// ACK is transmitted by a receiver as an affirmative response to the sender.
    pub const ACK: char = '\x06';

    /// # Data link escape
    ///
    /// DLE is used exclusively to provide supplementary transmission control functions.
    pub const DLE: char = '\x10';

    /// # Negative Acknowledge
    /// 
    /// NAK is transmitted by a receiver as a negative response to the sender.
    pub const NAK: char = '\x15';

    /// # Synchronous idle
    /// 
    /// SYN is used by a synchronous transmission system in the absence of any other character (idle condition) to
    /// provide a signal from which synchronism may be achieved or retained between data terminal equipment.
    pub const SYN: char = '\x16';

    /// # End of transmission block
    /// 
    /// ETB is used to indicate the end of a block of data where the data are divided into such blocks for transmission purposes.
    pub const ETB: char = '\x17';
}

pub mod device {
    /// # Device control 1
    /// 
    /// DC1 is primarily intended for turning on or starting an ancillary device. If it is not required for this
    /// purpose, it may be used to restore a device to the basic mode of operation (see also DC2 and DC3), or
    /// any other device control function not provided by other DCs.
    /// 
    /// ### Note
    /// When used for data flow control, DC1 is sometimes called **X-ON**.
    pub const DC1: char = '\x11';

    /// # Device control 2
    /// 
    /// DC2 is primarily intended for turning on or starting an ancillary device. If it is not required for this
    /// purpose, it may be used to set a device to a special mode of operation (in which case DC1 is used to
    /// restore the device to the basic mode), or for any other device control function not provided by other DCs.
    pub const DC2: char = '\x12';

    /// # Device control 3
    /// 
    /// DC3 is primarily intended for turning off or stopping an ancillary device. This function may be a
    /// secondary level stop, for example wait, pause, stand-by or halt (in which case DC1 is used to restore
    /// normal operation). If it is not required for this purpose, it may be used for any other device control
    /// function not provided by other DCs.
    /// 
    /// ### Note
    /// 
    /// When used for data flow control, DC3 is sometimes called "X-OFF".
    pub const DC3: char = '\x13';

    /// # Device control 4
    /// 
    /// DC4 is primarily intended for turning off, stopping or interrupting an ancillary device. If it is not
    /// required for this purpose, it may be used for any other device control function not provided by other DCs.
    pub const DC4: char = '\x14';
}

pub mod format {
    /// # Backspace
    /// 
    /// BS causes the active data position to be moved one character position in the data component in the
    /// direction opposite to that of the implicit movement.
    ///
    /// The direction of the implicit movement depends on the parameter value of SELECT IMPLICIT
    /// MOVEMENT DIRECTION (SIMD).
    pub const BS: char = '\x08';
    
    /// # Horizontal tabulation
    /// 
    /// HT causes the active presentation position to be moved to the following character tabulation stop in the
    /// presentation component.
    /// 
    /// In addition, if that following character tabulation stop has been set by TABULATION ALIGN CENTRE
    /// (TAC), TABULATION ALIGN LEADING EDGE (TALE), TABULATION ALIGN TRAILING EDGE
    /// (TATE) or TABULATION CENTRED ON CHARACTER (TCC), HT indicates the beginning of a string
    /// of text which is to be positioned within a line according to the properties of that tabulation stop. The end
    /// of the string is indicated by the next occurrence of HT or CARRIAGE RETURN (CR) or NEXT LINE
    /// (NEL) in the data stream.
    pub const HT: char = '\x09';
    
    /// # Line feed
    /// 
    /// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, LF causes the
    /// active presentation position to be moved to the corresponding character position of the following line in
    /// the presentation component.
    ///
    /// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, LF causes the active data
    /// position to be moved to the corresponding character position of the following line in the data
    /// component
    pub const LF: char = '\x0A';
    
    /// # Vertical tabulation
    /// 
    /// VT causes the active presentation position to be moved in the presentation component to the
    /// corresponding character position on the line at which the following line tabulation stop is set.
    pub const VT: char = '\x0B';
    
    /// # Form feed
    /// 
    /// FF causes the active presentation position to be moved to the corresponding character position of the
    /// line at the page home position of the next form or page in the presentation component. The page home
    /// position is established by the parameter value of SET PAGE HOME (SPH).
    pub const FF: char = '\x0C';
    
    /// # Carriage return
    /// 
    /// The effect of CR depends on the setting of the DEVICE COMPONENT SELECT MODE (DCSM) and
    /// on the parameter value of SELECT IMPLICIT MOVEMENT DIRECTION (SIMD).
    ///
    /// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION and with the
    /// parameter value of SIMD equal to 0, CR causes the active presentation position to be moved to the line
    /// home position of the same line in the presentation component. The line home position is established by
    /// the parameter value of SET LINE HOME (SLH).
    ///
    /// With a parameter value of SIMD equal to 1, CR causes the active presentation position to be moved to
    /// the line limit position of the same line in the presentation component. The line limit position is
    /// established by the parameter value of SET LINE LIMIT (SLL).
    ///
    /// If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA and with a parameter value of
    /// SIMD equal to 0, CR causes the active data position to be moved to the line home position of the same
    /// line in the data component. The line home position is established by the parameter value of SET LINE
    /// HOME (SLH).
    ///
    /// With a parameter value of SIMD equal to 1, CR causes the active data position to be moved to the line
    /// limit position of the same line in the data component. The line limit position is established by the
    /// parameter value of SET LINE LIMIT (SLL).
    pub const CR: char = '\x0D';
    
}
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
pub mod shift {
    /// # Shift in
    /// 
    /// SI is used for code extension purposes. It causes the meanings of the bit combinations following it in the
    /// data stream to be changed.  
    /// The use of SI is defined in Standard ECMA-35.
    /// 
    /// ### Note
    /// 
    /// SI is used in 7-bit environments only; in 8-bit environments LOCKING-SHIFT ZERO (LS0) is used
    /// instead.
    pub const SI: char = '\x0F';
    pub const LS0: char = SI;
    
    /// # Shift out
    /// 
    /// SO is used for code extension purposes. It causes the meanings of the bit combinations following it in
    /// the data stream to be changed.  
    /// The use of SO is defined in Standard ECMA-35.
    ///
    /// ### Note
    ///
    /// SO is used in 7-bit environments only; in 8-bit environments LOCKING-SHIFT ONE (LS1) is used
    /// instead.
    pub const SO: char = '\x0E';
    pub const LS1: char = SO;
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

/// # Escape
/// 
/// ESC is used for code extension purposes. It causes the meanings of a limited number of bit combinations
/// following it in the data stream to be changed.
///
/// The use of ESC is defined in Standard ECMA-35.
pub const ESC:char = '\x1B';

/// Space
pub const SPC:char = '\x20';

/// Delete
pub const DEL:char = '\x7f';