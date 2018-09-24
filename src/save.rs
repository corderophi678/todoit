use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::todo::Todo;

pub fn save(todos: Vec<Todo>, file: &str) -> Result<(), io::Error> {
    let mut f= File::create(file)?;
    let data = serde_json::to_string(&todos).unwrap();
    f.write_all(&data.as_bytes())?;
    Ok(())
}
