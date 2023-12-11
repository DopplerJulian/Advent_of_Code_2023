use std::io::Error;
use std::io::Read;
use std::fs::File;

#[allow(unused)]
pub fn read_file(path: &str)-> Result<Vec<String>, Error>{
    let mut file = File::open(path)?;
    let mut text = String::new();

    file.read_to_string(&mut text)?;

    let strings = text.split("\n")
        .map(|s| s.to_owned())
        .collect();
    Ok(strings)
}

#[allow(unused)]
pub fn read_file_as_string(path: &str) -> Result<String, Error>{
    let mut file = File::open(path)?;
    let mut text = String::new();

    file.read_to_string(&mut text)?;

    Ok(text)
}