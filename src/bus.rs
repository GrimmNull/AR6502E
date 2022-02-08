
pub mod bus {
    #[derive(Clone)]
    pub struct Bus {
        pub content: Vec<u8>
    }

    impl Bus {
        pub fn write_to_buss(&mut self, message: Vec<u8>) {
            self.content = message;
        }

        pub fn clear_bus(&mut self) {
            self.content = Vec::new();
        }
    }
}