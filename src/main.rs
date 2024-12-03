use advent_of_code::day2::{day2, get_day_2_input};

use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let day_2_input = get_day_2_input()?;
    let result = day2(day_2_input);
    let end = Instant::now();

    println!("Result: {:?}  Time:{:?}", result, end - now);
    Ok(())
}
