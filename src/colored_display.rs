
pub mod colored_display {
    use console::Style;
    use std::borrow::{Borrow, BorrowMut};
    use crate::bus::bus::Bus;
    use crate::device_interface::device_interface::Device;
    use crate::state::state_type::State;

    pub struct ColoredDisplay {
        r: u8,
        g: u8,
        b: u8,
        bus: Bus,
        id: String,
        addresses: Vec<String>
    }

    impl Device for ColoredDisplay {

        fn wake(&mut self, state: State) {
            self.r = self.bus.content[0].clone();
            self.g = self.bus.content[1].clone();
            self.b = self.bus.content[2].clone();
        }

        fn get_id(&self) -> String {
            self.id.to_string()
        }

        fn get_event_listeners(&self) {
            todo!()
        }

        fn get_bus(&mut self) -> &mut Bus {
            self.bus.borrow_mut()
        }

        fn set_memory_space(&mut self, addresses: Vec<String>) {
            self.addresses = addresses;
        }

        fn get_memory_width(&self) -> u8 {
            return 3;
        }

        fn get_window(&self) {
            let rgb_color = Style::new().color256((self.r+self.b+self.g)/3);
            println!("-----");
            println!("|    |");
            println!("| {} |", rgb_color.apply_to("O"));
            println!("|    |");
            println!("-----");
        }
    }
}