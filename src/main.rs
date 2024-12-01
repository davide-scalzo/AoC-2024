use std::time::Instant;

use advent_of_code::day1::day1a;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let (diff, similarity) = day1a()?;
    let end = Instant::now();
    println!(
        "Day1: Diff {:?} / Similarity {:?} - Time: {:?}",
        diff,
        similarity,
        end - start
    );
    Ok(())
}
