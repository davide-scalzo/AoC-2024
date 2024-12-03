use regex::Regex;
use std::{error::Error, fs, result::Result};

pub fn get_day_3_input() -> Result<String, Box<dyn Error>> {
    let day_3_input = fs::read_to_string("./src/day3/input.txt")?;
    Ok(day_3_input)
}

pub fn day3(input: String) -> Result<f64, String> {
    let mut is_collecting = true;
    let regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)|(don't)|(do)").unwrap();
    let sum = regex
        .captures_iter(&input)
        .filter_map(|cap| {
            let groups = (
                cap.get(1),
                cap.get(2),
                cap.get(3),
                cap.get(4),
                is_collecting,
            );
            match groups {
                (Some(a), Some(b), None, None, true) => Some((a, b)),
                (None, None, Some(_), None, _) => {
                    is_collecting = false;
                    None
                }
                (None, None, None, Some(_), false) => {
                    is_collecting = true;
                    None
                }
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
        let test_string = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );

        let result = day3(test_string).unwrap();

        assert_eq!(result, 48.0)
    }
}
