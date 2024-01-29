mod message;
mod database;

use message::Message;
use std::collections::HashMap;
use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};
use std::sync::{Arc, RwLock};

fn handle_tcp_stream(mut stream: TcpStream, database: Arc<RwLock<database::Database>>) {
    loop {
        let mut buf = vec![0;512];
        match stream.read(&mut buf) {
            Ok(_size) => {
                let str_buf = String::from_utf8(buf).unwrap_or("\r\n".to_string());
                if let Some(message) = Message::parse_request(str_buf) {
                    println!("{:?}", message);

                    if !message.handle(&mut stream, &database){
                        println!("error");
                        break;
                    } 
                    
                }else {
                    if stream.write(b"\r\n").is_err() {
                        break;
                    }
                }

            }
            _ => break
        };
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let database = Arc::new(RwLock::new(database::Database::new(get_args())));    
    for stream in listener.incoming() {
        let clone = database.clone();
        tokio::spawn(async move {
            match stream {
                Ok(stream) => {
                    
                    handle_tcp_stream(stream, clone); 
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}

pub fn get_args() -> HashMap<String, String>{
    let mut arguments:HashMap<String, String> = HashMap::new();
    let args:Vec<String> = std::env::args().collect();

    let mut temp: String = "".to_string();
    for (k,v) in args.iter().enumerate() {
        if k == 0 {
            continue;
        }
        if k % 2 == 0 {
            arguments.insert(temp, v.to_owned());
            temp = "".to_string();
        }else {
            let mut s = v.to_owned();
            if s.starts_with("--"){
                s = s[2..].to_string();
            }
            temp = s;
        }
    }

    arguments

}


