
pub mod memory_manager {
    use rand::Rng;
    use std::borrow::{Borrow, BorrowMut};
    use std::str::FromStr;
    use text_io::read;
    use crate::state::state_type::State;
    use crate::action::action_type::Action;
    use crate::action_types::action_types::ActionTypes;
    use crate::bus::bus::Bus;
    use crate::device_interface::device_interface::Device;
    use crate::event_listener_middleware::event_listener_middleware::EventListenerMiddleware;
    use crate::middleware_interface::middleware_interface::Middleware;



    pub struct Processor {
        user_tag: String,
        running: bool,
        pub state: State,
        pub reducer: fn (state: State, action: &Action) -> State,
        pub event_listener_manager: EventListenerMiddleware,
        pub frequency: i32,
        pub pre_dispatch: Vec<fn (state: State, action: &Action) -> State>,
        pub post_dispatch: Vec<fn (state: State, action: &Action) -> State>,
        pub busses: Vec<Bus>,
        pub devices: Vec<Box<dyn Device>>
    }

    impl Processor {
        fn dispatch(&mut self,  action: Action) {
            dispatch_middleware(self.state.clone(), action.borrow(), self.pre_dispatch.clone());

            self.state = (self.reducer)(self.state.clone(), action.borrow());

            dispatch_middleware(self.state.clone(), action.borrow(), self.post_dispatch.clone());
            self.event_listener_manager.run(self.state.clone(), action.borrow(), self.busses.borrow_mut(), self.devices.borrow_mut());
        }

        fn mount_devices(&mut self, devices: Vec<Box<dyn Device>>) {
            for mut device in devices.iter() {
                self.busses.push(Bus{ content: vec![] });
                self.devices.push(**device.borrow_mut());
                self.event_listener_manager.add_event_listener(device.get_event_listeners());
            }
        }


        pub fn run(&mut self) {
            if self.running {
                println!("The processor is already running for a user");
                return
            }
            self.running = true;

            loop {
                println!("Type your command: ");
                let line: String = read!("{}\n");
                let tokenized_line: Vec<&str> = line.split(" ").collect();
                self.dispatch(Action{
                    action_type: ActionTypes::from_str(&*tokenized_line[0].to_uppercase()).unwrap(),
                    action_arg1: if tokenized_line.len() >= 2 { tokenized_line[1].to_string()} else {"".to_string()},
                    action_arg2: if tokenized_line.len() >= 3 { tokenized_line[2].to_string()} else {"".to_string()},
                    action_arg3: if tokenized_line.len() >= 4 { tokenized_line[3].to_string()} else {"".to_string()},
                    dispatcher: "user_".to_string() + &*self.user_tag.to_string()
                });
                println!();
                self.state.print_flags();
                println!();
                self.state.print_memory(1);
                println!();
            }
        }
    }

    fn dispatch_middleware(state: State, action: &Action, mut middleware_list: Vec<fn (state: State, action: &Action) -> State>) -> State {
        if middleware_list.is_empty() {
            return state;
        }
        middleware_list.remove(0);
        return middleware_list[0](dispatch_middleware(state, action, middleware_list), action.borrow());
    }


    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#create-random-passwords-from-a-set-of-alphanumeric-characters
    fn generate_random_user_tag() -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        const TAG_LEN: usize = 10;
        let mut rng = rand::thread_rng();

        let user_tag: String = (0..TAG_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        return user_tag;
    }

    pub fn get_processor(initial_state: State, reducer: fn (state: State, action: &Action) -> State, frequency: i32, pre_dispatch: Vec<fn (state: State, action: &Action) -> State>, post_dispatch: Vec<fn (state: State, action: &Action) -> State>, devices: Vec<Box<dyn Device>>) -> Processor {
        let event_manager = EventListenerMiddleware{
            events: vec![]
        };
        let mut proc = Processor {
            user_tag: generate_random_user_tag(),
            running: false,
            state: initial_state,
            event_listener_manager: event_manager,
            reducer,
            frequency,
            pre_dispatch,
            post_dispatch: vec![],
            busses: vec![],
            devices: vec![]
        };
        proc.mount_devices(devices);
        return proc;
    }
}