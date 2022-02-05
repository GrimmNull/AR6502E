mod memory_manager;
mod state;
mod action;
mod reducer;
mod action_types;


use state::state_type::get_initial_state;
use memory_manager::memory_manager::Processor;
use reducer::reducer_type::reducer;


fn main() {
    let mut m6502 = Processor{
        state: get_initial_state(),
        frequency: 1000
    };
    m6502.run(reducer);
}
