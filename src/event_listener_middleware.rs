
pub mod event_listener_middleware {
    use crate::action::action_type::Action;
    use crate::event_listener::event_listener::{EventListener, EventType};
    use crate::middleware_interface::middleware_interface::Middleware;
    use crate::state::state_type::State;

    #[derive(Clone)]
    pub struct EventListenerMiddleware {
        pub events: Vec<EventListener>
    }

    impl Middleware for EventListenerMiddleware {
        fn run(&self, state: State, action: &Action) -> State {
            for mut event in self.events.clone() {
                match event.event_type {
                    EventType::Page => {
                        if !action.action_arg1.contains("$") {
                            return state
                        }
                        let first_operand = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                        if first_operand > (64 * (event.page - 1)) as i64 && first_operand < (64 * event.page - 1) as i64 {
                            event.device_bus.clear_bus();
                            event.device_bus.write_to_buss(state.memory[64 * (event.page - 1) as usize..(64 * event.page - 1) as usize].to_vec());
                            (event.device_call)();
                        }
                        break;
                    }
                    EventType::Address => {
                        if action.action_arg1 == event.address {
                            event.device_bus.clear_bus();
                            let index = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                            event.device_bus.write_to_buss(vec![state.memory[index as usize]]);
                            (event.device_call)();
                        }
                        break;
                    }
                    EventType::Addresses => {
                        if event.addresses.contains(&action.action_arg1) {
                            let mut bus = event.device_bus.content.clone();
                            let index = event.addresses.binary_search(&action.action_arg1).unwrap();
                            let memory_index = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                            bus[index] = state.memory[memory_index as usize].clone();
                            event.device_bus.write_to_buss(bus);
                            (event.device_call)();
                        }
                        break;
                    }
                }
            }
            return state;
        }
    }

    impl EventListenerMiddleware {
        pub fn add_event_listener(&mut self, event: EventListener) {
            self.events.push(event);
        }

        pub fn remove_event_listener_by_index(&mut self, index: u8) {
            self.events.remove(index as usize);
        }

        pub fn remove_event_listener(&mut self, event: EventListener) {
            let index = self.events.iter().position(|r| r.id == event.id).unwrap();
            self.events.remove(index);
        }
    }
}