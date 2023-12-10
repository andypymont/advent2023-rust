use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(10);

const GRID_SIZE: usize = 140;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Edge {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
struct ParseMazeError;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Pipe(Edge, Edge);

impl Pipe {
    fn traverse(self, edge: Edge) -> Option<Edge> {
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
            '|' => Some(Self(Edge::North, Edge::South)),
            '-' => Some(Self(Edge::East, Edge::West)),
            'L' => Some(Self(Edge::North, Edge::East)),
            'J' => Some(Self(Edge::North, Edge::West)),
            '7' => Some(Self(Edge::South, Edge::West)),
            'F' => Some(Self(Edge::East, Edge::South)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct EdgePosition {
    pos: usize,
    edge: Edge,
    steps: u32,
}

impl EdgePosition {
    fn cross_edge(&self) -> Self {
        match self.edge {
            Edge::North => Self {
                pos: self.pos - GRID_SIZE,
                edge: Edge::South,
                steps: self.steps,
            },
            Edge::East => Self {
                pos: self.pos + 1,
                edge: Edge::West,
                steps: self.steps,
            },
            Edge::South => Self {
                pos: self.pos + GRID_SIZE,
                edge: Edge::North,
                steps: self.steps,
            },
            Edge::West => Self {
                pos: self.pos - 1,
                edge: Edge::East,
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

struct Maze {
    start: usize,
    grid: [Option<Pipe>; GRID_SIZE * GRID_SIZE],
}

impl Maze {
    fn furthest_point_in_loop(&self) -> Option<u32> {
        let mut visited = [false; GRID_SIZE * GRID_SIZE];
        let mut queue = VecDeque::new();

        for edge in [Edge::North, Edge::East, Edge::South, Edge::West] {
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
}

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
pub fn part_two(_input: &str) -> Option<u32> {
    None
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
            Some(Pipe(Edge::East, Edge::West))
        );
        assert_eq!(
            Pipe::read_from_char('|'),
            Some(Pipe(Edge::North, Edge::South))
        );
        assert_eq!(
            Pipe::read_from_char('F'),
            Some(Pipe(Edge::East, Edge::South))
        );
    }

    #[test]
    fn test_traverse_pipe() {
        let pipe = Pipe(Edge::East, Edge::West);
        assert_eq!(pipe.traverse(Edge::North), None);
        assert_eq!(pipe.traverse(Edge::East), Some(Edge::West));
        assert_eq!(pipe.traverse(Edge::South), None);
        assert_eq!(pipe.traverse(Edge::West), Some(Edge::East));

        let pipe = Pipe(Edge::East, Edge::South);
        assert_eq!(pipe.traverse(Edge::North), None);
        assert_eq!(pipe.traverse(Edge::East), Some(Edge::South));
        assert_eq!(pipe.traverse(Edge::South), Some(Edge::East));
        assert_eq!(pipe.traverse(Edge::West), None);
    }

    #[test]
    fn test_parse_maze() {
        let maze: Maze = advent_of_code::template::read_file("examples", DAY)
            .parse()
            .expect("No error during Grid parsing");
        assert_eq!(maze.start, position(2, 0));

        let grid = maze.grid;
        assert_eq!(grid[position(0, 0)], Some(Pipe(Edge::South, Edge::West)));
        assert_eq!(grid[position(0, 1)], Some(Pipe(Edge::East, Edge::West)));
        assert_eq!(grid[position(0, 2)], Some(Pipe(Edge::East, Edge::South)));
        assert_eq!(grid[position(0, 3)], Some(Pipe(Edge::South, Edge::West)));
        assert_eq!(grid[position(0, 4)], Some(Pipe(Edge::East, Edge::West)));
        assert_eq!(grid[position(1, 0)], None);
        assert_eq!(grid[position(1, 1)], Some(Pipe(Edge::East, Edge::South)));
        assert_eq!(grid[position(1, 2)], Some(Pipe(Edge::North, Edge::West)));
        assert_eq!(grid[position(1, 3)], Some(Pipe(Edge::North, Edge::South)));
        assert_eq!(grid[position(1, 4)], Some(Pipe(Edge::South, Edge::West)));
        assert_eq!(grid[position(2, 0)], None);
        assert_eq!(grid[position(2, 1)], Some(Pipe(Edge::North, Edge::West)));
        assert_eq!(grid[position(2, 2)], Some(Pipe(Edge::North, Edge::East)));
        assert_eq!(grid[position(2, 3)], Some(Pipe(Edge::North, Edge::East)));
        assert_eq!(grid[position(2, 4)], Some(Pipe(Edge::South, Edge::West)));
        assert_eq!(grid[position(3, 0)], Some(Pipe(Edge::North, Edge::South)));
        assert_eq!(grid[position(3, 1)], Some(Pipe(Edge::East, Edge::South)));
        assert_eq!(grid[position(3, 2)], Some(Pipe(Edge::East, Edge::West)));
        assert_eq!(grid[position(3, 3)], Some(Pipe(Edge::East, Edge::West)));
        assert_eq!(grid[position(3, 4)], Some(Pipe(Edge::North, Edge::West)));
        assert_eq!(grid[position(4, 0)], Some(Pipe(Edge::North, Edge::East)));
        assert_eq!(grid[position(4, 1)], Some(Pipe(Edge::North, Edge::West)));
        assert_eq!(grid[position(4, 2)], None);
        assert_eq!(grid[position(4, 3)], Some(Pipe(Edge::North, Edge::East)));
        assert_eq!(grid[position(4, 4)], Some(Pipe(Edge::North, Edge::West)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
