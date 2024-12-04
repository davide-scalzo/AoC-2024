use std::{collections::HashSet, error::Error, fs, str::FromStr};

use strum::{EnumIter, IntoEnumIterator};

pub fn get_day_4_input() -> Result<String, Box<dyn Error>> {
    let day_4_input = fs::read_to_string("./src/day4/input.txt")?;
    Ok(day_4_input)
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: i16,
    y: i16,
}

#[derive(Debug, EnumIter)]
enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Coord {
    fn get_neighbour_coord(&self, d: &Direction) -> Coord {
        match d {
            Direction::TopLeft => Coord {
                x: self.x - 1,
                y: self.y - 1,
            },
            Direction::Top => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::TopRight => Coord {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Direction::BottomLeft => Coord {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::Bottom => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::BottomRight => Coord {
                x: self.x + 1,
                y: self.y + 1,
            },
        }
    }
}

#[derive(Debug)]
struct Grid {
    x_set: HashSet<Coord>,
    m_set: HashSet<Coord>,
    a_set: HashSet<Coord>,
    s_set: HashSet<Coord>,
}

impl FromStr for Grid {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut x_set: HashSet<Coord> = HashSet::new();
        let mut m_set: HashSet<Coord> = HashSet::new();
        let mut a_set: HashSet<Coord> = HashSet::new();
        let mut s_set: HashSet<Coord> = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    'X' => {
                        x_set.insert(Coord {
                            x: x as i16,
                            y: y as i16,
                        });
                    }
                    'M' => {
                        m_set.insert(Coord {
                            x: x as i16,
                            y: y as i16,
                        });
                    }
                    'A' => {
                        a_set.insert(Coord {
                            x: x as i16,
                            y: y as i16,
                        });
                    }
                    'S' => {
                        s_set.insert(Coord {
                            x: x as i16,
                            y: y as i16,
                        });
                    }
                    _ => {}
                }
            }
        }
        Ok(Grid {
            x_set,
            m_set,
            a_set,
            s_set,
        })
    }
}

impl Grid {
    fn part1_split(&self) -> u64 {
        let mut count = 0;
        for x in self.x_set.iter() {
            for d in Direction::iter() {
                if self.check_direction(&d, x, &self.m_set, 'M') {
                    count += 1
                }
            }
        }
        count
    }
    fn check_direction(&self, d: &Direction, c: &Coord, set: &HashSet<Coord>, depth: char) -> bool {
        let step_coordinates = c.get_neighbour_coord(d);
        match (set.get(&step_coordinates), depth) {
            (None, _) => false,
            (Some(_), 'M') => self.check_direction(d, &step_coordinates, &self.a_set, 'A'),
            (Some(_), 'A') => self.check_direction(d, &step_coordinates, &self.s_set, 'S'),
            (Some(_), 'S') => true,
            (_, _) => false,
        }
        // if set.get(&c.get_neighbour_coord(d)).is_some() {
        //     return self.check_direction(d, c, set, depth + 1);
        // }
    }
}

pub fn part2() {}

pub fn day4(input: String) -> Result<u64, String> {
    let grid = Grid::from_str(&input)?;
    let count = grid.part1_split();
    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_4_part_1() {
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
            "#;
        let grid = Grid::from_str(input).unwrap();
        let result = grid.part1_split();

        assert_eq!(result, 18);
    }
}
