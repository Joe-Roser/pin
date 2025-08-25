use std::collections::HashMap;

use bincode;

pub struct Store {
    map: HashMap<String, String>,
}

impl Store {
    pub fn init() -> Store {
        let map: HashMap<String, String> = if let Ok(file) = std::fs::File::open(get_pin_path()) {
            let reader = std::io::BufReader::new(file);
            println!("Saved!!! Isnt that cool!!");
            bincode::decode_from_reader(reader, bincode::config::standard()).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Store { map: map }
    }

    // Write hashmap to file
    pub fn save(self) {
        let mut file =
            std::fs::File::create(get_pin_path()).expect("Could not write store to file");
        let _ = bincode::encode_into_std_write(self.map, &mut file, bincode::config::standard())
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
        // TODO:
        self.map.get(key).clone().map(|k| (k.clone()))
    }

    pub fn delete(&mut self, alias: String) -> Result<(), ()> {
        match self.map.remove(&alias) {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }

    //prints all key value pairs
    pub fn list_all(&self) {
        self.map.iter().for_each(|(k, v)| println!("{k}: {v}"));
    }
}

// Gets the path of the _pins.store.bin
fn get_pin_path() -> String {
    format!("{}/.pin/store.bin", std::env::var("HOME").unwrap())
}
