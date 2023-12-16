use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(16);

const GRID_SIZE: usize = 110;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn is_vertical(self) -> bool {
        match self {
            Self::North | Self::South => true,
            Self::East | Self::West => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MirrorType {
    Forwardslash,
    Backslash,
}

impl MirrorType {
    fn next_directions(self, current: Direction) -> (Option<Direction>, Option<Direction>) {
        let nd = match (self, current) {
            (Self::Forwardslash, Direction::North) | (Self::Backslash, Direction::South) => {
                Direction::East
            }
            (Self::Forwardslash, Direction::East) | (Self::Backslash, Direction::West) => {
                Direction::North
            }
            (Self::Forwardslash, Direction::South) | (Self::Backslash, Direction::North) => {
                Direction::West
            }
            (Self::Forwardslash, Direction::West) | (Self::Backslash, Direction::East) => {
                Direction::South
            }
        };
        (Some(nd), None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SplitterType {
    Horizontal,
    Vertical,
}

impl SplitterType {
    fn next_directions(self, current: Direction) -> (Option<Direction>, Option<Direction>) {
        match (self, current.is_vertical()) {
            (Self::Vertical, false) => (Some(Direction::North), Some(Direction::South)),
            (Self::Horizontal, true) => (Some(Direction::West), Some(Direction::East)),
            _ => (Some(current), None),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Space {
    Empty,
    Mirror(MirrorType),
    Splitter(SplitterType),
}

impl Space {
    fn next_directions(self, current: Direction) -> (Option<Direction>, Option<Direction>) {
        match self {
            Self::Mirror(mirror_type) => mirror_type.next_directions(current),
            Self::Splitter(splitter_type) => splitter_type.next_directions(current),
            Self::Empty => (Some(current), None),
        }
    }
}

#[derive(Debug, PartialEq)]
struct EnergisationTracker {
    grid: [u8; GRID_SIZE * GRID_SIZE],
}

impl EnergisationTracker {
    fn new() -> Self {
        Self {
            grid: [0; GRID_SIZE * GRID_SIZE],
        }
    }

    fn energised_cells(&self) -> usize {
        self.grid.iter().filter(|v| v != &&0).count()
    }

    fn visit(&mut self, pos: usize, direction: Direction) -> bool {
        let value = match direction {
            Direction::North => 1,
            Direction::East => 2,
            Direction::South => 4,
            Direction::West => 8,
        };
        let visited = self.grid[pos] & value == value;
        self.grid[pos] |= value;
        visited
    }
}

#[derive(Debug, PartialEq)]
struct Contraption {
    grid: [Space; GRID_SIZE * GRID_SIZE],
}

impl Contraption {
    fn step(pos: usize, dir: Direction) -> Option<usize> {
        let row = pos / GRID_SIZE;
        let col = pos % GRID_SIZE;

        match dir {
            Direction::North => {
                if row > 0 {
                    Some(pos - GRID_SIZE)
                } else {
                    None
                }
            }
            Direction::East => {
                if (col + 1) < GRID_SIZE {
                    Some(pos + 1)
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
            Direction::West => {
                if col > 0 {
                    Some(pos - 1)
                } else {
                    None
                }
            }
        }
    }

    fn energised_cells(&self, start_pos: usize, start_facing: Direction) -> usize {
        let mut visited = EnergisationTracker::new();
        let mut queue = VecDeque::new();

        queue.push_back((start_pos, start_facing));

        while let Some((pos, direction)) = queue.pop_front() {
            if !visited.visit(pos, direction) {
                let (first, second) = self.grid[pos].next_directions(direction);

                if let Some(direction) = first {
                    if let Some(next_pos) = Self::step(pos, direction) {
                        queue.push_back((next_pos, direction));
                    }
                }

                if let Some(direction) = second {
                    if let Some(next_pos) = Self::step(pos, direction) {
                        queue.push_back((next_pos, direction));
                    }
                }
            }
        }

        visited.energised_cells()
    }

    fn most_energised_cells(&self) -> usize {
        let mut most = 0;

        for row_or_col in 0..GRID_SIZE {
            most = most.max(self.energised_cells(row_or_col, Direction::South));

            let south = (GRID_SIZE * GRID_SIZE) - 1 - row_or_col;
            most = most.max(self.energised_cells(south, Direction::North));

            let west = GRID_SIZE * row_or_col;
            most = most.max(self.energised_cells(west, Direction::East));

            let east = west + GRID_SIZE - 1;
            most = most.max(self.energised_cells(east, Direction::West));
        }

        most
    }
}

#[derive(Debug, PartialEq)]
struct ParseContraptionError;

impl TryFrom<char> for Space {
    type Error = ParseContraptionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Space::Empty),
            '\\' => Ok(Space::Mirror(MirrorType::Backslash)),
            '/' => Ok(Space::Mirror(MirrorType::Forwardslash)),
            '|' => Ok(Space::Splitter(SplitterType::Vertical)),
            '-' => Ok(Space::Splitter(SplitterType::Horizontal)),
            _ => Err(ParseContraptionError),
        }
    }
}

impl FromStr for Contraption {
    type Err = ParseContraptionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = [Space::Empty; GRID_SIZE * GRID_SIZE];

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let space = ch.try_into()?;
                grid[(row * GRID_SIZE) + col] = space;
            }
        }

        Ok(Self { grid })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(contraption) = Contraption::from_str(input) {
        Some(contraption.energised_cells(0, Direction::East))
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    if let Ok(contraption) = Contraption::from_str(input) {
        Some(contraption.most_energised_cells())
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

    fn example_contraption() -> Contraption {
        let mut grid = [Space::Empty; GRID_SIZE * GRID_SIZE];

        grid[position(0, 1)] = Space::Splitter(SplitterType::Vertical);
        grid[position(0, 5)] = Space::Mirror(MirrorType::Backslash);
        grid[position(1, 0)] = Space::Splitter(SplitterType::Vertical);
        grid[position(1, 2)] = Space::Splitter(SplitterType::Horizontal);
        grid[position(1, 4)] = Space::Mirror(MirrorType::Backslash);
        grid[position(2, 5)] = Space::Splitter(SplitterType::Vertical);
        grid[position(2, 6)] = Space::Splitter(SplitterType::Horizontal);
        grid[position(3, 8)] = Space::Splitter(SplitterType::Vertical);
        grid[position(5, 9)] = Space::Mirror(MirrorType::Backslash);
        grid[position(6, 4)] = Space::Mirror(MirrorType::Forwardslash);
        grid[position(6, 6)] = Space::Mirror(MirrorType::Backslash);
        grid[position(6, 7)] = Space::Mirror(MirrorType::Backslash);
        grid[position(7, 1)] = Space::Splitter(SplitterType::Horizontal);
        grid[position(7, 3)] = Space::Splitter(SplitterType::Horizontal);
        grid[position(7, 4)] = Space::Mirror(MirrorType::Forwardslash);
        grid[position(7, 7)] = Space::Splitter(SplitterType::Vertical);
        grid[position(8, 1)] = Space::Splitter(SplitterType::Vertical);
        grid[position(8, 6)] = Space::Splitter(SplitterType::Horizontal);
        grid[position(8, 7)] = Space::Splitter(SplitterType::Vertical);
        grid[position(8, 9)] = Space::Mirror(MirrorType::Backslash);
        grid[position(9, 2)] = Space::Mirror(MirrorType::Forwardslash);
        grid[position(9, 3)] = Space::Mirror(MirrorType::Forwardslash);
        grid[position(9, 5)] = Space::Splitter(SplitterType::Vertical);

        Contraption { grid }
    }

    #[test]
    fn test_parse_contraption() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(input.parse(), Ok(example_contraption()));
    }

    #[test]
    fn test_next_directions() {
        let space = Space::Empty;
        assert_eq!(
            space.next_directions(Direction::East),
            (Some(Direction::East), None),
        );

        let space = Space::Mirror(MirrorType::Forwardslash);
        assert_eq!(
            space.next_directions(Direction::East),
            (Some(Direction::North), None),
        );

        let space = Space::Mirror(MirrorType::Backslash);
        assert_eq!(
            space.next_directions(Direction::East),
            (Some(Direction::South), None),
        );

        let space = Space::Splitter(SplitterType::Horizontal);
        assert_eq!(
            space.next_directions(Direction::East),
            (Some(Direction::East), None),
        );

        let space = Space::Splitter(SplitterType::Vertical);
        assert_eq!(
            space.next_directions(Direction::East),
            (Some(Direction::North), Some(Direction::South)),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(446));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(548));
    }
}
