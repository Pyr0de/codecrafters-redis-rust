use std::collections::HashMap;

#[derive(Default)]
pub struct Database {
    db: HashMap<String, String>,
}

impl Database {

    pub fn get(&self, k:&String) -> Option<&String> {
        self.db.get(k)
    }

    pub fn set(&mut self, k:String, v:String) {
        self.db.insert(k, v);
    }
}
