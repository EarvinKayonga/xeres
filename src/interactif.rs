use failure::Error;
use std::io::{self, BufRead};

pub fn get_text() -> Result<String, Error> {
    println!("Text to encrypt ?");

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer)?;

    Ok(buffer.trim_right().to_string())
}

pub fn get_cipher_text() -> Result<String, Error> {
    println!("Encrypted text ?");

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;

    Ok(buffer.trim_right().to_string())
}

pub fn get_key() -> Result<String, Error> {
    println!("Encryption key ?");

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer)?;

    Ok(buffer.trim_right().to_string())
}
