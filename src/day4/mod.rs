use std::{error::Error, fs};

pub fn get_day_4_input() -> Result<String, Box<dyn Error>> {
    let day_4_input = fs::read_to_string("./src/day4/input.txt")?;
    Ok(day_4_input)
}

pub fn day4(input: String) -> Result<u64, String> {
    Ok(0)
}
