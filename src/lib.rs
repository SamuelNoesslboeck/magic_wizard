#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use core::time::Duration;
use std::net::{ToSocketAddrs, UdpSocket};
use std::io;

mod mbcp;
pub use mbcp::*;

use crate::MBCPEvent::{ButtonPress, ButtonRelease, EncoderChange, JoystickChange};

/// The default address used within the MBCP
pub const MBCP_DEF_ADDRESS : &'static str = "0.0.0.0:22609";

/// ID enum for each button of the MagicBox
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub enum ButtonID {
    A1,
    A2,
    A3,
    ULT,
    RE_SW,
    JS_SW
}

/// Events that can be triggered by the devices of the MagicBox
pub enum MBCPEvent {
    /// Button with the ID [ButtonID] has been pressed
    ButtonPress(ButtonID),
    /// Button with the ID [ButtonID] has been released
    ButtonRelease(ButtonID),
    /// The encoder has seen a relative change `i8`, and is currently at the position `i32`
    EncoderChange(i32, i8),
    /// The joystick has seen a change in position (second tuple `(x, y)`), from the old position (first tuple `(x, y)`)
    JoystickChange((i8, i8), (i8, i8))
}

/// The state machine evaluates changes in the MagicBox state and fires events accordingly
pub struct StateMachine<F : FnMut(&MBCPState, MBCPEvent)> {
    handler : F,
    encoder_counter : i32,
    old_state : MBCPState,
    socket : UdpSocket
}

impl<F : FnMut(&MBCPState, MBCPEvent)> StateMachine<F> {
    /// Create a new state machine, binding it to the given address on your local machine  
    /// On incoming messages, the given handler function is called
    pub fn new<A : ToSocketAddrs>(bind_address : A, handler : F) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(bind_address)?;
        socket.set_read_timeout(Some(Duration::from_millis(200)))?;

        Ok(Self {
            handler,
            encoder_counter: 0,
            old_state: MBCPState::default(),
            socket
        })
    }
    
    /// Evalues a new incoming state and fires events accordingly
    pub fn eval(&mut self, new_state : MBCPState) {
        // --- A1 ---
        if !self.old_state.buttons.a1_active() && new_state.buttons.a1_active() {
            (self.handler)(&new_state, ButtonPress(ButtonID::A1));
        }
        if self.old_state.buttons.a1_active() && !new_state.buttons.a1_active() {
            (self.handler)(&new_state, ButtonRelease(ButtonID::A1));
        }

        // --- A2 ---
        if !self.old_state.buttons.a2_active() && new_state.buttons.a2_active() {
            (self.handler)(&new_state, ButtonPress(ButtonID::A2));
        }
        if self.old_state.buttons.a2_active() && !new_state.buttons.a2_active() {
            (self.handler)(&new_state, ButtonRelease(ButtonID::A2));
        }

        // --- A3 ---
        if !self.old_state.buttons.a3_active() && new_state.buttons.a3_active() {
            (self.handler)(&new_state, ButtonPress(ButtonID::A3));
        }
        if self.old_state.buttons.a3_active() && !new_state.buttons.a3_active() {
            (self.handler)(&new_state, ButtonRelease(ButtonID::A3));
        }

        // --- ULT ---
        if !self.old_state.buttons.ult_active() && new_state.buttons.ult_active() {
            (self.handler)(&new_state, ButtonPress(ButtonID::ULT));
        }
        if self.old_state.buttons.ult_active() && !new_state.buttons.ult_active() {
            (self.handler)(&new_state, ButtonRelease(ButtonID::ULT));
        }

        // --- RE_SW ---
        if !self.old_state.buttons.re_sw_active() && new_state.buttons.re_sw_active() {
            (self.handler)(&new_state, ButtonPress(ButtonID::RE_SW));
        }
        if self.old_state.buttons.re_sw_active() && !new_state.buttons.re_sw_active() {
            (self.handler)(&new_state, ButtonRelease(ButtonID::RE_SW));
        }

        // --- JS_SW ---
        if !self.old_state.buttons.js_sw_active() && new_state.buttons.js_sw_active() {
            (self.handler)(&new_state, ButtonPress(ButtonID::JS_SW));
        }
        if self.old_state.buttons.js_sw_active() && !new_state.buttons.js_sw_active() {
            (self.handler)(&new_state, ButtonRelease(ButtonID::JS_SW));
        }

        /* Rotary Encoder */
        if new_state.re_rel_val != 0 {
            (self.handler)(&new_state, EncoderChange(self.encoder_counter, new_state.re_rel_val));
        }

        /* JoyStick */
        // if (new_state.js_x_val != 0) || (new_state.js_y_val != 0) {
        //     (self.handler)(&new_state, JoystickChange((self.old_state.js_x_val, self.old_state.js_y_val)));
        // }

        if (new_state.js_x_val != self.old_state.js_x_val) || (new_state.js_y_val != self.old_state.js_y_val) {
            (self.handler)(&new_state, JoystickChange((self.old_state.js_x_val, self.old_state.js_y_val), (new_state.js_x_val, new_state.js_y_val)));

        }

        // Update old state at the end of evaluation
        self.encoder_counter += new_state.re_rel_val as i32;
        self.old_state = new_state;
    }
    
    /// Puts the state machine in the timeout state, meaning the MagicBox has not sent a state for a previously set amount of time  
    /// This ensures that devices are released and not held indefinetly when their release event fails to be delivered
    pub fn timeout(&mut self) {
        self.eval(MBCPState::default());
    }

    /// Starts the blocking message loop that handles incoming state messages from the MagicBox  
    /// These messages are evaluated and the connected events created
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