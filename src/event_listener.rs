

pub mod event_listener {

    #[derive(Clone)]
    pub enum EventType {
        Page,
        Address,
        Addresses
    }

    #[derive(Clone)]
    pub struct EventListener {
        pub id: i32,
        pub device_id: String,
        pub device_bus: u8,
        pub event_type: EventType,
        pub page: u8,
        pub address: String,
        pub addresses: Vec<String>
    }

    impl EventListener {
        pub fn get_page_event_listener(id: i32, device_id: String, device_bus: u8, event_type: EventType, page: u8) -> EventListener {
            return EventListener {
                id,
                device_id,
                device_bus,
                event_type,
                page,
                address: "".to_string(),
                addresses: Vec::new()
            }
        }

        pub fn get_address_event_listener(id: i32, device_id: String, device_bus: u8, event_type: EventType, address: String) -> EventListener {
            return EventListener {
                id,
                device_id,
                device_bus,
                event_type,
                page: 0,
                address,
                addresses: Vec::new()
            }
        }

        pub fn get_addresses_event_listener(id: i32, device_id: String, device_bus: u8, event_type: EventType, addresses: Vec<String>) -> EventListener {
            return EventListener {
                id,
                device_id,
                device_bus,
                event_type,
                page: 0,
                address: "".to_string(),
                addresses
            }
        }
    }
}