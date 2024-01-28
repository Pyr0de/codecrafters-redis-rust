mod parser;
mod handler;

#[derive(Debug, PartialEq)]
pub enum Message {
    Ping,
    Echo(String),
    Get(String),
    Set(String, String),
    Unknown(String),
}


#[cfg(test)]
mod message_test {
   
    #[test]
    fn str_to_message() {
        let t1 = super::Message::parse_request("*1\r\n$4\r\nping\r\n".to_string());
        assert_eq!(t1, Some(super::Message::Ping));

        let t2 = super::Message::parse_request("*2\r\n$4\r\nECHO\r\n$3\r\nhey$ hello& 12&\r\n".to_string());
        assert_eq!(t2, Some(super::Message::Echo("hey$ hello& 12&".to_string())));

        let t3 = super::Message::parse_request("*2\r\n$3\r\nasd\r\n$2\r\nas\r\n".to_string());
        assert_eq!(t3, Some(super::Message::Unknown("asd".to_string())));

        let t4 = super::Message::parse_request("\r\n".to_string());
        assert_eq!(t4, None);
    }
}
