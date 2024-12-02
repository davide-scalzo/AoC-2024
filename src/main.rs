use advent_of_code::day2::day2;

use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let result = day2();
    let end = Instant::now();

    println!("Result: {:?}", end - now);
    Ok(())
}
