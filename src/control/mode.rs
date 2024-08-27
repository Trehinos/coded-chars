use std::fmt::Display;
use crate::control::ControlSequence;

pub struct Mode {
    modes: Vec<String>,
}
impl Mode {
    pub fn new() -> Self { Self { modes: vec![] } }
    pub fn guarded_area_transfer(&mut self) -> &mut Self { self.add("1") }
    pub fn keyboard_action(&mut self) -> &mut Self { self.add("2") }
    pub fn control_representation(&mut self) -> &mut Self { self.add("3") }
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