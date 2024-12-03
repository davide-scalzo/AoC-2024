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

    fn compare_within_bounds(a: &i32, b: &i32, allowance: i32) -> bool {
        let diff = (a - b).abs();
        if diff <= allowance {
            return true;
        }
        false
    }

    fn is_monotonic_trio(a: &i32, b: &i32, c: &i32) -> bool {
        if a > b && b > c {
            return true;
        }
        if a < b && b < c {
            return true;
        }
        false
    }

    fn check_trio(a: &i32, b: &i32, c: &i32) -> bool {
        if !Report::is_monotonic_trio(a, b, c) {
            return false;
        }
        if !Report::compare_within_bounds(a, b, 3) {
            return false;
        }

        if !Report::compare_within_bounds(b, c, 3) {
            return false;
        }

        true
    }

    pub fn is_safe(&self) -> bool {
        let mut skip_idx: Option<i32> = None;
        for (idx, a) in self.values.iter().enumerate() {
            let b = self.values.get(idx + 1);
            let c = self.values.get(idx + 2);

            match (b, c) {
                (Some(b), Some(c)) => {
                    if !Report::check_trio(a, b, c) {
                        if let Some(next) = self.values.get(idx + 3) {
                            // check if skipping the third item in the trio works
                            if Report::check_trio(a, b, next) {
                                let target_idx = Some(idx as i32 + 2);
                                if skip_idx.is_some() && skip_idx != target_idx {
                                    return false;
                                }
                                skip_idx = target_idx;
                                continue;
                            }

                            // check if skipping the second item in the trio works
                            if Report::check_trio(a, c, next) {
                                let target_idx = Some(idx as i32 + 1);
                                if skip_idx.is_some() && skip_idx != target_idx {
                                    return false;
                                }
                                skip_idx = target_idx;
                                continue;
                            }

                            // check if skipping the first item in the trio works
                            if Report::check_trio(b, c, next) {
                                let target_idx = Some(idx as i32);
                                if skip_idx.is_some() && skip_idx != target_idx {
                                    return false;
                                }
                                skip_idx = target_idx;
                                continue;
                            }
                        } else {
                            if let Some(skip_idx) = skip_idx {
                                if skip_idx == idx as i32 + 1 {
                                    return true;
                                }
                            }
                            return skip_idx.is_none();
                        }

                        skip_idx = Some(idx as i32)
                    }
                }
                _ => break,
            }
        }
        true
    }
}

pub fn get_day_2_input() -> Result<String, Box<dyn Error>> {
    let day_2_input = fs::read_to_string("./src/day2/input.txt")?;
    Ok(day_2_input)
}

pub fn day2(input: String) -> Result<i32, Box<dyn Error>> {
    let mut safe_reports = 0;

    for line in input.lines() {
        let report = Report::from_string(line);
        let is_safe = report.is_safe();
        if is_safe {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}
