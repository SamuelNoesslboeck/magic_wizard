use core::time::Duration;
use std::net::{ToSocketAddrs, UdpSocket};
use std::io;

mod mbcp;
pub use mbcp::*;

pub type ButtonEvent<T> = fn(&mut T) -> ();
pub type EncoderEvent<T> = fn(&mut T, i32, i8) -> ();
pub type JoysStickEvent<T> = fn(&mut T, (i8, i8)) -> ();
pub type JoyStickChangedEvent<T> = fn(&mut T, (i8, i8), (i8, i8)) -> ();

pub struct EventList<T> {
    pub on_a1_pressed : Option<ButtonEvent<T>>,
    pub on_a2_pressed : Option<ButtonEvent<T>>,
    pub on_a3_pressed : Option<ButtonEvent<T>>,
    pub on_ult_pressed : Option<ButtonEvent<T>>,
    pub on_re_sw_pressed : Option<ButtonEvent<T>>,
    pub on_js_sw_pressed : Option<ButtonEvent<T>>,

    pub on_a1_released : Option<ButtonEvent<T>>,
    pub on_a2_released : Option<ButtonEvent<T>>,
    pub on_a3_released : Option<ButtonEvent<T>>,
    pub on_ult_released : Option<ButtonEvent<T>>,
    pub on_re_sw_released : Option<ButtonEvent<T>>,
    pub on_js_sw_released : Option<ButtonEvent<T>>,

    pub on_re_moved : Option<EncoderEvent<T>>,

    pub on_js_active : Option<JoysStickEvent<T>>,
    pub on_js_changed : Option<JoyStickChangedEvent<T>>
}

impl<T> Default for EventList<T> {
    fn default() -> Self {
        EventList {
            on_a1_pressed: None,
            on_a2_pressed: None,
            on_a3_pressed: None,
            on_ult_pressed: None,
            on_re_sw_pressed: None,
            on_js_sw_pressed: None,

            on_a1_released: None,
            on_a2_released: None,
            on_a3_released: None,
            on_ult_released: None,
            on_re_sw_released: None,
            on_js_sw_released: None,

            on_re_moved: None,

            on_js_active: None,
            on_js_changed: None
        }
    }
}

pub struct StateMachine<T> {
    pub other_state : T,
    pub encoder_counter : i32,
    pub old_state : MBCPState,
    pub event_list : EventList<T>,
    pub socket : UdpSocket
}

impl<T> StateMachine<T> {
    pub fn new<A : ToSocketAddrs>(bind_address : A, other_state: T) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(bind_address)?;
        socket.set_read_timeout(Some(Duration::from_millis(200)))?;

        Ok(Self {
            other_state,
            encoder_counter: 0,
            old_state: MBCPState::default(),
            event_list: EventList::default(),
            socket
        })
    }

    pub fn eval(&mut self, new_state : MBCPState) {
        // Macro or helper style expansion for each button:
        // --- A1 ---
        if !self.old_state.buttons.a1_active() && new_state.buttons.a1_active() {
            if let Some(event) = self.event_list.on_a1_pressed {
                event(&mut self.other_state);
            }
        }
        if self.old_state.buttons.a1_active() && !new_state.buttons.a1_active() {
            if let Some(event) = self.event_list.on_a1_released {
                event(&mut self.other_state);
            }
        }

        // --- A2 ---
        if !self.old_state.buttons.a2_active() && new_state.buttons.a2_active() {
            if let Some(event) = self.event_list.on_a2_pressed {
                event(&mut self.other_state);
            }
        }
        if self.old_state.buttons.a2_active() && !new_state.buttons.a2_active() {
            if let Some(event) = self.event_list.on_a2_released {
                event(&mut self.other_state);
            }
        }

        // --- A3 ---
        if !self.old_state.buttons.a3_active() && new_state.buttons.a3_active() {
            if let Some(event) = self.event_list.on_a3_pressed {
                event(&mut self.other_state);
            }
        }
        if self.old_state.buttons.a3_active() && !new_state.buttons.a3_active() {
            if let Some(event) = self.event_list.on_a3_released {
                event(&mut self.other_state);
            }
        }

        // --- ULT ---
        if !self.old_state.buttons.ult_active() && new_state.buttons.ult_active() {
            if let Some(event) = self.event_list.on_ult_pressed {
                event(&mut self.other_state);
            }
        }
        if self.old_state.buttons.ult_active() && !new_state.buttons.ult_active() {
            if let Some(event) = self.event_list.on_ult_released {
                event(&mut self.other_state);
            }
        }

        // --- RE_SW ---
        if !self.old_state.buttons.re_sw_active() && new_state.buttons.re_sw_active() {
            if let Some(event) = self.event_list.on_re_sw_pressed {
                event(&mut self.other_state);
            }
        }
        if self.old_state.buttons.re_sw_active() && !new_state.buttons.re_sw_active() {
            if let Some(event) = self.event_list.on_re_sw_released {
                event(&mut self.other_state);
            }
        }

        // --- JS_SW ---
        if !self.old_state.buttons.js_sw_active() && new_state.buttons.js_sw_active() {
            if let Some(event) = self.event_list.on_js_sw_pressed {
                event(&mut self.other_state);
            }
        }
        if self.old_state.buttons.js_sw_active() && !new_state.buttons.js_sw_active() {
            if let Some(event) = self.event_list.on_js_sw_released {
                event(&mut self.other_state);
            }
        }

        /* Rotary Encoder */
        if new_state.re_rel_val != 0 {
            if let Some(event) = self.event_list.on_re_moved {
                event(&mut self.other_state, self.encoder_counter, new_state.re_rel_val);
            }
        }

        /* JoyStick */
        if (new_state.js_x_val != 0) || (new_state.js_y_val != 0) {
            if let Some(event) = self.event_list.on_js_active {
                event(&mut self.other_state, (new_state.js_x_val, new_state.js_y_val));
            }
        }

        if (new_state.js_x_val != self.old_state.js_x_val) || (new_state.js_y_val != self.old_state.js_y_val) {
            if let Some(event) = self.event_list.on_js_changed {
                event(&mut self.other_state, (self.old_state.js_x_val, self.old_state.js_y_val), (new_state.js_x_val, new_state.js_y_val))
            }
        }

        // Update old state at the end of evaluation
        self.encoder_counter += new_state.re_rel_val as i32;
        self.old_state = new_state;
    }
    
    pub fn timeout(&mut self) {
        self.eval(MBCPState::default());
    }

    pub fn start_msg_loop(&mut self) -> std::io::Result<()> {
        let mut buf = [0; 64];
        
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((number_of_bytes, _)) => {
                    if number_of_bytes == 4 {
                        let fixed_buf : [u8; 4] = buf[0..4].try_into().unwrap();    // TODO: Remove unwrap

                        self.eval(unsafe { 
                            core::mem::transmute(fixed_buf)
                        });
                    } else {
                        // Bad message, skipping. TODO: Implement maybe some error handling?
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut => {
                    self.timeout();
                }
                Err(e) => {
                    println!("An error occurred: {}", e);
                }
            }
        }
    }
}