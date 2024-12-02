use std::{error::Error, fs};

#[derive(Debug)]
struct Report {
    values: Vec<i32>,
}

#[derive(Debug)]
pub enum ReportError {
    TooHighDiff(i32),
    ZeroDiff,
    NotMonotonic(Vec<i32>),
}

impl Report {
    pub fn from_string(input: &str) -> Self {
        let values = input
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        Report { values }
    }

    // [5 4 2] 3 2 1
    // 5 [4 2 3] 2 1
    // 5 4 [2 3 2] 1
    // 5 4 2 [3 2 1]
    //
    // [4 1 2] 3 4
    // 4 [1 2 3] 4
    // 4 1 [2 3 4]
    // 4 1 2 3 4
    // 4 1 2 3 4
    //
    // [1 2 3] 4 0
    // 1 [2 3 4] 0
    // 1 2 [3 4 0]

    fn compare_within_bounds(a: i32, b: i32, allowance: i32) -> bool {
        let diff = (a - b).abs();
        if diff <= allowance {
            return true;
        }
        false
    }

    fn is_monotonic_trio(a: i32, b: i32, c: i32) -> bool {
        if a > b && b > c {
            return true;
        }
        if a < b && b < c {
            return true;
        }
        false
    }

    pub fn is_safe(&self) -> bool {
        let mut skips = 0;
        for (idx, item) in self.values.iter().enumerate() {
            let plus_one = self.values.get(idx + 1);
            let plus_two = self.values.get(idx + 2);

            match (plus_one, plus_two) {
                (Some(plus_one), Some(plus_two)) => {
                    if !Report::compare_within_bounds(*item, *plus_one, 3)
                        || !Report::compare_within_bounds(*plus_one, *plus_two, 3)
                        || !Report::is_monotonic_trio(*item, *plus_one, *plus_two)
                    {
                        skips += 1;
                    }
                }
                _ => break,
            }
        }

        if skips == 2 {
            println!("Values: {:?}", self.values)
        }
        true
    }
}

pub fn day2() -> Result<i32, Box<dyn Error>> {
    let file = fs::read_to_string("./src/day2/input.txt")?;
    let mut safe_reports = 0;

    for line in file.lines() {
        let report = Report::from_string(line);
        let is_safe = report.is_safe();
        if is_safe {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compare_within_bounds() {
        let result = Report::compare_within_bounds(3, 10, 4);
        assert!(!result);

        let result = Report::compare_within_bounds(3, 5, 3);
        assert!(result);

        let result = Report::compare_within_bounds(2, -1, 3);
        assert!(result);

        let result = Report::compare_within_bounds(2, -2, 3);
        assert!(!result);
    }

    #[test]
    fn is_monotonic_trio() {
        let result = Report::is_monotonic_trio(3, 10, 4);
        assert!(!result);

        let result = Report::is_monotonic_trio(3, 1, 4);
        assert!(!result);

        let result = Report::is_monotonic_trio(3, 4, 4);
        assert!(!result);

        let result = Report::is_monotonic_trio(-1, 1, 4);
        assert!(result);

        let result = Report::is_monotonic_trio(5, 4, 2);
        assert!(result);
    }

    #[test]
    fn pattern_a() {
        let report = Report::from_string("48 46 47 49 51 54 56");
        let result = report.is_safe();
        assert!(result);
    }

    #[test]
    fn pattern_b() {
        let report = Report::from_string("1 1 2 3 4 5");
        let result = report.is_safe();
        assert!(result);
    }

    #[test]
    fn pattern_c() {
        let report = Report::from_string("1 2 3 4 5 5");
        let result = report.is_safe();
        assert!(result);
    }

    #[test]
    fn pattern_d() {
        let report = Report::from_string("5 1 2 3 4 5");
        let result = report.is_safe();
        assert!(result);
    }

    #[test]
    fn pattern_e() {
        let report = Report::from_string("1 4 3 2 1");
        let result = report.is_safe();
        assert!(result);
    }

    #[test]
    fn pattern_f() {
        let report = Report::from_string("1 6 7 8 9");
        let result = report.is_safe();
        assert!(result);
    }

    #[test]
    fn pattern_g() {
        let report = Report::from_string("1 2 3 4 3");
        let result = report.is_safe();
        assert!(result);
    }

    #[test]
    fn pattern_h() {
        let report = Report::from_string("9 8 7 6 7");
        let result = report.is_safe();
        assert!(result);
    }

    #[test]
    fn pattern_j() {
        let report = Report::from_string("7 10 8 10 11");
        let result = report.is_safe();
        assert!(result);
    }

    #[test]
    fn pattern_k() {
        let report = Report::from_string("29 28 27 25 26 25 22 20");
        let result = report.is_safe();
        assert!(result);
    }

    #[test]
    fn pattern_i() {
        let report = Report::from_string("5 8 15 15 17");
        let result = report.is_safe();
        assert!(result);
    }
}
