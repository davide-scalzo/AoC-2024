use std::{fs, error::Error, result::Result};

pub fn get_day_3_input() -> Result<String, Box<dyn Error>> {
    let day_3_input = fs::read_to_string("./src/day3/input.txt")?;
    Ok(day_3_input)
}

pub fn day3(input: String) -> Result<u64, String> {
    let mut total = 0;

    for c in input.chars() {

    }
    Ok(0)
}


#[cfg(test)]
mod test {
    use super::day3;

    #[test]
    fn test_input() {
        let test_string = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        let result = day3(test_string).unwrap();

        assert_eq!(result, 161)
    }
}
