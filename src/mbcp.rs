/// A single byte that stores the state of all buttons of the MagicBox
#[derive(Clone, Debug, Default)]
pub struct MBCPButtonStates(pub u8);

#[allow(missing_docs)]
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

/// The state that is used within the MBCP to communicate different actions used on the MagicBox
#[derive(Clone, Debug, Default)]
pub struct MBCPState {
    /// The states of the all the buttons on the MagicBox
    pub buttons : MBCPButtonStates,
    /// Relative value of how much the rotary encoder has moved
    pub re_rel_val : i8,
    /// Current value of the joystick in X direction
    pub js_x_val : i8,
    /// Current value of the joystick in Y direction
    pub js_y_val : i8
}