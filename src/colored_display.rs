
pub mod colored_display {
    use colorful::Colorful;
    use console::Style;
    use crate::bus::bus::Bus;
    use crate::device_interface::device_interface::Device;
    use crate::event_listener::event_listener::{EventListener, EventType};
    extern crate colorful;

    #[derive(Clone)]
    pub struct ColoredDisplay {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub bus: u8,
        pub id: String,
        pub addresses: Vec<String>
    }

    impl Device for ColoredDisplay {

        fn wake(&mut self, bus: Bus) {
            self.r = bus.content[0].clone();
            self.g = bus.content[1].clone();
            self.b = bus.content[2].clone();

            self.get_window();
        }

        fn get_id(&self) -> String {
            self.id.to_string()
        }

        fn get_event_listeners(&mut self) -> EventListener {
            return EventListener {
                id: 1,
                device_id: self.id.to_string(),
                device_bus: self.bus.clone(),
                event_type: EventType::Addresses,
                page: 0,
                address: "".to_string(),
                addresses: vec!["$1B".to_string(), "$1C".to_string(), "$1D".to_string()]
            }
        }

        fn get_bus(&mut self) -> u8 {
            self.bus.clone()
        }

        fn set_memory_space(&mut self, addresses: Vec<String>) {
            self.addresses = addresses;
        }

        fn get_memory_width(&self) -> u8 {
            return 4;
        }

        fn get_window(&self) {
            println!("-----");
            println!("|    |");
            println!("| {} |", "0".rgb(self.r, self.g, self.b));
            println!("|    |");
            println!("-----");
        }

        fn get_clone(self: &Self) -> Box<dyn Device> {
            return Box::new(self.clone()) as Box<dyn Device>;
        }
    }
}