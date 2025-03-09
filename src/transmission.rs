//! All transmission-related characters.

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soh() {
        assert_eq!(SOH, '\x01');
    }

    #[test]
    fn test_stx() {
        assert_eq!(STX, '\x02');
    }

    #[test]
    fn test_etx() {
        assert_eq!(ETX, '\x03');
    }

    #[test]
    fn test_eot() {
        assert_eq!(EOT, '\x04');
    }

    #[test]
    fn test_enq() {
        assert_eq!(ENQ, '\x05');
    }

    #[test]
    fn test_ack() {
        assert_eq!(ACK, '\x06');
    }

    #[test]
    fn test_dle() {
        assert_eq!(DLE, '\x10');
    }

    #[test]
    fn test_nak() {
        assert_eq!(NAK, '\x15');
    }

    #[test]
    fn test_syn() {
        assert_eq!(SYN, '\x16');
    }

    #[test]
    fn test_etb() {
        assert_eq!(ETB, '\x17');
    }
}
