use std::{net::TcpStream, io::Write};

use super::Message;

impl Message {

    pub fn handle(self, stream: &mut TcpStream) -> bool {
        let output_str = match self {
            Self::Ping => "+PONG\r\n".to_string(),
            Self::Echo(s) => format!("${}\r\n{s}\r\n", s.len()),
            Self::Unknown(s) => format!("-Unknown command \'{s}\'\r\n"),
        };
        stream.write(output_str.as_bytes()).is_ok()
    }
}


