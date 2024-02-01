mod config;

use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}, path::Path};


#[derive(Default)]
pub struct Database {
    db: HashMap<String, (String, Option<u128>)>,
    config: HashMap<String, String>
}

impl Database {

    pub fn new(args: HashMap<String, String>) -> Self {
        let mut db: HashMap<String, (String, Option<u128>)> = HashMap::new();

        if args.contains_key("dir") && args.contains_key("dbfilename") {
            let dir = args.get("dir").unwrap();
            let dbfilename = args.get("dbfilename").unwrap();

            let dbfilename_path = Path::new(dir).join(dbfilename).to_str().unwrap().to_owned();
            
            if let Some(file_db) = config::read(dbfilename_path) {
                println!("Setting db");
                db = file_db;
            }
        }

        Database {
            db,
            config: args,
        }


    }

    pub fn keys(&self, pat: String) -> Vec<String>{
        if pat != "*".to_string(){
            return vec![];
        }
        self.db.iter()
            .filter(|(_k,(_v, t))| t.unwrap_or(0 as u128) > current_time_ms() || t.is_none())
            .map(|(k,_v)| k.clone())
            .collect()

       

    }
    
    pub fn get_config(&self, k:&String) -> Option<&String> {
        self.config.get(k)
    }

    pub fn get(&self, k:&String) -> Option<&String> {
        if let Some((v, expiry)) = self.db.get(k){
            if expiry.is_none() || expiry.unwrap() > current_time_ms() {
                return Some(v);
            }
        }
        None
    }

    pub fn set(&mut self, k:String, v:String, exp_ms:Option<u128>) {
        if let Some(expiry_ms) = exp_ms {
            
            self.db.insert(k, (v, Some(current_time_ms() + expiry_ms))); 
        }else {
            self.db.insert(k, (v,None)); 
        }
    }

}

fn current_time_ms() -> u128{
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}
