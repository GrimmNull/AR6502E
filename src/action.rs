
pub mod action_type {
    use crate::action_types::action_types::ActionTypes;

    #[derive(Clone)]
    pub struct Action {
        pub action_type: ActionTypes,
        pub action_arg1: String,
        pub action_arg2: String,
        pub action_arg3: String
    }
}