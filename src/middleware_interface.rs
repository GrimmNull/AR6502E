
pub mod middleware_interface {
    use crate::action::action_type::Action;
    use crate::state::state_type::State;

    pub trait Middleware {
        fn run(&self, state: State, action: &Action) -> State;
    }
}