use std::error::Error;
use std::fs::read_to_string;

pub fn read_characters(filename: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let chars = read_to_string(filename)?
        .lines()
        .map(|line| line.to_string().into_bytes())
        .collect();

    Ok(chars)
}
