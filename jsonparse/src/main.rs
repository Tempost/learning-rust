use std::fs::File;
use std::io::{BufReader, BufRead};

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
    address: Address
}

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String
}

fn main() -> Result<()>{
    let file = File::open("testjson.json")
        .expect("[ Error ] File Not Found!");

    let buf_file = BufReader::new(file);
    let mut content = String::from(r#""#);

    for line in buf_file.lines() {
        content.push_str(&String::from(line.unwrap())); 
    }

    let json: Person = serde_json::from_str(&content)?;

    println!("{} is {} years old, they can be reached at {}, they also live in {}",
        json.name, json.age, json.phones[0], json.address.city);

    Ok(())
}
