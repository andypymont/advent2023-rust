use std::str::FromStr;

advent_of_code::solution!(21);

const GRID_SIZE: usize = 131;
type Positions = [bool; GRID_SIZE * GRID_SIZE];

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn move_from_position(&self, pos: usize) -> Option<usize> {
        let row = pos / GRID_SIZE;
        let col = pos % GRID_SIZE;

        match self {
            Direction::North => pos.checked_sub(GRID_SIZE),
            Direction::East => {
                if (col + 1) < GRID_SIZE {
                    Some(pos + 1)
                } else {
                    None
                }
            }
            Direction::West => {
                if col > 0 {
                    Some(pos - 1)
                } else {
                    None
                }
            }
            Direction::South => {
                if (row + 1) < GRID_SIZE {
                    Some(pos + GRID_SIZE)
                } else {
                    None
                }
            }
        }
    }
}

const COMPASS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, PartialEq)]
struct Garden {
    start: usize,
    rocks: Positions,
}

impl Garden {
    fn initial_positions(&self) -> Positions {
        let mut positions = [false; GRID_SIZE * GRID_SIZE];
        positions[self.start] = true;
        positions
    }

    fn step_from_positions(&self, positions: &Positions) -> Positions {
        let mut after = [false; GRID_SIZE * GRID_SIZE];
        for position in positions
            .iter()
            .enumerate()
            .filter_map(|(pos, occ)| if *occ { Some(pos) } else { None })
        {
            for direction in COMPASS {
                if let Some(dest) = direction.move_from_position(position) {
                    if !self.rocks[dest] {
                        after[dest] = true;
                    }
                }
            }
        }
        after
    }

    fn total_reachable_in_steps(&self, steps: usize) -> usize {
        let mut positions = self.initial_positions();
        for _ in 0..steps {
            positions = self.step_from_positions(&positions);
        }
        positions.iter().filter(|x| **x).count()
    }
}

#[derive(Debug, PartialEq)]
struct ParseGardenError;

impl FromStr for Garden {
    type Err = ParseGardenError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rocks = [false; GRID_SIZE * GRID_SIZE];
        let mut start: Result<usize, Self::Err> = Err(ParseGardenError);

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => rocks[(row * GRID_SIZE) + col] = true,
                    'S' => start = Ok((row * GRID_SIZE) + col),
                    '.' => (),
                    _ => return Err(ParseGardenError),
                }
            }
        }

        let start = start?;
        Ok(Self { start, rocks })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(garden) = Garden::from_str(input) {
        Some(garden.total_reachable_in_steps(64))
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

    fn example_garden() -> Garden {
        let mut rocks = [false; GRID_SIZE * GRID_SIZE];

        for n in 0..=12 {
            rocks[position(n, 0)] = true;
            rocks[position(0, n)] = true;
            rocks[position(n, 12)] = true;
            rocks[position(12, n)] = true;
        }
        rocks[position(2, 6)] = true;
        rocks[position(2, 7)] = true;
        rocks[position(2, 8)] = true;
        rocks[position(2, 10)] = true;
        rocks[position(3, 2)] = true;
        rocks[position(3, 3)] = true;
        rocks[position(3, 4)] = true;
        rocks[position(3, 6)] = true;
        rocks[position(3, 7)] = true;
        rocks[position(3, 10)] = true;
        rocks[position(4, 3)] = true;
        rocks[position(4, 5)] = true;
        rocks[position(4, 9)] = true;
        rocks[position(5, 5)] = true;
        rocks[position(5, 7)] = true;
        rocks[position(6, 2)] = true;
        rocks[position(6, 3)] = true;
        rocks[position(6, 7)] = true;
        rocks[position(6, 8)] = true;
        rocks[position(6, 9)] = true;
        rocks[position(6, 10)] = true;
        rocks[position(7, 2)] = true;
        rocks[position(7, 3)] = true;
        rocks[position(7, 6)] = true;
        rocks[position(7, 10)] = true;
        rocks[position(8, 8)] = true;
        rocks[position(8, 9)] = true;
        rocks[position(9, 2)] = true;
        rocks[position(9, 3)] = true;
        rocks[position(9, 5)] = true;
        rocks[position(9, 7)] = true;
        rocks[position(9, 8)] = true;
        rocks[position(9, 9)] = true;
        rocks[position(9, 10)] = true;
        rocks[position(10, 2)] = true;
        rocks[position(10, 3)] = true;
        rocks[position(10, 6)] = true;
        rocks[position(10, 7)] = true;
        rocks[position(10, 9)] = true;
        rocks[position(10, 10)] = true;

        Garden {
            start: position(6, 6),
            rocks,
        }
    }

    fn example_initial_positions() -> Positions {
        let mut positions = [false; GRID_SIZE * GRID_SIZE];
        positions[position(6, 6)] = true;
        positions
    }

    fn example_positions_after_one_step() -> Positions {
        let mut positions = [false; GRID_SIZE * GRID_SIZE];
        positions[position(5, 6)] = true;
        positions[position(6, 5)] = true;
        positions
    }

    fn example_positions_after_two_steps() -> Positions {
        let mut positions = [false; GRID_SIZE * GRID_SIZE];
        positions[position(4, 6)] = true;
        positions[position(6, 6)] = true;
        positions[position(6, 4)] = true;
        positions[position(7, 5)] = true;
        positions
    }

    #[test]
    fn test_garden_initial_positions() {
        let garden = example_garden();
        let initial = garden.initial_positions();
        assert_eq!(initial, example_initial_positions());
    }

    #[test]
    fn test_garden_step_from_positions() {
        let garden = example_garden();
        let initial = example_initial_positions();

        let one = example_positions_after_one_step();
        assert_eq!(garden.step_from_positions(&initial), one);

        let two = example_positions_after_two_steps();
        assert_eq!(garden.step_from_positions(&one), two);
    }

    #[test]
    fn test_garden_from_str() {
        assert_eq!(
            Garden::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_garden()),
        );
    }

    #[test]
    fn test_total_reachable_in_steps() {
        let garden = example_garden();
        assert_eq!(garden.total_reachable_in_steps(6), 16);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
