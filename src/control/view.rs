use crate::control::ControlSequence;

/// # SPH - Set page home
/// 
/// If the DEVICE COMPONENT SELECT MODE is set to PRESENTATION, SPH is used to establish at
/// line position n in the active page (the page that contains the active presentation position) and subsequent
/// pages in the presentation component the position to which the active presentation position will be moved
/// by subsequent occurrences of FORM FEED (FF) in the data stream; where n equals the value of Pn. In
/// the case of a device without data component, it is also the position ahead of which no implicit movement
/// of the active presentation position shall occur.
/// 
/// If the DEVICE COMPONENT SELECT MODE is set to DATA, SPH is used to establish at line position
/// n in the active page (the page that contains the active data position) and subsequent pages in the data
/// component the position to which the active data position will be moved by subsequent occurrences of
/// FORM FEED (FF) in the data stream; where n equals the value of Pn. It is also the position ahead of
/// which no implicit movement of the active presentation position shall occur.
///
/// The established position is called the page home position and remains in effect until the next occurrence
/// of SPH in the data stream.
pub fn set_page_home(c: usize) -> ControlSequence {
    ControlSequence::new(&[&c.to_string()], " i")
}