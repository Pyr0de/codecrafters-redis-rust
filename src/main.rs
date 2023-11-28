use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};


fn handle_tcp_stream(mut stream: TcpStream) {
    loop {
        let mut buf = [0; 512];
        match stream.read(&mut buf) {
            Ok(_size) => {
                if let Err(_) = stream.write("+PONG\r\n".as_bytes()) {
                    break;
                }
            }
            _ => {break;}
        };
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        tokio::spawn(async move {
            match stream {
                Ok(_stream) => {
                    println!("accepted new connection");
                    handle_tcp_stream(_stream); 
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}
