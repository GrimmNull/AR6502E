
pub mod event_listener_middleware {
    use std::borrow::{Borrow, BorrowMut};
    use crate::action::action_type::Action;
    use crate::bus::bus::Bus;
    use crate::device_interface::device_interface::Device;
    use crate::event_listener::event_listener::{EventListener, EventType};
    use crate::state::state_type::State;

    #[derive(Clone)]
    pub struct EventListenerMiddleware {
        pub events: Vec<EventListener>
    }

    impl EventListenerMiddleware {
        pub(crate) fn run(&self, state: State, action: &Action, busses: &mut [Bus], devices: &mut [Box<dyn Device>]) -> State {
            for mut event in self.events.clone() {
                match event.event_type {
                    EventType::Page => {
                        if !action.action_arg1.contains("$") {
                            return state
                        }
                        let first_operand = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                        if first_operand > (64 * (event.page - 1)) as i64 && first_operand < (64 * event.page - 1) as i64 {
                            busses[event.device_bus as usize].clear_bus();
                            busses[event.device_bus as usize].write_to_buss(state.memory[64 * (event.page - 1) as usize..(64 * event.page - 1) as usize].to_vec());
                            let mut index: u8 = 0;
                            for device in devices.iter() {
                                if device.get_id() == event.device_id.to_string() {
                                    break;
                                }
                                index+=1;
                            }
                            let mut device : &dyn Borrow<dyn Device> = devices[index as usize].borrow_mut();
                            device.wake(&device, busses[event.device_bus as usize].clone());
                        }
                        break;
                    }
                    EventType::Address => {
                        if action.action_arg1 == event.address {
                            busses[event.device_bus as usize].clear_bus();
                            let index = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                            busses[event.device_bus as usize].write_to_buss(vec![state.memory[index as usize]]);
                            let mut index: u8 = 0;
                            for device in devices.iter() {
                                if device.get_id() == event.device_id.to_string() {
                                    break;
                                }
                                index+=1;
                            }
                            let mut device = devices[index as usize].borrow_mut();

                            device.wake(busses[event.device_bus as usize].clone());
                        }
                        break;
                    }
                    EventType::Addresses => {
                        if event.addresses.contains(&action.action_arg1) {
                            let mut bus = busses[event.device_bus as usize].content.clone();
                            let index = event.addresses.binary_search(&action.action_arg1).unwrap();
                            let memory_index = i64::from_str_radix(&action.action_arg1.to_string()[1..action.action_arg1.len()], 16).unwrap();
                            bus[index] = state.memory[memory_index as usize].clone();
                            busses[event.device_bus as usize].write_to_buss(bus);
                            let mut index: u8 = 0;
                            for device in devices.iter() {
                                if device.get_id() == event.device_id.to_string() {
                                    break;
                                }
                                index+=1;
                            }
                            let mut device = devices[index as usize].borrow_mut();

                            device.wake(busses[event.device_bus as usize].clone());
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