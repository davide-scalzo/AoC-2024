use regex::Regex;
use std::{error::Error, fs, iter::Product, result::Result};

pub fn get_day_3_input() -> Result<String, Box<dyn Error>> {
    let day_3_input = fs::read_to_string("./src/day3/input.txt")?;
    Ok(day_3_input)
}

pub fn day3(input: String) -> Result<f64, String> {
    let regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();
    let sum = regex
        .captures_iter(&input)
        .filter_map(|cap| {
            let groups = (cap.get(1), cap.get(2));
            match groups {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            }
        })
        .map(|(a, b)| a.as_str().parse::<f64>().unwrap() * b.as_str().parse::<f64>().unwrap())
        .sum::<f64>();
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::day3;

    #[test]
    fn test_input() {
        let test_string =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        let result = day3(test_string).unwrap();

        assert_eq!(result, 161)
    }
}
