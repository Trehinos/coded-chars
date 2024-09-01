use crate::control::ControlSequence;

/// A struct representing an `SM` or an `RM` CSI function.
///
/// ### Examples
/// ```
/// use coded_chars::control::mode::mode;
///
/// // Set a mode (EXPLICIT-BDSM mode)
/// println!("{}", mode().bi_directional_support().set());
///
/// // Reset a mode (IMPLICIT-BDSM mode)
/// println!("{}", mode().bi_directional_support().reset());
/// ```
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

    /// # SRTM - Status report transfer mode
    ///
    /// - NORMAL (set) :  Status reports in the form of DEVICE CONTROL STRINGs (DCS) are not generated automatically.
    /// - DIAGNOSTIC (reset) : Status reports in the form of DEVICE CONTROL STRINGs (DCS) are included in every data stream transmitted or transferred.
    ///
    /// ### Note
    /// No control functions are affected.
    pub fn status_report_transfer(&mut self) -> &mut Self { self.add("5") }

    /// # ERM - Erasure mode
    ///
    /// - PROTECT (set) : Only the contents of unprotected areas are affected by an erasure control function.
    /// - ALL (reset) : The contents of protected as well as of unprotected areas are affected by an erasure control function.
    ///
    /// ### Note
    /// Control functions affected are: EA, ECH, ED, EF, EL.
    ///
    /// - [crate::control::area::erase]
    /// - [crate::control::rendition::erase_char]
    /// - [crate::control::area::erase_in_page]
    /// - [crate::control::area::erase_in_field]
    /// - [crate::control::area::erase_in_line]
    pub fn erasure(&mut self) -> &mut Self { self.add("6") }

    /// # VEM - Line editing mode
    ///
    /// - FOLLOWING (set) :
    ///     - If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, a line insertion
    ///     causes the contents of the active line (the line that contains the active presentation position) and of the
    ///     following lines in the presentation component to be shifted in the direction of the line progression; a line
    ///     deletion causes the contents of the lines following the active line to be shifted in the direction opposite
    ///     to that of the line progression.
    ///     - If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, a line insertion causes the
    ///     contents of the active line (the line that contains the active data position) and of the following lines in
    ///     the data component to be shifted in the direction of the line progression; a line deletion causes the
    ///     contents of the lines following the active line to be shifted in the direction opposite to that of the line
    ///     progression.
    /// - PRECEDING (reset) :
    ///     - If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, a line insertion
    ///     causes the contents of the active line (the line that contains the active presentation position) and of the
    ///     preceding lines to be shifted in the direction opposite to that of the line progression; a line deletion
    ///     causes the contents of the lines preceding the active line to be shifted in the direction of the line
    ///     progression.
    ///     - If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, a line insertion causes the
    ///     contents of the active line (the line that contains the active data position) and of the preceding lines to be
    ///     shifted in the direction opposite to that of the line progression; a line deletion causes the contents of the
    ///     lines preceding the active line to be shifted in the direction of the line progression.
    ///
    /// ### Note
    /// Control functions affected are: DL, IL.
    ///
    /// - [crate::control::rendition::delete_line]
    /// - [crate::control::rendition::insert_line]
    pub fn line_editing(&mut self) -> &mut Self { self.add("7") }

    /// # BDSM - Bidirectional support mode
    ///
    /// - EXPLICIT (set) : Control functions are performed in the data component or in the presentation component, depending on
    /// the setting of the DEVICE COMPONENT SELECT MODE (DCSM).
    /// - IMPLICIT (reset) :  Control functions are performed in the data component. All bi-directional aspects of data are handled by
    /// the device itself.
    pub fn bi_directional_support(&mut self) -> &mut Self { self.add("8") }

    /// # DCSM -- Device component select mode
    ///
    /// - PRESENTATION (set) : Certain control functions are performed in the presentation component. The active presentation position
    /// (or the active line, where applicable) in the presentation component is the reference position against
    /// which the relevant control functions are performed.
    /// - DATA (reset) : Certain control functions are performed in the data component. The active data position (or the active
    /// line, where applicable) in the data component is the reference position against which the relevant control
    /// functions are performed
    ///
    /// ### Note
    /// Control functions affected are: CPR, CR, DCH, DL, EA, ECH, ED, EF, EL, ICH, IL, LF, NEL, RI, SLH,
    /// SLL, SPH, SPL.
    ///
    /// - (todo CPR),
    /// - [crate::characters::format::CR],
    /// - [crate::control::rendition::delete_char],
    /// - [crate::control::rendition::delete_line],
    /// - [crate::control::area::erase],
    /// - [crate::control::rendition::erase_char]
    /// - [crate::control::area::erase_in_page]
    /// - [crate::control::area::erase_in_field]
    /// - [crate::control::area::erase_in_line]
    /// - [crate::control::rendition::insert_char]
    /// - [crate::control::rendition::insert_line]
    /// - [crate::characters::format::LF],
    /// - [crate::escape::NEL],
    /// - [crate::escape::RI],
    /// - (todo SLH),
    /// - (todo SLL),
    /// - (todo SPH),
    /// - (todo SPL).
    ///
    pub fn device_component_select(&mut self) -> &mut Self { self.add("9") }

    /// # HEM - Character editing mode
    ///
    /// - FOLLOWING (set) :
    ///     - If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, a character
    ///     insertion causes the contents of the active presentation position and of the following character positions
    ///     in the presentation component to be shifted in the direction of the character path; a character deletion
    ///     causes the contents of the character positions following the active presentation position to be shifted in
    ///     the direction opposite to that of the character path.
    ///     - If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, a character insertion causes
    ///     the contents of the active data position and of the following character positions in the data component to
    ///     be shifted in the direction of the character progression; a character deletion causes the contents of the
    ///     character positions following the active data position to be shifted in the direction opposite to that of the
    ///     character progression.
    /// - PRECEDING (reset) :
    ///     - If the DEVICE COMPONENT SELECT MODE (DCSM) is set to PRESENTATION, a character
    ///     insertion causes the contents of the active presentation position and of the following character positions
    ///     in the presentation component to be shifted in the direction opposite to that of the character path; a
    ///     character deletion causes the contents of the character positions following the active presentation
    ///     position to be shifted in the direction of the character path.
    ///     - If the DEVICE COMPONENT SELECT MODE (DCSM) is set to DATA, a character insertion causes
    ///     the contents of the active data position and of preceding character positions in the data component to be
    ///     shifted in the direction opposite to that of the character progression; a character deletion causes the
    ///     contents of the character positions preceding the active data position to be shifted in the direction of the
    ///     character progression.
    ///
    /// ### Note
    /// Control functions affected are: DCH, ICH.
    ///
    /// - [crate::control::rendition::delete_char],
    /// - [crate::control::rendition::insert_char].
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

    /// # SM - Set Mode
    /// SM causes the modes of the receiving device to be set as specified.
    pub fn set(&self) -> ControlSequence {
        ControlSequence::new(&self.modes.iter().map(|s| s.as_str()).collect::<Vec<_>>(), "h")
    }

    /// # RM - Reset Mode
    /// RM causes the modes of the receiving device to be reset as specified.
    pub fn reset(&self) -> ControlSequence {
        ControlSequence::new(&self.modes.iter().map(|s| s.as_str()).collect::<Vec<_>>(), "l")
    }
    fn add(&mut self, s: &str) -> &mut Self {
        self.modes.push(s.to_string());
        self
    }
}

/// Creates a new [Mode] sequence to set or reset devices modes.
///
/// ### Example
/// ```
/// use coded_chars::control::mode::mode;
///
/// // Sets the DCSM mode to PRESENTATION and the HEM mode to FOLLOWING.
/// mode().device_component_select().character_editing().set().exec();
/// ```
pub fn mode() -> Mode { Mode::new() }