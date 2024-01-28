use std::{collections::HashMap, time::{Duration, SystemTime}};

#[derive(Default)]
pub struct Database {
    db: HashMap<String, (String, Option<SystemTime>)>,
}

impl Database {

    pub fn get(&self, k:&String) -> Option<&String> {
        if let Some((v, expiry)) = self.db.get(k){
            if expiry.is_none() || expiry.unwrap() > SystemTime::now() {
                return Some(v);
            }
        }
        None
    }

    pub fn set(&mut self, k:String, v:String, exp_ms:Option<Duration>) {
        if let Some(expiry_ms) = exp_ms {
            
            self.db.insert(k, (v, Some(SystemTime::now() + expiry_ms))); 
        }else {
            self.db.insert(k, (v,None)); 
        }
    }

}
