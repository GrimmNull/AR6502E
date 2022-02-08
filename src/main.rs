mod memory_manager;
mod state;
mod action;
mod reducer;
mod action_types;
mod device_interface;
mod bus;
mod event_listener_middleware;
mod event_listener;
mod colored_display;
mod middleware_interface;


use std::borrow::Borrow;
use state::state_type::get_initial_state;
use memory_manager::memory_manager::get_processor;
use reducer::reducer_type::reducer;


fn main() {

    let mut m6502 = get_processor(
         get_initial_state(),
        reducer,
        1000,
         vec![],
        vec![]
    );
    m6502.run();
}
