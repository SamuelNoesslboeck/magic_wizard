mod mbcp;
pub use mbcp::*;

type ButtonEvent<T> = fn(&mut T) -> ();

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
    pub on_js_sw_released : Option<ButtonEvent<T>>
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
            on_js_sw_released: None
        }
    }
}

pub struct StateMachine<T> {
    pub other_state : T,
    pub old_state : MBCPState,
    pub event_list : EventList<T>
}

impl<T> StateMachine<T> {
    pub fn new(other_state: T) -> Self {
        StateMachine {
            other_state,
            old_state: MBCPState::default(),
            event_list: EventList::default()
        }
    }

    pub fn eval(&mut self, new_state : MBCPButtonState) {

    }
    
    pub fn timeout(&mut self) {
        
    }
}