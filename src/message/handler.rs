use std::{net::TcpStream, io::Write, sync::{Arc, RwLock}};

use crate::database::Database;

use super::Message;


impl Message {

    pub fn handle(self, stream: &mut TcpStream, database: &Arc<RwLock<Database>>) -> bool {
        let output_str = match self {
            Self::Ping => "+PONG\r\n".to_string(),
            Self::Echo(s) => format!("${}\r\n{s}\r\n", s.len()),
            Self::Get(k) => {
                if let Some(v) = database.read().unwrap().get(&k) {
                    format!("${}\r\n{v}\r\n", v.len())
                }else {
                    "$-1\r\n".to_string()
                }
            },
            Self::Set(k, v, expiry) => {
                database.write().unwrap().set(k, v, expiry);
                "+OK\r\n".to_string()
            },
            Self::Unknown(s) => format!("-Unknown command \'{s}\'\r\n"),
        };
        stream.write(output_str.as_bytes()).is_ok()
    }
}


