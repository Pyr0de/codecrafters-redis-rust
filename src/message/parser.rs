use std::time::Duration;

use super::Message;

impl Message {
    pub fn parse_request(input: String) -> Option<Self> {

        let parsed = parse(input);

        if parsed.len() == 0 {
            return None
        }

        return Some(match parsed[0].to_lowercase().as_str() {
            "ping" => Self::Ping,
            "echo" => Self::Echo(parsed[1].to_owned()),
            "get" => Self::Get(parsed[1].to_owned()),
            "set" => {
                let mut expiry: Option<Duration> = None;

                if parsed.get(4).is_some() && parsed.get(3).unwrap() == "px" {
                    let expiry_ms_res = parsed.get(4).unwrap().parse::<u64>();
                    
                    if let Ok(expiry_ms) = expiry_ms_res {
                        expiry = Some(Duration::from_millis(expiry_ms));
                    }else {
                        return Some(Self::Unknown("invalid time".to_string()))
                    }
                }

                Self::Set(parsed[1].to_owned(), parsed[2].to_owned(), expiry)
            },

            _ => Self::Unknown(parsed[0].to_owned())
        })

    }
    
}

fn parse(input:String) -> Vec<String> {

    let mut ret_vec:Vec<String> = vec![];
    let mut n = 0;
    for i in input.split("\r\n") {
        if n%2 == 0 && n != 0 {
            ret_vec.push(i.to_string());
        }
        n += 1;
    }

    ret_vec
}
