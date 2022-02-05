
pub mod memory_manager {
    use std::str::FromStr;
    use text_io::read;
    use crate::state::state_type::State;
    use crate::action::action_type::Action;
    use crate::action_types::action_types::ActionTypes;


    #[derive(Clone)]
    pub struct Processor {
        pub state: State,
        pub frequency: i32
    }

    impl Processor {
        pub fn dispatch(&mut self, reducer: fn (state: State, action: Action) -> State ,  action: Action) {
            self.state = reducer(self.state.clone(), action);
        }

        pub fn run(&mut self, reducer: fn (state: State, action: Action) -> State ) {
            loop {
                println!("Type your command: ");
                let line: String = read!("{}\n");
                let tokenized_line: Vec<&str> = line.split(" ").collect();
                self.dispatch(reducer, Action{
                    action_type: ActionTypes::from_str(&*tokenized_line[0].to_uppercase()).unwrap(),
                    action_arg1: if tokenized_line.len() >= 2 { tokenized_line[1].to_string()} else {"".to_string()},
                    action_arg2: if tokenized_line.len() >= 3 { tokenized_line[2].to_string()} else {"".to_string()},
                    action_arg3: if tokenized_line.len() == 4 { tokenized_line[3].to_string()} else {"".to_string()}
                });
                println!();
                self.state.print_flags();
                println!();
                self.state.print_memory(1);
                println!();
            }
        }
    }
}