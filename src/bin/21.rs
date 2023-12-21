use std::collections::HashSet;
use std::str::FromStr;

advent_of_code::solution!(21);

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

const COMPASS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, Eq, Hash, PartialEq)]
struct RepeatedGridPosition {
    x: i32,
    y: i32,
    position: usize,
}

#[derive(Debug, PartialEq)]
struct Garden {
    height: usize,
    width: usize,
    start: usize,
    rocks: Vec<bool>,
}

impl Garden {
    fn initial_position(&self) -> HashSet<usize> {
        let mut positions = HashSet::new();
        positions.insert(self.start);
        positions
    }

    fn initial_position_on_repeated_grid(&self) -> HashSet<RepeatedGridPosition> {
        let mut positions = HashSet::new();
        positions.insert(RepeatedGridPosition {
            x: 0,
            y: 0,
            position: self.start,
        });
        positions
    }

    fn step_in_direction(&self, position: usize, direction: &Direction) -> Option<usize> {
        let row = position / self.width;
        let col = position % self.width;

        match direction {
            Direction::North => position.checked_sub(self.width),
            Direction::East => {
                if (col + 1) < self.width {
                    Some(position + 1)
                } else {
                    None
                }
            }
            Direction::South => {
                if (row + 1) < self.height {
                    Some(position + self.width)
                } else {
                    None
                }
            }
            Direction::West => {
                if col > 0 {
                    Some(position - 1)
                } else {
                    None
                }
            }
        }
    }

    fn step_in_direction_on_repeated_grid(
        &self,
        from: &RepeatedGridPosition,
        direction: &Direction,
    ) -> RepeatedGridPosition {
        let row = from.position / self.width;
        let col = from.position % self.width;

        match direction {
            Direction::North => {
                if row > 0 {
                    RepeatedGridPosition {
                        x: from.x,
                        y: from.y,
                        position: from.position - self.width,
                    }
                } else {
                    RepeatedGridPosition {
                        x: from.x,
                        y: from.y - 1,
                        position: ((self.height - 1) * self.width) + col,
                    }
                }
            }
            Direction::East => {
                if (col + 1) < self.width {
                    RepeatedGridPosition {
                        x: from.x,
                        y: from.y,
                        position: from.position + 1,
                    }
                } else {
                    RepeatedGridPosition {
                        x: from.x + 1,
                        y: from.y,
                        position: (row * self.width),
                    }
                }
            }
            Direction::South => {
                if (row + 1) < self.height {
                    RepeatedGridPosition {
                        x: from.x,
                        y: from.y,
                        position: from.position + self.width,
                    }
                } else {
                    RepeatedGridPosition {
                        x: from.x,
                        y: from.y + 1,
                        position: col,
                    }
                }
            }
            Direction::West => {
                if col > 0 {
                    RepeatedGridPosition {
                        x: from.x,
                        y: from.y,
                        position: from.position - 1,
                    }
                } else {
                    RepeatedGridPosition {
                        x: from.x - 1,
                        y: from.y,
                        position: (self.width * row) + self.width - 1,
                    }
                }
            }
        }
    }

    fn step_from_positions(&self, positions: &HashSet<usize>) -> HashSet<usize> {
        let mut after = HashSet::new();

        for position in positions {
            for direction in COMPASS {
                if let Some(dest) = self.step_in_direction(*position, &direction) {
                    if !self.rocks[dest] {
                        after.insert(dest);
                    }
                }
            }
        }

        after
    }

    fn step_from_positions_on_repeated_grid(
        &self,
        positions: &HashSet<RepeatedGridPosition>,
    ) -> HashSet<RepeatedGridPosition> {
        let mut after = HashSet::new();

        for position in positions {
            for direction in COMPASS {
                let dest = self.step_in_direction_on_repeated_grid(position, &direction);
                if !self.rocks[dest.position] {
                    after.insert(dest);
                }
            }
        }

        after
    }

    fn total_reachable_in_steps(&self, steps: usize) -> usize {
        let mut positions = self.initial_position();
        for _ in 0..steps {
            positions = self.step_from_positions(&positions);
        }
        positions.len()
    }

    fn total_reachable_on_repeated_grid(&self, steps: usize) -> usize {
        let mut positions = self.initial_position_on_repeated_grid();
        for _ in 0..steps {
            positions = self.step_from_positions_on_repeated_grid(&positions);
        }
        positions.len()
    }
}

#[derive(Debug, PartialEq)]
struct ParseGardenError;

impl FromStr for Garden {
    type Err = ParseGardenError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rocks = Vec::new();
        let mut start: Result<usize, Self::Err> = Err(ParseGardenError);

        let mut width = 0;
        let mut height = 0;

        for (row, line) in input.lines().enumerate() {
            height = row + 1;
            width = width.max(line.len());

            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => rocks.push(true),
                    'S' => {
                        rocks.push(false);
                        start = Ok((row * width) + col);
                    }
                    '.' => rocks.push(false),
                    _ => return Err(ParseGardenError),
                }
            }
        }

        let start = start?;
        Ok(Self {
            height,
            width,
            start,
            rocks,
        })
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
        (row * 11) + col
    }

    fn example_garden() -> Garden {
        let mut rocks = vec![false; 11 * 11];
        rocks[position(1, 5)] = true;
        rocks[position(1, 6)] = true;
        rocks[position(1, 7)] = true;
        rocks[position(1, 9)] = true;
        rocks[position(2, 1)] = true;
        rocks[position(2, 2)] = true;
        rocks[position(2, 3)] = true;
        rocks[position(2, 5)] = true;
        rocks[position(2, 6)] = true;
        rocks[position(2, 9)] = true;
        rocks[position(3, 2)] = true;
        rocks[position(3, 4)] = true;
        rocks[position(3, 8)] = true;
        rocks[position(4, 4)] = true;
        rocks[position(4, 6)] = true;
        rocks[position(5, 1)] = true;
        rocks[position(5, 2)] = true;
        rocks[position(5, 6)] = true;
        rocks[position(5, 7)] = true;
        rocks[position(5, 8)] = true;
        rocks[position(5, 9)] = true;
        rocks[position(6, 1)] = true;
        rocks[position(6, 2)] = true;
        rocks[position(6, 5)] = true;
        rocks[position(6, 9)] = true;
        rocks[position(7, 7)] = true;
        rocks[position(7, 8)] = true;
        rocks[position(8, 1)] = true;
        rocks[position(8, 2)] = true;
        rocks[position(8, 4)] = true;
        rocks[position(8, 6)] = true;
        rocks[position(8, 7)] = true;
        rocks[position(8, 8)] = true;
        rocks[position(8, 9)] = true;
        rocks[position(9, 1)] = true;
        rocks[position(9, 2)] = true;
        rocks[position(9, 5)] = true;
        rocks[position(9, 6)] = true;
        rocks[position(9, 8)] = true;
        rocks[position(9, 9)] = true;

        Garden {
            height: 11,
            width: 11,
            start: position(5, 5),
            rocks,
        }
    }

    fn example_initial_position() -> HashSet<usize> {
        let mut positions = HashSet::new();
        positions.insert(position(5, 5));
        positions
    }

    fn example_positions_after_one_step() -> HashSet<usize> {
        let mut positions = HashSet::new();
        positions.insert(position(4, 5));
        positions.insert(position(5, 4));
        positions
    }

    fn example_positions_after_two_steps() -> HashSet<usize> {
        let mut positions = HashSet::new();
        positions.insert(position(3, 5));
        positions.insert(position(5, 5));
        positions.insert(position(5, 3));
        positions.insert(position(6, 4));
        positions
    }

    #[test]
    fn test_garden_initial_position() {
        let garden = example_garden();
        let initial = garden.initial_position();
        assert_eq!(initial, example_initial_position());
    }

    #[test]
    fn test_garden_step_from_positions() {
        let garden = example_garden();
        let initial = example_initial_position();

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
    fn test_total_reachable_on_repeated_grid() {
        let garden = example_garden();
        assert_eq!(garden.total_reachable_on_repeated_grid(6), 16);
        assert_eq!(garden.total_reachable_on_repeated_grid(50), 1594);
        assert_eq!(garden.total_reachable_on_repeated_grid(100), 6536);
        //       assert_eq!(garden.total_reachable_on_repeated_grid(5000), 16_733_044);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
