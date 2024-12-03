use std::time::Instant;

use advent_of_code::day3::{day3, get_day_3_input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let day_3_input = get_day_3_input()?;
    let result = day3(day_3_input);
    let end = Instant::now();

    println!("Result: {:?}  Time:{:?}", result, end - now);
    Ok(())
}
