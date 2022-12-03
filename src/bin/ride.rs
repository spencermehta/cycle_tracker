pub mod ride {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq)]
    pub struct Ride {
        pub length: f64,
    }

    pub struct RideStore {
        data: HashMap<u32, Ride>,
        current_id: u32,
    }

    impl RideStore {
        pub fn new() -> RideStore {
            RideStore {
                data: HashMap::new(),
                current_id: 0,
            }
        }

        pub fn add(&mut self, ride: Ride) -> u32 {
            let id = self.generate_id();
            self.data.insert(id, ride);
            id
        }

        fn generate_id(&mut self) -> u32 {
            self.current_id += 1;
            self.current_id
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ride::*;

    #[test]
    fn create_ride() {
        let mut store = RideStore::new();

        assert_eq!(1, store.add(Ride { length: 50.0 }));

        assert_eq!(2, store.add(Ride { length: 60.0 }));
    }
}
