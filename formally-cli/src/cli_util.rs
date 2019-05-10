use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::io;

pub fn ask(toprint: String, accepts_empty: bool) -> String {
    let mut answer = String::new();
    while answer.len() == 0 {
        println!("{}", toprint);
        match io::stdin().read_line(&mut answer) {
            Ok(_) => { if accepts_empty { break; } }
            Err(error) => println!("error: {}", error),
        }
    }
    answer
}

pub fn save(serialized: String, filename: String) {
    match File::create(filename) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            serde_yaml::to_writer(writer, &serialized);
        }
        Err(e) => println!("error : {:?}", e),
    }
}

pub fn open(filename: String) -> String {
    match File::open(filename) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            return serde_yaml::from_reader(reader).unwrap();
        }
        Err(e) => println!("error : {:?}", e),
    }
    "".to_string()
}
