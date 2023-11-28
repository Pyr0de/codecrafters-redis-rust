use std::{net::{TcpListener, TcpStream}, io::{Write}};

fn handle_tcp_stream(mut stream: TcpStream) {
    let response = "+PONG\r\n";
    
    stream.write(&response.as_bytes()).expect("Error writing");
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_tcp_stream(_stream); 
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
