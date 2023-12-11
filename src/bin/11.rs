use std::str::FromStr;

advent_of_code::solution!(11);

const GRID_SIZE: usize = 140;

#[derive(Debug, PartialEq)]
struct StarMap {
    stars: Vec<usize>,
    empty_rows: [bool; GRID_SIZE],
    empty_cols: [bool; GRID_SIZE],
}

impl StarMap {
    fn galactic_distance(&self, first: usize, second: usize) -> usize {
        let x: usize = {
            let (first, second) = {
                let first = first % GRID_SIZE;
                let second = second % GRID_SIZE;
                if first > second {
                    (second, first)
                } else {
                    (first, second)
                }
            };
            ((first + 1)..=second)
                .map(|x| if self.empty_cols[x] { 2 } else { 1 })
                .sum()
        };
        let y: usize = {
            let (first, second) = {
                let first = first / GRID_SIZE;
                let second = second / GRID_SIZE;
                if first > second {
                    (second, first)
                } else {
                    (first, second)
                }
            };
            ((first + 1)..=second)
                .map(|y| if self.empty_rows[y] { 2 } else { 1 })
                .sum()
        };
        x + y
    }

    fn total_galactic_distance(&self) -> usize {
        self.stars
            .iter()
            .enumerate()
            .flat_map(|(ix, first)| {
                self.stars[ix + 1..]
                    .iter()
                    .map(|second| self.galactic_distance(*first, *second))
            })
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseStarMapError;

impl FromStr for StarMap {
    type Err = ParseStarMapError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut stars = Vec::new();
        let mut empty_rows = [true; GRID_SIZE];
        let mut empty_cols = [true; GRID_SIZE];

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    empty_rows[row] = false;
                    empty_cols[col] = false;
                    stars.push((row * GRID_SIZE) + col);
                }
            }
        }

        Ok(StarMap {
            stars,
            empty_rows,
            empty_cols,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(starmap) = StarMap::from_str(input) {
        Some(starmap.total_galactic_distance())
    } else {
        None
    }
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(row: usize, col: usize) -> usize {
        (row * GRID_SIZE) + col
    }

    fn example_starmap() -> StarMap {
        let stars = vec![
            position(0, 3),
            position(1, 7),
            position(2, 0),
            position(4, 6),
            position(5, 1),
            position(6, 9),
            position(8, 7),
            position(9, 0),
            position(9, 4),
        ];

        let mut empty_rows = [true; GRID_SIZE];
        empty_rows[0] = false;
        empty_rows[1] = false;
        empty_rows[2] = false;
        empty_rows[4] = false;
        empty_rows[5] = false;
        empty_rows[6] = false;
        empty_rows[8] = false;
        empty_rows[9] = false;

        let mut empty_cols = [true; GRID_SIZE];
        empty_cols[0] = false;
        empty_cols[1] = false;
        empty_cols[3] = false;
        empty_cols[4] = false;
        empty_cols[6] = false;
        empty_cols[7] = false;
        empty_cols[9] = false;

        StarMap {
            stars,
            empty_rows,
            empty_cols,
        }
    }

    #[test]
    fn test_parse_starmap() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(input.parse(), Ok(example_starmap()),);
    }

    #[test]
    fn test_galactic_distance() {
        let starmap = example_starmap();

        for (first_ix, second_ix, expected) in [(0, 6, 15), (2, 5, 17), (7, 8, 5)] {
            let first = starmap.stars[first_ix];
            let second = starmap.stars[second_ix];
            assert_eq!(
                starmap.galactic_distance(first, second),
                expected,
                "galactic_distance between star {first_ix} and star {second_ix} is {expected}"
            );
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
