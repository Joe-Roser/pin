use std::collections::HashMap;

use bincode;
use prettytable::{Table, row};

// The Datastore
// Tags: convert the value to a struct to add tags
// maybe a created at??
pub struct Store {
    map: HashMap<String, String>,
}

impl Store {
    // Read in the store or make a fresh one
    pub fn init() -> Store {
        let map: HashMap<String, String> = if let Ok(file) = std::fs::File::open(get_pin_path()) {
            let reader = std::io::BufReader::new(file);
            bincode::decode_from_reader(reader, bincode::config::standard()).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Store { map: map }
    }

    // Write hashmap to file
    pub fn save(self) {
        let mut file = std::fs::File::create(get_pin_path()).expect(
            "Error: Could not write store to file. Any additions this session will be lost.",
        );
        let _ = bincode::encode_into_std_write(
            self.map.clone(),
            &mut file,
            bincode::config::standard(),
        )
        .unwrap();
    }

    // Validate the input and add it to the map
    pub fn add(&mut self, key: String, val: String) -> Result<(), String> {
        match self.map.insert(key, val) {
            None => Ok(()),
            Some(path) => Err(path),
        }
    }

    // Get the path for the matching alias
    pub fn get(&self, key: &String) -> Option<String> {
        self.map.get(key).cloned()
    }

    pub fn delete(&mut self, alias: String) -> Result<(), ()> {
        match self.map.remove(&alias) {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }

    //return all key value pairs
    pub fn list_all(&self) -> String {
        let mut table = Table::new();
        table.add_row(row!["Alias", "Path"]);

        self.map.iter().map(|(k, v)| row![k, v]).for_each(|r| {
            table.add_row(r);
        });

        table.to_string()
    }
}

// Gets the path of the _pins.store.bin
fn get_pin_path() -> String {
    format!("{}/.pin/store.bin", std::env::var("HOME").unwrap())
}
