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
