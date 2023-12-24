use std::collections::HashSet;
use std::str::FromStr;

advent_of_code::solution!(21);

#[derive(Debug, PartialEq)]
struct ParseGridError;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(i64, i64);

impl Point {
    fn neighbour_in_direction(self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Self(self.0 - 1, self.1),
            Direction::East => Self(self.0, self.1 + 1),
            Direction::South => Self(self.0 + 1, self.1),
            Direction::West => Self(self.0, self.1 - 1),
        }
    }

    fn neighbours(self) -> impl Iterator<Item = Self> {
        COMPASS
            .iter()
            .map(move |direction| self.neighbour_in_direction(direction))
    }
}

#[derive(Debug, PartialEq)]
struct Grid {
    start: Point,
    size: i64,
    rocks: HashSet<Point>,
}

impl Grid {
    fn initial_state(&self) -> HashSet<Point> {
        let mut state = HashSet::new();
        state.insert(self.start);
        state
    }

    fn point_in_bounds(&self, point: &Point) -> bool {
        point.0 >= 0 && point.0 < self.size && point.1 >= 0 && point.1 < self.size
    }

    fn reachable_point(&self, point: &Point, repeating_grid: bool) -> bool {
        if repeating_grid {
            let bounded_point = Point(point.0.rem_euclid(self.size), point.1.rem_euclid(self.size));
            !self.rocks.contains(&bounded_point)
        } else {
            self.point_in_bounds(point) & !self.rocks.contains(point)
        }
    }

    fn step(&self, state: &HashSet<Point>, repeating_grid: bool) -> HashSet<Point> {
        state
            .iter()
            .flat_map(|pt| pt.neighbours())
            .filter(|pt| self.reachable_point(pt, repeating_grid))
            .collect()
    }

    fn reachable_in_steps(&self, steps: usize, repeating_grid: bool) -> usize {
        let mut reached = self.initial_state();
        for _ in 0..steps {
            reached = self.step(&reached, repeating_grid);
        }
        reached.len()
    }
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rocks = HashSet::new();
        let mut height = 0;
        let mut width = 0;
        let mut start = Err(ParseGridError);

        for (row, line) in input.lines().enumerate() {
            let row = i64::try_from(row).map_err(|_| ParseGridError)?;
            height = row + 1;
            width = width.max(line.len());

            for (col, ch) in line.chars().enumerate() {
                let col = i64::try_from(col).map_err(|_| ParseGridError)?;
                let pt = Point(row, col);
                match ch {
                    'S' => start = Ok(pt),
                    '#' => {
                        rocks.insert(pt);
                    }
                    '.' => (),
                    _ => return Err(ParseGridError),
                }
            }
        }

        let width = i64::try_from(width).map_err(|_| ParseGridError)?;

        if height == 0 || width == 0 || height != width {
            Err(ParseGridError)
        } else {
            let start = start?;
            Ok(Self {
                rocks,
                size: height,
                start,
            })
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(grid) = Grid::from_str(input) {
        Some(grid.reachable_in_steps(64, false))
    } else {
        None
    }
}

fn quadratic_formula(sample1: usize, sample2: usize, sample3: usize, n: usize) -> usize {
    let a = (sample3 - (2 * sample2) + sample1) / 2;
    let b = sample2 - sample1 - a;
    let c = sample1;
    (a * n.pow(2)) + (b * n) + c
}

const GOAL: usize = 26_501_365;

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    if let Ok(grid) = Grid::from_str(input) {
        let Ok(size) = usize::try_from(grid.size) else {
            return None;
        };
        let edge = size / 2;

        let sample1 = grid.reachable_in_steps(edge, true);
        let sample2 = grid.reachable_in_steps(edge + size, true);
        let sample3 = grid.reachable_in_steps(edge + (2 * size), true);

        Some(quadratic_formula(
            sample1,
            sample2,
            sample3,
            (GOAL - edge) / size,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_grid() -> Grid {
        let mut rocks = HashSet::new();
        rocks.insert(Point(1, 5));
        rocks.insert(Point(1, 6));
        rocks.insert(Point(1, 7));
        rocks.insert(Point(1, 9));
        rocks.insert(Point(2, 1));
        rocks.insert(Point(2, 2));
        rocks.insert(Point(2, 3));
        rocks.insert(Point(2, 5));
        rocks.insert(Point(2, 6));
        rocks.insert(Point(2, 9));
        rocks.insert(Point(3, 2));
        rocks.insert(Point(3, 4));
        rocks.insert(Point(3, 8));
        rocks.insert(Point(4, 4));
        rocks.insert(Point(4, 6));
        rocks.insert(Point(5, 1));
        rocks.insert(Point(5, 2));
        rocks.insert(Point(5, 6));
        rocks.insert(Point(5, 7));
        rocks.insert(Point(5, 8));
        rocks.insert(Point(5, 9));
        rocks.insert(Point(6, 1));
        rocks.insert(Point(6, 2));
        rocks.insert(Point(6, 5));
        rocks.insert(Point(6, 9));
        rocks.insert(Point(7, 7));
        rocks.insert(Point(7, 8));
        rocks.insert(Point(8, 1));
        rocks.insert(Point(8, 2));
        rocks.insert(Point(8, 4));
        rocks.insert(Point(8, 6));
        rocks.insert(Point(8, 7));
        rocks.insert(Point(8, 8));
        rocks.insert(Point(8, 9));
        rocks.insert(Point(9, 1));
        rocks.insert(Point(9, 2));
        rocks.insert(Point(9, 5));
        rocks.insert(Point(9, 6));
        rocks.insert(Point(9, 8));
        rocks.insert(Point(9, 9));
        Grid {
            start: Point(5, 5),
            size: 11,
            rocks,
        }
    }

    fn example_state_initial() -> HashSet<Point> {
        let mut state = HashSet::new();
        state.insert(Point(5, 5));
        state
    }

    fn example_state_after_one_step() -> HashSet<Point> {
        let mut state = HashSet::new();
        state.insert(Point(4, 5));
        state.insert(Point(5, 4));
        state
    }

    fn example_state_after_two_steps() -> HashSet<Point> {
        let mut state = HashSet::new();
        state.insert(Point(3, 5));
        state.insert(Point(5, 5));
        state.insert(Point(5, 3));
        state.insert(Point(6, 4));
        state
    }

    #[test]
    fn test_grid_from_str() {
        assert_eq!(
            Grid::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_grid()),
        );
    }

    #[test]
    fn test_grid_initial_state() {
        let grid = example_grid();
        assert_eq!(grid.initial_state(), example_state_initial());
    }

    #[test]
    fn test_grid_step() {
        let grid = example_grid();

        let initial = example_state_initial();
        let one = example_state_after_one_step();
        assert_eq!(grid.step(&initial, false), one);

        let two = example_state_after_two_steps();
        assert_eq!(grid.step(&one, false), two);
    }

    #[test]
    fn test_reachable_in_steps() {
        let grid = example_grid();
        assert_eq!(grid.reachable_in_steps(6, false), 16);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_reachable_in_steps_on_repeated_grid() {
        let grid = example_grid();
        assert_eq!(grid.reachable_in_steps(6, true), 16);
        assert_eq!(grid.reachable_in_steps(10, true), 50);
        assert_eq!(grid.reachable_in_steps(50, true), 1594);
        assert_eq!(grid.reachable_in_steps(100, true), 6536);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(528192461129799));
    }
}
