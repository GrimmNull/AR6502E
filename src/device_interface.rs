
pub mod device_interface {
    use crate::bus::bus::Bus;
    use crate::event_listener::event_listener::EventListener;

    pub trait Device {
        fn wake(self: &mut Self, bus: Bus);

        fn get_id(self: &Self) -> String;

        fn get_event_listeners(self: &mut Self) -> EventListener;

        fn get_bus(self: &mut Self) -> u8;

        fn set_memory_space(self: &mut Self, addresses: Vec<String>);

        fn get_memory_width(self: &Self) -> u8;

        fn get_window(self: &Self);
    }
}