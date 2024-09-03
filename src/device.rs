//! Control sequences that are devices-related.

use std::fmt::{Display, Formatter};
use crate::control::ControlSequence;
use crate::escape::{escape, EscapeSequence};

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

/// Disable manual input
pub const DMI: EscapeSequence = escape('`');

/// Enable manual input
pub const EMI: EscapeSequence = escape('b');

/// Interrupt
pub const INT: EscapeSequence = escape('a');

/// Reset to initial state
pub const RIS: EscapeSequence = escape('c');

/// # DA - Device attributes
///
/// With a parameter value not equal to 0, DA is used to identify the device which sends the DA. The
/// parameter value is a device type identification code according to a register which is to be established. If
/// the parameter value is 0, DA is used to request an identifying DA from a device.
pub fn attributes(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], "c")
}

#[derive(Copy, Clone, Debug)]
pub enum StatusReport {
    /// Ready, no malfunction detected.
    Ready,
    /// Busy, another DSR must be requested later.
    BusyRetry,
    /// Busy, another DSR will be sent later.
    BusyWaiting,
    /// Some malfunction detected, another DSR must be requested later.
    ErrorRetry,
    /// Some malfunction detected, another DSR will be sent later.
    ErrorWaiting,
    /// A DSR is requested.
    MessageWaiting,
    /// A report of the active presentation position or of the active data position in the form of ACTIVE
    /// POSITION REPORT (CPR) is requested.
    PositionWaiting,
}

impl Display for StatusReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StatusReport::Ready => "0",
            StatusReport::BusyRetry => "1",
            StatusReport::BusyWaiting => "2",
            StatusReport::ErrorRetry => "3",
            StatusReport::ErrorWaiting => "4",
            StatusReport::MessageWaiting => "5",
            StatusReport::PositionWaiting => "6",
        })
    }
}

/// # DSR - Device status report
///
/// DSR is used either to report the status of the sending device or to request a status report from the
/// receiving device.
///
/// DSR with parameter value 0 [StatusReport::Ready], 1 [StatusReport::BusyRetry], 2 [StatusReport::BusyWaiting], 3 [StatusReport::ErrorRetry]
/// or 4 [StatusReport::ErrorWaiting] may be sent either unsolicited or as a response to a request such as a DSR with
/// a parameter value 5 [StatusReport::MessageWaiting] or MESSAGE WAITING (MW).
pub fn report_status(status_report: StatusReport) -> ControlSequence {
    ControlSequence::new(&[&status_report.to_string()], "c")
}

/// # FNK - Function key
///
/// FNK is a control function in which the parameter value identifies the function key which has been
/// operated.
pub fn function_key(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " W")
}
/// # IDCS - Identify device control string
///
/// IDCS is used to specify the purpose and format of the command string of subsequent DEVICE
/// CONTROL STRINGs (DCS). The specified purpose and format remain in effect until the next
/// occurrence of IDCS in the data stream.
///
/// The format and interpretation of the command string corresponding to these parameter values are to be
/// defined in appropriate standards. If this control function is used to identify a private command string, a
/// private parameter value shall be used.
pub fn identify_control_string(control_string: ControlString) -> ControlSequence {
    ControlSequence::new(&[&control_string.to_string()], " O")
}

#[derive(Copy, Clone, Debug)]
pub enum ControlString {
    SRTMDiagnose,
    Ecma35DCRS,
}

impl Display for ControlString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ControlString::SRTMDiagnose => "1",
            ControlString::Ecma35DCRS => "2"
        })
    }
}

/// # IGS - Identify graphic sub-repertoire
///
/// IGS is used to indicate that a repertoire of the graphic characters of ISO/IEC 10367 is used in the
/// subsequent text.
/// The parameter value of IGS identifies a graphic character repertoire registered in accordance with
/// ISO/IEC 7350.
pub fn identify_graphic_sub(n: usize) -> ControlSequence {
    ControlSequence::new(&[&n.to_string()], " W")
}

#[derive(Copy, Clone, Debug)]
pub enum CopyStatus {
    /// Initiate transfer to a primary auxiliary device.
    InitTo1,
    /// Initiate transfer from a primary auxiliary device.
    InitFrom1,
    /// Initiate transfer to a secondary auxiliary device.
    InitTo2,
    /// Initiate transfer from a secondary auxiliary device.
    InitFrom2,
    /// Stop relay to a primary auxiliary device.
    Stop1,
    /// Start relay to a primary auxiliary device.
    Start1,
    /// Stop relay to a secondary auxiliary device.
    Stop2,
    /// Start relay to a secondary auxiliary device.
    Start2,
}

impl Display for CopyStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CopyStatus::InitTo1 => "0",
            CopyStatus::InitFrom1 => "1",
            CopyStatus::InitTo2 => "2",
            CopyStatus::InitFrom2 => "3",
            CopyStatus::Stop1 => "4",
            CopyStatus::Start1 => "5",
            CopyStatus::Stop2 => "6",
            CopyStatus::Start2 => "7",
        })
    }
}

/// # MC - Media copy
///
/// MC is used either to initiate a transfer of data from or to an auxiliary input/output device or to enable or
/// disable the relay of the received data stream to an auxiliary input/output device.
///
/// This control function may not be used to switch on or off an auxiliary device.
pub fn media_copy(copy_status: CopyStatus) -> ControlSequence {
    ControlSequence::new(&[&copy_status.to_string()], "i")
}

/// # SEF - Sheet eject and feed
///
/// SEF causes a sheet of paper to be ejected from a printing device into a specified output stacker and
/// another sheet to be loaded into the printing device from a specified paper bin.
pub fn eject_and_feed(bin: usize, stacker: usize) -> ControlSequence {
    ControlSequence::new(&[&bin.to_string(), &stacker.to_string()], " Y")
}