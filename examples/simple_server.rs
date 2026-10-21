use magic_wizard::StateMachine;

fn main() -> std::io::Result<()> {
    let mut state_machine = StateMachine::new("0.0.0.0:22609", ())?;

    state_machine.event_list.on_a1_pressed = Some(|_| {
        println!("A1 pressed!");
    });

    state_machine.event_list.on_a1_released = Some(|_| {
        println!("A1 released!");
    });

    state_machine.start_msg_loop()
}