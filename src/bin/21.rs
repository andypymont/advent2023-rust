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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Space {
    Empty,
    Blocked,
    Occupied,
}

#[derive(Debug, PartialEq)]
struct Grid {
    size: usize,
    spaces: Vec<Space>,
}

impl Grid {
    fn expanded(&self) -> Self {
        let mut spaces = Vec::new();
        let size = self.size * 5;

        for r in 0..size {
            let copy_r = r % self.size;
            for c in 0..size {
                let copy_c = c % self.size;
                let is_centre = (r / self.size == 2) && (c / self.size == 2);

                let space = self.spaces[(copy_r * self.size) + copy_c];
                if !is_centre && space == Space::Occupied {
                    spaces.push(Space::Empty);
                } else {
                    spaces.push(space);
                }
            }
        }

        Self { size, spaces }
    }

    fn occupied_spaces(&self) -> usize {
        self.spaces
            .iter()
            .filter(|space| space == &&Space::Occupied)
            .count()
    }

    fn reachable_neighbours(&self, pos: usize) -> impl Iterator<Item = usize> + '_ {
        let row = pos / self.size;
        let col = pos % self.size;

        COMPASS.iter().filter_map(move |direction| {
            match direction {
                Direction::North => pos.checked_sub(self.size),
                Direction::East => {
                    if (col + 1) < self.size {
                        Some(pos + 1)
                    } else {
                        None
                    }
                }
                Direction::South => {
                    if (row + 1) < self.size {
                        Some(pos + self.size)
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
            }
            .filter(|&pos| self.spaces[pos] != Space::Blocked)
        })
    }

    fn reachable_in_steps(self, steps: usize) -> usize {
        let mut reached = self;
        for _ in 0..steps {
            reached = reached.step();
        }
        reached.occupied_spaces()
    }

    fn reachable_in_key_step_counts(self, small_size: usize) -> Option<(usize, usize, usize)> {
        let offset = small_size / 2;
        let size = self.size / 2;
        let mut recorded = Vec::new();

        let mut reached = self;
        for steps in 1..=size {
            reached = reached.step();
            if let Some(steps) = steps.checked_sub(offset) {
                if steps % small_size == 0 {
                    recorded.push(reached.occupied_spaces());
                }
            }
        }

        let mut values = recorded.into_iter();
        let one = values.next()?;
        let two = values.next()?;
        let three = values.next()?;
        Some((one, two, three))
    }

    fn step(&self) -> Self {
        let mut stepped = vec![Space::Empty; self.spaces.len()];

        for (pos, space) in self.spaces.iter().enumerate() {
            match space {
                Space::Empty => (),
                Space::Blocked => stepped[pos] = Space::Blocked,
                Space::Occupied => {
                    for neighbour in self.reachable_neighbours(pos) {
                        stepped[neighbour] = Space::Occupied;
                    }
                }
            }
        }

        Self {
            size: self.size,
            spaces: stepped,
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseGridError;

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().peekable();
        let Some(size) = lines.peek().map(|line| line.len()) else {
            return Err(ParseGridError);
        };

        let mut spaces = Vec::new();

        for line in lines {
            if line.len() != size {
                return Err(ParseGridError);
            }

            for ch in line.chars() {
                match ch {
                    '.' => spaces.push(Space::Empty),
                    '#' => spaces.push(Space::Blocked),
                    'S' => spaces.push(Space::Occupied),
                    _ => return Err(ParseGridError),
                }
            }
        }

        Ok(Self { size, spaces })
    }
}

fn solve_quadratic(sample1: usize, sample2: usize, sample3: usize, n: usize) -> usize {
    let a = (sample3 - (2 * sample2) + sample1) / 2;
    let b = sample2 - sample1 - a;
    let c = sample1;
    (a * n.pow(2)) + (b * n) + c
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(grid) = Grid::from_str(input) {
        Some(grid.reachable_in_steps(64))
    } else {
        None
    }
}

const GOAL: usize = 26_501_365;

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    if let Ok(grid) = Grid::from_str(input) {
        let size = grid.size;
        grid.expanded()
            .reachable_in_key_step_counts(size)
            .map(|(a, b, c)| solve_quadratic(a, b, c, (GOAL - (size / 2)) / size))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_grid() -> Grid {
        let mut spaces = vec![Space::Empty; 11 * 11];
        spaces[16] = Space::Blocked;
        spaces[17] = Space::Blocked;
        spaces[18] = Space::Blocked;
        spaces[20] = Space::Blocked;
        spaces[23] = Space::Blocked;
        spaces[24] = Space::Blocked;
        spaces[25] = Space::Blocked;
        spaces[27] = Space::Blocked;
        spaces[28] = Space::Blocked;
        spaces[31] = Space::Blocked;
        spaces[35] = Space::Blocked;
        spaces[37] = Space::Blocked;
        spaces[41] = Space::Blocked;
        spaces[48] = Space::Blocked;
        spaces[50] = Space::Blocked;
        spaces[56] = Space::Blocked;
        spaces[57] = Space::Blocked;
        spaces[60] = Space::Occupied;
        spaces[61] = Space::Blocked;
        spaces[62] = Space::Blocked;
        spaces[63] = Space::Blocked;
        spaces[64] = Space::Blocked;
        spaces[67] = Space::Blocked;
        spaces[68] = Space::Blocked;
        spaces[71] = Space::Blocked;
        spaces[75] = Space::Blocked;
        spaces[84] = Space::Blocked;
        spaces[85] = Space::Blocked;
        spaces[89] = Space::Blocked;
        spaces[90] = Space::Blocked;
        spaces[92] = Space::Blocked;
        spaces[94] = Space::Blocked;
        spaces[95] = Space::Blocked;
        spaces[96] = Space::Blocked;
        spaces[97] = Space::Blocked;
        spaces[100] = Space::Blocked;
        spaces[101] = Space::Blocked;
        spaces[104] = Space::Blocked;
        spaces[105] = Space::Blocked;
        spaces[107] = Space::Blocked;
        spaces[108] = Space::Blocked;
        Grid { size: 11, spaces }
    }

    #[test]
    fn test_grid_from_str() {
        assert_eq!(
            Grid::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_grid()),
        );
    }

    fn example_grid_after_one_step() -> Grid {
        let mut grid = example_grid();
        grid.spaces[60] = Space::Empty;
        grid.spaces[49] = Space::Occupied;
        grid.spaces[59] = Space::Occupied;
        grid
    }

    fn example_grid_after_two_steps() -> Grid {
        let mut grid = example_grid();
        grid.spaces[38] = Space::Occupied;
        grid.spaces[58] = Space::Occupied;
        grid.spaces[70] = Space::Occupied;
        grid
    }

    #[test]
    fn test_grid_step() {
        let initial = example_grid();

        let one = example_grid_after_one_step();
        assert_eq!(initial.step(), one);

        let two = example_grid_after_two_steps();
        assert_eq!(one.step(), two);
    }

    #[test]
    fn test_grid_reachable_in_steps() {
        let grid = example_grid();
        assert_eq!(grid.reachable_in_steps(6), 16);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    fn example_expanded_grid() -> Grid {
        let mut spaces = vec![Space::Empty; 55 * 55];

        for offset in [
            0, 11, 22, 33, 44, 605, 616, 627, 638, 649, 1210, 1221, 1232, 1243, 1254, 1815, 1826,
            1837, 1848, 1859, 2420, 2431, 2442, 2453, 2464,
        ] {
            spaces[offset + 60] = Space::Blocked;
            spaces[offset + 61] = Space::Blocked;
            spaces[offset + 62] = Space::Blocked;
            spaces[offset + 64] = Space::Blocked;
            spaces[offset + 111] = Space::Blocked;
            spaces[offset + 112] = Space::Blocked;
            spaces[offset + 113] = Space::Blocked;
            spaces[offset + 115] = Space::Blocked;
            spaces[offset + 116] = Space::Blocked;
            spaces[offset + 119] = Space::Blocked;
            spaces[offset + 167] = Space::Blocked;
            spaces[offset + 169] = Space::Blocked;
            spaces[offset + 173] = Space::Blocked;
            spaces[offset + 224] = Space::Blocked;
            spaces[offset + 226] = Space::Blocked;
            spaces[offset + 276] = Space::Blocked;
            spaces[offset + 277] = Space::Blocked;
            spaces[offset + 281] = Space::Blocked;
            spaces[offset + 282] = Space::Blocked;
            spaces[offset + 283] = Space::Blocked;
            spaces[offset + 284] = Space::Blocked;
            spaces[offset + 331] = Space::Blocked;
            spaces[offset + 332] = Space::Blocked;
            spaces[offset + 335] = Space::Blocked;
            spaces[offset + 339] = Space::Blocked;
            spaces[offset + 392] = Space::Blocked;
            spaces[offset + 393] = Space::Blocked;
            spaces[offset + 441] = Space::Blocked;
            spaces[offset + 442] = Space::Blocked;
            spaces[offset + 444] = Space::Blocked;
            spaces[offset + 446] = Space::Blocked;
            spaces[offset + 447] = Space::Blocked;
            spaces[offset + 448] = Space::Blocked;
            spaces[offset + 449] = Space::Blocked;
            spaces[offset + 496] = Space::Blocked;
            spaces[offset + 497] = Space::Blocked;
            spaces[offset + 500] = Space::Blocked;
            spaces[offset + 501] = Space::Blocked;
            spaces[offset + 503] = Space::Blocked;
            spaces[offset + 504] = Space::Blocked;
        }

        spaces[1512] = Space::Occupied;

        Grid { size: 55, spaces }
    }

    #[test]
    fn test_grid_expanded() {
        let grid = example_grid();
        assert_eq!(grid.expanded(), example_expanded_grid());
    }

    #[test]
    fn test_grid_reachable_in_key_step_counts() {
        let grid = example_expanded_grid();
        assert_eq!(grid.reachable_in_key_step_counts(11), Some((13, 129, 427)),);
    }

    #[test]
    fn test_solve_quadratic() {
        assert_eq!(
            solve_quadratic(13, 129, 427, 2_409_214),
            528_192_461_129_799,
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(528_192_461_129_799));
    }
}
