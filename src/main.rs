use std::time::Instant;

use advent_of_code::day3::{day3, get_day_3_input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let day_3_input = get_day_3_input()?;
    // let day_3_input =
    // String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    let result = day3(day_3_input);
    let end = Instant::now();

    println!("Result: {:?}  Time:{:?}", result, end - now);
    Ok(())
}
