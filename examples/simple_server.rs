use magic_wizard::{ButtonID, MBCPEvent, StateMachine};

fn main() -> std::io::Result<()> {
    let mut state_machine = StateMachine::new("0.0.0.0:22609", |state, event| {
        match event {
            MBCPEvent::ButtonPress(ButtonID::A1) => {
                println!("Button A1 pressed!");
            }
            _ => { }
        }
    })?;


    state_machine.start_msg_loop()
}
