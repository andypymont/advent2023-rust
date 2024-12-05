use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(10);

const GRID_SIZE: usize = 140;

#[derive(Clone, Copy, Debug, PartialEq)]
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
struct Pipe(Direction, Direction);

impl Pipe {
    fn traverse(self, edge: Direction) -> Option<Direction> {
        if edge == self.0 {
            Some(self.1)
        } else if edge == self.1 {
            Some(self.0)
        } else {
            None
        }
    }

    fn read_from_char(ch: char) -> Option<Self> {
        match ch {
            '|' => Some(Self(Direction::North, Direction::South)),
            '-' => Some(Self(Direction::East, Direction::West)),
            'L' => Some(Self(Direction::North, Direction::East)),
            'J' => Some(Self(Direction::North, Direction::West)),
            '7' => Some(Self(Direction::South, Direction::West)),
            'F' => Some(Self(Direction::East, Direction::South)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct EdgePosition {
    pos: usize,
    edge: Direction,
    steps: u32,
}

impl EdgePosition {
    fn cross_edge(&self) -> Self {
        match self.edge {
            Direction::North => Self {
                pos: self.pos - GRID_SIZE,
                edge: Direction::South,
                steps: self.steps,
            },
            Direction::East => Self {
                pos: self.pos + 1,
                edge: Direction::West,
                steps: self.steps,
            },
            Direction::South => Self {
                pos: self.pos + GRID_SIZE,
                edge: Direction::North,
                steps: self.steps,
            },
            Direction::West => Self {
                pos: self.pos - 1,
                edge: Direction::East,
                steps: self.steps,
            },
        }
    }

    fn traverse_pipe(&self, pipe: Pipe) -> Option<Self> {
        pipe.traverse(self.edge).map(|edge| Self {
            pos: self.pos,
            edge,
            steps: self.steps + 1,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Corner {
    fn move_crosses(self, direction: Direction) -> Option<Direction> {
        match (self, direction) {
            (Corner::TopLeft, Direction::East) | (Corner::TopRight, Direction::West) => {
                Some(Direction::North)
            }
            (Corner::TopLeft, Direction::South) | (Corner::BottomLeft, Direction::North) => {
                Some(Direction::West)
            }
            (Corner::TopRight, Direction::South) | (Corner::BottomRight, Direction::North) => {
                Some(Direction::East)
            }
            (Corner::BottomLeft, Direction::East) | (Corner::BottomRight, Direction::West) => {
                Some(Direction::South)
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct CornerPosition {
    pos: usize,
    corner: Corner,
}

impl CornerPosition {
    fn adjacent_positions<'a>(&'a self, pipe: &'a Option<Pipe>) -> impl Iterator<Item = Self> + 'a {
        // calculate adjacent locations, accounting for OOB
        let north = self.pos.checked_sub(GRID_SIZE);
        let east = if (self.pos % GRID_SIZE) == (GRID_SIZE - 1) {
            None
        } else {
            Some(self.pos + 1)
        };
        let west = self.pos.checked_sub(1);
        let south = if (self.pos / GRID_SIZE) == (GRID_SIZE - 1) {
            None
        } else {
            Some(self.pos + GRID_SIZE)
        };

        COMPASS.iter().filter_map(move |direction| {
            // check for move blocked by part of pipe
            if let Some(cross) = self.corner.move_crosses(*direction) {
                if let Some(pipe) = pipe {
                    if pipe.0 == cross || pipe.1 == cross {
                        return None;
                    }
                }
            }

            // calculate destination
            match (direction, self.corner) {
                (Direction::North, Corner::TopLeft) => north.map(|pos| Self {
                    pos,
                    corner: Corner::BottomLeft,
                }),
                (Direction::North, Corner::TopRight) => north.map(|pos| Self {
                    pos,
                    corner: Corner::BottomRight,
                }),
                (Direction::North, Corner::BottomLeft) | (Direction::West, Corner::TopRight) => {
                    Some(Self {
                        pos: self.pos,
                        corner: Corner::TopLeft,
                    })
                }
                (Direction::North, Corner::BottomRight) | (Direction::East, Corner::TopLeft) => {
                    Some(Self {
                        pos: self.pos,
                        corner: Corner::TopRight,
                    })
                }
                (Direction::East, Corner::TopRight) => east.map(|pos| Self {
                    pos,
                    corner: Corner::TopLeft,
                }),
                (Direction::East, Corner::BottomLeft) | (Direction::South, Corner::TopRight) => {
                    Some(Self {
                        pos: self.pos,
                        corner: Corner::BottomRight,
                    })
                }
                (Direction::East, Corner::BottomRight) => east.map(|pos| Self {
                    pos,
                    corner: Corner::BottomLeft,
                }),
                (Direction::South, Corner::TopLeft) | (Direction::West, Corner::BottomRight) => {
                    Some(Self {
                        pos: self.pos,
                        corner: Corner::BottomLeft,
                    })
                }
                (Direction::South, Corner::BottomLeft) => south.map(|pos| Self {
                    pos,
                    corner: Corner::TopLeft,
                }),
                (Direction::South, Corner::BottomRight) => south.map(|pos| Self {
                    pos,
                    corner: Corner::TopRight,
                }),
                (Direction::West, Corner::TopLeft) => west.map(|pos| Self {
                    pos,
                    corner: Corner::TopRight,
                }),
                (Direction::West, Corner::BottomLeft) => west.map(|pos| Self {
                    pos,
                    corner: Corner::BottomRight,
                }),
            }
        })
    }
}

#[derive(Debug, PartialEq)]
struct CornerVisitTracker([u8; GRID_SIZE * GRID_SIZE]);

impl CornerVisitTracker {
    fn new() -> Self {
        Self([0; GRID_SIZE * GRID_SIZE])
    }

    fn count_unvisited(&self) -> u32 {
        self.0
            .iter()
            .fold(0, |empty, val| empty + u32::from(val == &0))
    }

    fn visit(&mut self, pos: &CornerPosition) -> bool {
        let value = match pos.corner {
            Corner::TopLeft => 1,
            Corner::TopRight => 2,
            Corner::BottomLeft => 4,
            Corner::BottomRight => 8,
        };
        let visited = self.0[pos.pos] & value == value;
        self.0[pos.pos] |= value;
        visited
    }
}

struct Maze {
    start: usize,
    grid: [Option<Pipe>; GRID_SIZE * GRID_SIZE],
}

impl Maze {
    fn furthest_point_in_loop(&self) -> Option<u32> {
        let mut visited = [false; GRID_SIZE * GRID_SIZE];
        let mut queue = VecDeque::new();

        for edge in COMPASS {
            queue.push_back(
                EdgePosition {
                    pos: self.start,
                    edge,
                    steps: 0,
                }
                .cross_edge(),
            );
        }

        while let Some(pos) = queue.pop_front() {
            if let Some(pipe) = self.grid[pos.pos] {
                if let Some(next) = pos.traverse_pipe(pipe).map(|p| p.cross_edge()) {
                    if visited[next.pos] {
                        return Some(next.steps + 1);
                    }
                    visited[next.pos] = true;
                    queue.push_back(next);
                }
            }
        }

        None
    }

    fn replacement_start_pipe(&self) -> Option<Pipe> {
        let pos = self.start;
        let mut edges = COMPASS.iter().filter_map(|edge| {
            let cross = EdgePosition {
                pos,
                edge: *edge,
                steps: 0,
            }
            .cross_edge();
            let pipe = self.grid[cross.pos]?;
            cross.traverse_pipe(pipe).map(|_| edge)
        });

        let first = edges.next();
        let second = edges.next();
        match (first, second) {
            (Some(first), Some(second)) => Some(Pipe(*first, *second)),
            _ => None,
        }
    }

    fn spaces_enclosed_by_loop(&self) -> u32 {
        let mut visited = CornerVisitTracker::new();
        let mut queue = VecDeque::new();

        let start_pipe = self.replacement_start_pipe();

        queue.push_back(CornerPosition {
            pos: 0,
            corner: Corner::TopLeft,
        });

        while let Some(pos) = queue.pop_front() {
            if !visited.visit(&pos) {
                let pipe = if pos.pos == self.start {
                    start_pipe
                } else {
                    self.grid[pos.pos]
                };
                queue.extend(pos.adjacent_positions(&pipe));
            }
        }

        visited.count_unvisited()
    }
}

#[derive(Debug, PartialEq)]
struct ParseMazeError;

impl FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut start = 0;
        let mut grid = [None; GRID_SIZE * GRID_SIZE];

        for (row, line) in input.lines().enumerate() {
            let row_start = row * GRID_SIZE;
            for (col, ch) in line.chars().enumerate() {
                let pos = row_start + col;
                grid[pos] = Pipe::read_from_char(ch);
                if ch == 'S' {
                    start = pos;
                }
            }
        }

        Ok(Self { start, grid })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(maze) = Maze::from_str(input) {
        maze.furthest_point_in_loop()
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    if let Ok(maze) = Maze::from_str(input) {
        Some(maze.spaces_enclosed_by_loop())
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

    #[test]
    fn test_pipe_read_from_char() {
        assert_eq!(Pipe::read_from_char('!'), None);
        assert_eq!(Pipe::read_from_char('.'), None);
        assert_eq!(
            Pipe::read_from_char('-'),
            Some(Pipe(Direction::East, Direction::West))
        );
        assert_eq!(
            Pipe::read_from_char('|'),
            Some(Pipe(Direction::North, Direction::South))
        );
        assert_eq!(
            Pipe::read_from_char('F'),
            Some(Pipe(Direction::East, Direction::South))
        );
    }

    #[test]
    fn test_traverse_pipe() {
        let pipe = Pipe(Direction::East, Direction::West);
        assert_eq!(pipe.traverse(Direction::North), None);
        assert_eq!(pipe.traverse(Direction::East), Some(Direction::West));
        assert_eq!(pipe.traverse(Direction::South), None);
        assert_eq!(pipe.traverse(Direction::West), Some(Direction::East));

        let pipe = Pipe(Direction::East, Direction::South);
        assert_eq!(pipe.traverse(Direction::North), None);
        assert_eq!(pipe.traverse(Direction::East), Some(Direction::South));
        assert_eq!(pipe.traverse(Direction::South), Some(Direction::East));
        assert_eq!(pipe.traverse(Direction::West), None);
    }

    #[test]
    fn test_parse_maze() {
        let maze: Maze = advent_of_code::template::read_file("examples", DAY)
            .parse()
            .expect("No error during Maze parsing");
        assert_eq!(maze.start, position(1, 4));

        let grid = maze.grid;

        for (pipe, count) in [
            (Some(Pipe(Direction::North, Direction::South)), 40),
            (Some(Pipe(Direction::East, Direction::West)), 32),
            (Some(Pipe(Direction::North, Direction::East)), 32),
            (Some(Pipe(Direction::East, Direction::South)), 31),
            (Some(Pipe(Direction::South, Direction::West)), 31),
            (Some(Pipe(Direction::North, Direction::West)), 30),
        ] {
            assert_eq!(
                grid.iter().filter(|p| p == &&pipe).count(),
                count,
                "{count} pipes matching {pipe:?}",
            );
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_replacement_start_pipe() {
        let maze: Maze = advent_of_code::template::read_file("examples", DAY)
            .parse()
            .expect("No error during Maze parsing");
        assert_eq!(
            maze.replacement_start_pipe(),
            Some(Pipe(Direction::South, Direction::West)),
        )
    }

    #[test]
    fn test_corner_adjacent_positions() {
        let pos = CornerPosition {
            pos: position(0, 0),
            corner: Corner::TopLeft,
        };
        let pipe = Some(Pipe(Direction::East, Direction::South));
        assert_eq!(
            pos.adjacent_positions(&pipe)
                .collect::<Vec<CornerPosition>>(),
            vec![
                CornerPosition {
                    pos: position(0, 0),
                    corner: Corner::TopRight
                },
                CornerPosition {
                    pos: position(0, 0),
                    corner: Corner::BottomLeft
                },
            ],
        );

        let pos = CornerPosition {
            pos: position(2, 2),
            corner: Corner::TopRight,
        };
        let pipe = Some(Pipe(Direction::East, Direction::West));
        assert_eq!(
            pos.adjacent_positions(&pipe)
                .collect::<Vec<CornerPosition>>(),
            vec![
                CornerPosition {
                    pos: position(1, 2),
                    corner: Corner::BottomRight
                },
                CornerPosition {
                    pos: position(2, 3),
                    corner: Corner::TopLeft
                },
                CornerPosition {
                    pos: position(2, 2),
                    corner: Corner::TopLeft
                },
            ],
        );

        let pos = CornerPosition {
            pos: position(3, 3),
            corner: Corner::BottomRight,
        };
        let pipe = None;
        assert_eq!(
            pos.adjacent_positions(&pipe)
                .collect::<Vec<CornerPosition>>(),
            vec![
                CornerPosition {
                    pos: position(3, 3),
                    corner: Corner::TopRight
                },
                CornerPosition {
                    pos: position(3, 4),
                    corner: Corner::BottomLeft
                },
                CornerPosition {
                    pos: position(4, 3),
                    corner: Corner::TopRight
                },
                CornerPosition {
                    pos: position(3, 3),
                    corner: Corner::BottomLeft
                },
            ],
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
