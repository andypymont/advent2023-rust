use std::collections::HashMap;
use std::str::FromStr;

advent_of_code::solution!(14);

const GRID_SIZE: usize = 100;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Rock {
    Empty,
    Rounded,
    Cube,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Dish {
    grid: [Rock; GRID_SIZE * GRID_SIZE],
}

impl Dish {
    fn load_after_cycles(self, cycles: u32) -> usize {
        let mut visited = HashMap::new();
        let mut state = self;

        for cycle in 1..=cycles {
            state = state.tilt_cycle();

            if let Some(first_visited) = visited.get(&state) {
                let repeat_length = cycle - first_visited;
                let remaining = (cycles - cycle) % repeat_length;
                for _ in 0..remaining {
                    state = state.tilt_cycle();
                }
                break;
            }

            visited.insert(state, cycle);
        }

        state.load_on_north_support_beams()
    }

    fn load_on_north_support_beams(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(pos, rock)| {
                if rock == &Rock::Rounded {
                    GRID_SIZE - (pos / GRID_SIZE)
                } else {
                    0
                }
            })
            .sum()
    }

    fn roll_east(&self) -> Self {
        let mut rolled = [Rock::Empty; GRID_SIZE * GRID_SIZE];

        for row in 0..GRID_SIZE {
            let mut limit = GRID_SIZE - 1;
            for col in (0..GRID_SIZE).rev() {
                match self.grid[(row * GRID_SIZE) + col] {
                    Rock::Empty => (),
                    Rock::Cube => {
                        limit = col.saturating_sub(1);
                        rolled[(row * GRID_SIZE) + col] = Rock::Cube;
                    }
                    Rock::Rounded => {
                        rolled[(row * GRID_SIZE) + limit] = Rock::Rounded;
                        limit = limit.saturating_sub(1);
                    }
                }
            }
        }

        Dish { grid: rolled }
    }

    fn roll_north(&self) -> Self {
        let mut rolled = [Rock::Empty; GRID_SIZE * GRID_SIZE];

        for col in 0..GRID_SIZE {
            let mut limit = 0;
            for row in 0..GRID_SIZE {
                match self.grid[(row * GRID_SIZE) + col] {
                    Rock::Empty => (),
                    Rock::Cube => {
                        limit = row + 1;
                        rolled[(row * GRID_SIZE) + col] = Rock::Cube;
                    }
                    Rock::Rounded => {
                        rolled[(limit * GRID_SIZE) + col] = Rock::Rounded;
                        limit += 1;
                    }
                }
            }
        }

        Dish { grid: rolled }
    }

    fn roll_south(&self) -> Self {
        let mut rolled = [Rock::Empty; GRID_SIZE * GRID_SIZE];

        for col in 0..GRID_SIZE {
            let mut limit = GRID_SIZE - 1;
            for row in (0..GRID_SIZE).rev() {
                match self.grid[(row * GRID_SIZE) + col] {
                    Rock::Empty => (),
                    Rock::Cube => {
                        limit = row.saturating_sub(1);
                        rolled[(row * GRID_SIZE) + col] = Rock::Cube;
                    }
                    Rock::Rounded => {
                        rolled[(limit * GRID_SIZE) + col] = Rock::Rounded;
                        limit = limit.saturating_sub(1);
                    }
                }
            }
        }

        Dish { grid: rolled }
    }

    fn roll_west(&self) -> Self {
        let mut rolled = [Rock::Empty; GRID_SIZE * GRID_SIZE];

        for row in 0..GRID_SIZE {
            let mut limit = 0;
            for col in 0..GRID_SIZE {
                match self.grid[(row * GRID_SIZE) + col] {
                    Rock::Empty => (),
                    Rock::Cube => {
                        limit = col + 1;
                        rolled[(row * GRID_SIZE) + col] = Rock::Cube;
                    }
                    Rock::Rounded => {
                        rolled[(row * GRID_SIZE) + limit] = Rock::Rounded;
                        limit += 1;
                    }
                }
            }
        }

        Dish { grid: rolled }
    }

    fn tilt_cycle(&self) -> Self {
        self.roll_north().roll_west().roll_south().roll_east()
    }
}

#[derive(Debug, PartialEq)]
struct ParseDishError;

impl TryFrom<char> for Rock {
    type Error = ParseDishError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Cube),
            'O' => Ok(Self::Rounded),
            _ => Err(ParseDishError),
        }
    }
}

impl FromStr for Dish {
    type Err = ParseDishError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = [Rock::Empty; GRID_SIZE * GRID_SIZE];

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let rock = ch.try_into()?;
                grid[(row * GRID_SIZE) + col] = rock;
            }
        }

        Ok(Dish { grid })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(dish) = Dish::from_str(input) {
        Some(dish.roll_north().load_on_north_support_beams())
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    if let Ok(dish) = Dish::from_str(input) {
        Some(dish.load_after_cycles(1_000_000_000))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(row: usize, col: usize) -> usize {
        (row * GRID_SIZE) + col
    }

    fn example() -> Dish {
        let mut grid = [Rock::Empty; GRID_SIZE * GRID_SIZE];

        grid[position(0, 0)] = Rock::Rounded;
        grid[position(0, 5)] = Rock::Cube;
        grid[position(1, 0)] = Rock::Rounded;
        grid[position(1, 2)] = Rock::Rounded;
        grid[position(1, 3)] = Rock::Rounded;
        grid[position(1, 4)] = Rock::Cube;
        grid[position(1, 9)] = Rock::Cube;
        grid[position(2, 5)] = Rock::Cube;
        grid[position(2, 6)] = Rock::Cube;
        grid[position(3, 0)] = Rock::Rounded;
        grid[position(3, 1)] = Rock::Rounded;
        grid[position(3, 3)] = Rock::Cube;
        grid[position(3, 4)] = Rock::Rounded;
        grid[position(3, 9)] = Rock::Rounded;
        grid[position(4, 1)] = Rock::Rounded;
        grid[position(4, 7)] = Rock::Rounded;
        grid[position(4, 8)] = Rock::Cube;
        grid[position(5, 0)] = Rock::Rounded;
        grid[position(5, 2)] = Rock::Cube;
        grid[position(5, 5)] = Rock::Rounded;
        grid[position(5, 7)] = Rock::Cube;
        grid[position(5, 9)] = Rock::Cube;
        grid[position(6, 2)] = Rock::Rounded;
        grid[position(6, 5)] = Rock::Cube;
        grid[position(6, 6)] = Rock::Rounded;
        grid[position(6, 9)] = Rock::Rounded;
        grid[position(7, 7)] = Rock::Rounded;
        grid[position(8, 0)] = Rock::Cube;
        grid[position(8, 5)] = Rock::Cube;
        grid[position(8, 6)] = Rock::Cube;
        grid[position(8, 7)] = Rock::Cube;
        grid[position(9, 0)] = Rock::Cube;
        grid[position(9, 1)] = Rock::Rounded;
        grid[position(9, 2)] = Rock::Rounded;
        grid[position(9, 5)] = Rock::Cube;

        Dish { grid }
    }

    fn example_rolled_north() -> Dish {
        let mut grid = [Rock::Empty; GRID_SIZE * GRID_SIZE];

        grid[position(0, 0)] = Rock::Rounded;
        grid[position(0, 1)] = Rock::Rounded;
        grid[position(0, 2)] = Rock::Rounded;
        grid[position(0, 3)] = Rock::Rounded;
        grid[position(0, 5)] = Rock::Cube;
        grid[position(0, 7)] = Rock::Rounded;
        grid[position(1, 0)] = Rock::Rounded;
        grid[position(1, 1)] = Rock::Rounded;
        grid[position(1, 4)] = Rock::Cube;
        grid[position(1, 9)] = Rock::Cube;
        grid[position(2, 0)] = Rock::Rounded;
        grid[position(2, 1)] = Rock::Rounded;
        grid[position(2, 4)] = Rock::Rounded;
        grid[position(2, 5)] = Rock::Cube;
        grid[position(2, 6)] = Rock::Cube;
        grid[position(2, 9)] = Rock::Rounded;
        grid[position(3, 0)] = Rock::Rounded;
        grid[position(3, 3)] = Rock::Cube;
        grid[position(3, 5)] = Rock::Rounded;
        grid[position(3, 6)] = Rock::Rounded;
        grid[position(4, 8)] = Rock::Cube;
        grid[position(5, 2)] = Rock::Cube;
        grid[position(5, 7)] = Rock::Cube;
        grid[position(5, 9)] = Rock::Cube;
        grid[position(6, 2)] = Rock::Rounded;
        grid[position(6, 5)] = Rock::Cube;
        grid[position(6, 7)] = Rock::Rounded;
        grid[position(6, 9)] = Rock::Rounded;
        grid[position(7, 2)] = Rock::Rounded;
        grid[position(8, 0)] = Rock::Cube;
        grid[position(8, 5)] = Rock::Cube;
        grid[position(8, 6)] = Rock::Cube;
        grid[position(8, 7)] = Rock::Cube;
        grid[position(9, 0)] = Rock::Cube;
        grid[position(9, 5)] = Rock::Cube;

        Dish { grid }
    }

    #[test]
    fn test_parse_dish() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse::<Dish>(),
            Ok(example()),
        );
    }

    #[test]
    fn test_parse_dish_roll_north() {
        assert_eq!(example().roll_north(), example_rolled_north());
    }

    #[test]
    fn test_load_on_support_beams() {
        assert_eq!(example_rolled_north().load_on_north_support_beams(), 1756);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1756));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1255));
    }
}
