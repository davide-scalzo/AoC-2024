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
    height: i16,
    width: i16,
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

        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();
        Ok(Grid {
            height: height as i16,
            width: width as i16,
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

    fn check_for_x(&self, a: &Coord) -> (bool, bool) {
        let mut tlbr_diag = false;
        let mut trbl_diag = false;
        let tlbr = (
            self.m_set
                .contains(&a.get_neighbour_coord(&Direction::TopLeft)),
            self.m_set
                .contains(&a.get_neighbour_coord(&Direction::BottomRight)),
        );

        let trbl = (
            self.m_set
                .contains(&a.get_neighbour_coord(&Direction::TopRight)),
            self.m_set
                .contains(&a.get_neighbour_coord(&Direction::BottomLeft)),
        );

        // here we check for opposing S
        match tlbr {
            // M on top left, we check S on bottom right
            (true, false) => {
                if self
                    .s_set
                    .contains(&a.get_neighbour_coord(&Direction::BottomRight))
                {
                    tlbr_diag = true
                }
            }
            (false, true) => {
                if self
                    .s_set
                    .contains(&a.get_neighbour_coord(&Direction::TopLeft))
                {
                    tlbr_diag = true
                }
            }
            _ => {}
        }

        match trbl {
            // M on top left, we check S on bottom right
            (true, false) => {
                if self
                    .s_set
                    .contains(&a.get_neighbour_coord(&Direction::BottomLeft))
                {
                    trbl_diag = true
                }
            }
            (false, true) => {
                if self
                    .s_set
                    .contains(&a.get_neighbour_coord(&Direction::TopRight))
                {
                    trbl_diag = true
                }
            }
            _ => {}
        }

        (tlbr_diag, trbl_diag)
    }

    fn part2(&self) -> u64 {
        let mut count = 0;
        for a in self.a_set.iter() {
            if a.x == 0 || a.y == 0 || a.x == self.width - 1 || a.y == self.height - 1 {
                continue;
            }

            if let (true, true) = self.check_for_x(a) {
                count += 1;
            }
        }

        count
    }
}

pub fn part1(input: String) -> Result<u64, String> {
    let grid = Grid::from_str(&input)?;
    let count = grid.part1_split();
    Ok(count)
}

pub fn part2(input: String) -> Result<u64, String> {
    let grid = Grid::from_str(&input)?;
    let count = grid.part2();
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

    #[test]
    fn test_day_4_part_2() {
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
        let result = grid.part2();

        assert_eq!(result, 9);
    }
}
