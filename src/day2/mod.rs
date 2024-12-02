use std::{cmp::Ordering, error::Error, fs};

struct Report {
    values: Vec<i32>,
    direction: Option<Ordering>,
    last_value: Option<i32>,
    free_passes: i32,
}

#[derive(Debug)]
pub enum ReportError {
    TooHighDiff(i32),
    TooLowDiff(i32),
    NotMonotonic(Vec<i32>),
}

impl Report {
    pub fn from_string(input: &str, free_passes: i32) -> Self {
        let values = input
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        Report {
            values,
            free_passes,
            direction: None,
            last_value: None,
        }
    }

    pub fn from_values(values: Vec<i32>, free_passes: i32) -> Self {
        Report {
            values,
            free_passes,
            direction: None,
            last_value: None,
        }
    }

    fn use_free_pass(&mut self) -> Result<(), &str> {
        if self.free_passes == 0 {
            return Err("Too many infringements");
        };
        self.free_passes -= 1;
        Ok(())
    }

    fn is_safe_inner(&mut self) -> Result<bool, ReportError> {
        for v in self.values.clone() {
            if self.last_value.is_none() {
                self.last_value = Some(v);
                continue;
            }

            // Check diff
            let diff = (v - self.last_value.unwrap()).abs();

            if diff > 3 {
                match self.use_free_pass() {
                    Ok(_) => continue,
                    Err(_) => return Err(ReportError::TooHighDiff(diff)),
                }
            }

            if diff == 0 {
                match self.use_free_pass() {
                    Ok(_) => continue,
                    Err(_) => return Err(ReportError::TooLowDiff(diff)),
                }
            }

            let d = self.last_value.unwrap().cmp(&v);

            if self.direction.is_none() {
                self.direction = Some(d);
                self.last_value = Some(v);
                continue;
            }

            if !self.direction.unwrap().eq(&d) {
                match self.use_free_pass() {
                    Ok(_) => continue,
                    Err(_) => return Err(ReportError::NotMonotonic(self.values.clone())),
                }
            }

            self.direction = Some(d);
            self.last_value = Some(v);
        }
        Ok(true)
    }

    pub fn is_safe(&self) -> Result<(), &str> {
        let mut is_safe = false;
        for (idx, _) in self.values.iter().enumerate() {
            let mut new_values = self.values.clone();
            new_values.remove(idx);

            let mut report = Report::from_values(new_values, 0);
            match report.is_safe_inner() {
                Ok(true) => is_safe = true,
                Ok(false) => {}
                Err(_) => {}
            }
        }
        match is_safe {
            true => Ok(()),
            false => Err("Not safe"),
        }
    }
}

#[inline]
pub fn day2() -> Result<i32, Box<dyn Error>> {
    let file = fs::read_to_string("./src/day2/input.txt")?;
    let mut safe_reports = 0;

    for line in file.lines() {
        let report = Report::from_string(line, 1);
        if report.is_safe().is_ok() {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_removal_of_first_item() {
        let report = Report::from_string("7 2 3 5 6", 1);
        let _ = report.is_safe();
    }

    #[test]
    fn test_removal_of_previous_item() {
        let report = Report::from_string("28 27 25 26 25 24", 1);
        let _ = report.is_safe();
    }

    #[test]
    fn test_removal_of_last_item() {
        let report = Report::from_string("28 27 26 25 20", 1);
        let _ = report.is_safe();
    }
}
