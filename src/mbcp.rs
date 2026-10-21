#[derive(Clone, Debug, Default)]
pub struct MBCPButtonStates(pub u8);

impl MBCPButtonStates {
    pub fn a1_active(&self) -> bool {
        self.0 & 0x01 != 0
    }

    pub fn a2_active(&self) -> bool {
        self.0 & 0x02 != 0
    }

    pub fn a3_active(&self) -> bool {
        self.0 & 0x04 != 0
    }

    pub fn ult_active(&self) -> bool {
        self.0 & 0x08 != 0
    }

    pub fn re_sw_active(&self) -> bool {
        self.0 & 0x10 != 0
    }

    pub fn js_sw_active(&self) -> bool {
        self.0 & 0x20 != 0
    }
}

#[derive(Clone, Debug, Default)]
pub struct MBCPState {
    pub buttons : MBCPButtonStates,
    pub re_rel_val : i8,
    pub js_x_val : i8,
    pub js_y_val : i8
}