use std::fs;

pub fn read_file_to_string(filename: &str) -> Result<String, std::io::Error> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}
