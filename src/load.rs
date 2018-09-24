use std::fs::File;
use std::io;
use std::io::Read;

use crate::todo::Todo;

pub fn load(data: &str) -> Result<Vec<Todo>, io::Error> {
    let mut s = String::new();
    File::open(data)?.read_to_string(&mut s)?;
    let deserialized: Vec<Todo> = serde_json::from_str(&s).unwrap();
    Ok(deserialized)
}
