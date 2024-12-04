use std::time::Instant;

use advent_of_code::day4::{day4, get_day_4_input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let day_4_input = get_day_4_input()?;
    let result = day4(day_4_input);
    let end = Instant::now();

    println!("Result: {:?}  Time:{:?}", result, end - now);
    Ok(())
}
