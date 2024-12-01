use std::{collections::HashMap, fs};

pub fn day1() -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let mut left_list: Vec<u32> = Vec::with_capacity(1000);
    let mut right_list: Vec<u32> = Vec::with_capacity(1000);
    let input = fs::read_to_string("./src/day1/input.txt")?;
    let mut occurrences: HashMap<u32, u32> = HashMap::with_capacity(1000);

    for line in input.lines() {
        let mut line_split = line.split("   ");

        if let (Ok(left), Ok(right)) = (
            line_split.next().ok_or("end of file"),
            line_split.next().ok_or("end of file"),
        ) {
            let left_val = left.parse()?;
            let right_val = right.parse()?;
            left_list.push(left_val);
            right_list.push(right_val);

            *occurrences.entry(right_val).or_insert(0) += 1;
        }
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    let diff = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| l.max(r) - l.min(r))
        .sum();

    let similarity = left_list
        .iter()
        .filter_map(|&val| occurrences.get(&val).map(|&occ| val * occ))
        .sum();

    Ok((diff, similarity))
}
