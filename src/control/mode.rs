use std::fmt::Display;
use crate::control::ControlSequence;

pub struct Mode {
    modes: Vec<String>,
}
impl Mode {
    pub fn new() -> Self { Self { modes: vec![] } }

    /// # GATM - Guarded area transfer mode
    ///
    /// - GUARD (set) : Only the contents of unguarded areas in an eligible area are transmitted or transferred.
    /// - ALL (reset) : The contents of guarded as well as of unguarded areas in an eligible area are transmitted or transferred.
    ///
    /// ### Note
    /// No control functions are affected.
    pub fn guarded_area_transfer(&mut self) -> &mut Self { self.add("1") }
    
    /// # KAM - Keyboard action mode
    /// 
    /// - ENABLED (set) : All or part of the manual input facilities are enabled to be used.
    /// - DISABLED (reset) : All or part of the manual input facilities are disabled.
    /// 
    /// ### Note
    /// No control functions are affected.
    pub fn keyboard_action(&mut self) -> &mut Self { self.add("2") }
    
    /// # CRM - Control representation mode
    /// 
    /// - CONTROL (set) : All control functions are performed as defined; the way formator functions are processed depends on the
    /// setting of the FORMAT EFFECTOR ACTION MODE (FEAM). A device may choose to image the
    /// graphical representations of control functions in addition to performing them.
    /// - GRAPHIC (reset) : All control functions, except RESET MODE (RM), are treated as graphic characters. A device may
    /// choose to perform some control functions in addition to storing them and imaging their graphical
    /// representations.
    /// 
    /// ### Note
    /// All control functions, except RM, are affected.
    pub fn control_representation(&mut self) -> &mut Self { self.add("3") }
    
    /// # IRM - Insertion replacement mode
    /// 
    /// - REPLACE (set) : The graphic symbol of a graphic character or of a control function, for which a graphical representation
    /// is required, replaces (or, depending upon the implementation, is combined with) the graphic symbol
    /// imaged at the active presentation position.
    /// - INSERT (reset) : The graphic symbol of a graphic character or of a control function, for which a graphical representation
    /// is required, is inserted at the active presentation position.
    /// 
    /// ### Note
    /// Only control functions for which a graphical representation is required are affected.
    pub fn insertion_replacement(&mut self) -> &mut Self { self.add("4") }
    pub fn status_report_transfer(&mut self) -> &mut Self { self.add("5") }
    pub fn erasure(&mut self) -> &mut Self { self.add("6") }
    pub fn line_editing(&mut self) -> &mut Self { self.add("7") }
    pub fn bi_directional_support(&mut self) -> &mut Self { self.add("8") }
    pub fn device_component_select(&mut self) -> &mut Self { self.add("9") }
    pub fn character_editing(&mut self) -> &mut Self { self.add("10") }
    pub fn positioning_unit(&mut self) -> &mut Self { self.add("11") }
    pub fn send_receive(&mut self) -> &mut Self { self.add("12") }
    pub fn format_effector_action(&mut self) -> &mut Self { self.add("13") }
    pub fn format_effector_transfer(&mut self) -> &mut Self { self.add("14") }
    pub fn multiple_area_transfer(&mut self) -> &mut Self { self.add("15") }
    pub fn transfert_termination(&mut self) -> &mut Self { self.add("16") }
    pub fn selected_area_transfer(&mut self) -> &mut Self { self.add("17") }
    pub fn tabulation_stop(&mut self) -> &mut Self { self.add("18") }
    pub fn graphic_rendition_combination(&mut self) -> &mut Self { self.add("21") }
    pub fn zero_default(&mut self) -> &mut Self { self.add("22") }
    pub fn set(&self) -> ControlSequence {
        ControlSequence::new(&self.modes.iter().map(|s| s.as_str()).collect::<Vec<_>>(), "h")
    }
    pub fn reset(&self) -> ControlSequence {
        ControlSequence::new(&self.modes.iter().map(|s| s.as_str()).collect::<Vec<_>>(), "l")
    }
    fn add(&mut self, s: &str) -> &mut Self {
        self.modes.push(s.to_string());
        self
    }
}

pub fn mode() -> Mode { Mode::new() }