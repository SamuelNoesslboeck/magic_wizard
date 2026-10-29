# magic_wizard

A simple library crate that enables to host servers for the [MagicBox](https://github.com/SamuelNoesslboeck/)

```rs
/* # Simple server example
 *
 * This example showcases a simple use of this library to host an UDP port that the MagicBox can send its states to. 
 * The events can be collected as required by the application
 */

use magic_wizard::{ButtonID, MBCP_DEF_ADDRESS, MBCPEvent, StateMachine};

fn main() -> std::io::Result<()> {
    // Create the state machine and bind it to the default port (22609) and all interfaces using 0.0.0.0
    let mut state_machine = StateMachine::new(MBCP_DEF_ADDRESS, |_, event| {
        match event {
            MBCPEvent::ButtonPress(ButtonID::A1) => {
                println!("Button A1 pressed!");
            },
            MBCPEvent::ButtonRelease(ButtonID::RE_SW) => {
                println!("Button RE_SW released!");
            },
            MBCPEvent::EncoderChange(abs_pos, rel) => {
                println!("The encoder moved {} and is now at {}!", rel, abs_pos);
            },
            MBCPEvent::JoystickChange(old_pos, new_pos) => {
                println!("The joystick moved from {:?} to {:?}", old_pos, new_pos);
            }
            _ => { }
        }
    })?;

    // Start the blocking message loop, from now on messages will be received, can be moved to a thread if required
    state_machine.start_msg_loop()
}
```