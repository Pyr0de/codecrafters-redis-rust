use std::{io::{SeekFrom, Seek, Read}, fs::File, os::unix::fs::MetadataExt, collections::HashMap, u128};


pub fn read(file: &String) -> Option<HashMap<String, (String, Option<u128>)>>{
    let path = std::path::Path::new(file);

    if let Ok(mut f) = File::open(path){
        let _ = f.seek(SeekFrom::Start(0));
        let mut buf = vec![0; f.metadata().unwrap().size() as usize];
        f.read_exact(&mut buf).expect("Cannot read file");

        return Some(parse(buf))
    }

    None
}

fn parse(buf: Vec<u8>) -> HashMap<String, (String, Option<u128>)>{
    let mut current_idx = buf.iter().position(|&r| r == 0xfb).unwrap() + 3;
    let mut parsed: HashMap<String, (String, Option<u128>)> = HashMap::new();



    while buf[current_idx] != 0xff {
       // println!("{:X?} {}", buf[current_idx], buf[current_idx]);
        
        //expiry time
        let mut time: Option<u128> = None;
        if buf[current_idx] == 0xfc {
            current_idx+=1;
            time = Some(usize::from_le_bytes(buf[current_idx..current_idx+8].try_into().unwrap()) as u128);
            current_idx+=8;
        }
        let _type = buf[current_idx];
        current_idx += 1;

        let mut k: Option<String> = None;
        let mut v: Option<String> = None;
        for _ in 0..2 {
            let len = buf[current_idx] as usize;
            current_idx += 1;
            
            let slice = String::from_utf8(buf[current_idx..current_idx+len].to_owned()).unwrap();

            if k.is_none() {
                k = Some(slice);
            }else{
                v = Some(slice);
            }
            current_idx += len;
        }
        if k.is_some() {

            parsed.insert(k.unwrap(), (v.unwrap_or("".to_string()), time));
        }
        //key 
        
    }

    for (k,v) in parsed.clone() {
        println!("{k} {v:?}");
    }
    parsed
}

