
pub mod device_interface {
    use crate::bus::bus::Bus;
    use crate::state::state_type::State;

    pub trait Device {
        fn wake(&mut self, state: State);

        fn get_id(&self) -> String;

        fn get_event_listeners(&self);

        fn get_bus(&mut self) -> &mut Bus;

        fn set_memory_space(&mut self, addresses: Vec<String>);

        fn get_memory_width(&self) -> u8;

        fn get_window(&self);
    }
}