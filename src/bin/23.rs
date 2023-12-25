use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

advent_of_code::solution!(23);

const GRID_SIZE: usize = 141;

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

#[derive(Debug, PartialEq)]
struct GraphMappingState {
    source: usize,
    position: usize,
    steps: u32,
    visited: [bool; GRID_SIZE * GRID_SIZE],
}

impl GraphMappingState {
    fn new(position: usize) -> Self {
        let mut visited = [false; GRID_SIZE * GRID_SIZE];
        visited[position] = true;
        Self {
            source: position,
            position,
            steps: 0,
            visited,
        }
    }

    fn visit(&self, position: usize) -> Self {
        let mut visited = self.visited;
        visited[position] = true;
        Self {
            source: self.source,
            position,
            steps: self.steps + 1,
            visited,
        }
    }
}

#[derive(Debug, PartialEq)]
struct HikeState {
    position: usize,
    steps: u32,
    visited: [bool; GRID_SIZE * GRID_SIZE],
}

impl HikeState {
    fn new(position: usize) -> Self {
        let mut visited = [false; GRID_SIZE * GRID_SIZE];
        visited[position] = true;
        Self {
            position,
            steps: 0,
            visited,
        }
    }

    fn visit(&self, position: usize, steps: u32) -> Self {
        let mut visited = self.visited;
        visited[position] = true;
        Self {
            position,
            steps: self.steps + steps,
            visited,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Trail {
    Empty,
    Start,
    Finish,
    Forest,
    SlopeWE,
    SlopeNS,
}

impl Trail {
    fn can_exit_in_direction(self, direction: &Direction) -> bool {
        match self {
            Self::Empty | Self::Start => true,
            Self::Forest | Self::Finish => false,
            Self::SlopeNS => direction == &Direction::South,
            Self::SlopeWE => direction == &Direction::East,
        }
    }
}

#[derive(Debug, PartialEq)]
struct TrailGraph {
    start: usize,
    finish: usize,
    nodes: HashMap<usize, HashMap<usize, u32>>,
}

impl TrailGraph {
    fn add_connection(&mut self, source: usize, destination: usize, distance: u32) {
        self.nodes
            .entry(source)
            .or_default()
            .insert(destination, distance);
    }

    fn longest_hike(&self) -> Option<u32> {
        let mut hikes = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(HikeState::new(self.start));

        while let Some(state) = queue.pop_front() {
            if state.position == self.finish {
                hikes.push(state.steps);
            } else if let Some(node) = self.nodes.get(&state.position) {
                for (position, steps) in node {
                    if !state.visited[*position] {
                        queue.push_back(state.visit(*position, *steps));
                    }
                }
            }
        }

        hikes.into_iter().max()
    }
}

#[derive(Debug, PartialEq)]
struct TrailMap {
    trails: [Trail; GRID_SIZE * GRID_SIZE],
}

impl TrailMap {
    fn graph(&self) -> TrailGraph {
        let (start, finish) = self.trails.iter().enumerate().fold(
            (0, 0),
            |(start, finish), (pos, trail)| match trail {
                Trail::Start => (pos, finish),
                Trail::Finish => (start, pos),
                _ => (start, finish),
            },
        );
        let mut graph = TrailGraph {
            start,
            finish,
            nodes: HashMap::new(),
        };

        let mut queued = [false; GRID_SIZE * GRID_SIZE];
        let mut queue = VecDeque::new();
        queue.push_back(GraphMappingState::new(start));
        queued[start] = true;

        while let Some(state) = queue.pop_front() {
            let trail = self.trails[state.position];
            let exits: Vec<usize> = COMPASS
                .iter()
                .filter_map(move |direction| {
                    if !trail.can_exit_in_direction(direction) {
                        return None;
                    }
                    Self::step_in_direction(state.position, direction)
                        .filter(|pos| self.trails[*pos] != Trail::Forest)
                })
                .collect();

            if state.position != state.source
                && (exits.len() > 2 || trail == Trail::Start || trail == Trail::Finish)
            {
                // new point of interest found; record route, then initiate a new
                // search from this POI if we haven't already
                graph.add_connection(state.source, state.position, state.steps);
                if !queued[state.position] {
                    queue.push_back(GraphMappingState::new(state.position));
                    queued[state.position] = true;
                }
            } else {
                // not at a new point of interest: keep exploring this route
                for new_pos in exits {
                    if !state.visited[new_pos] {
                        queue.push_back(state.visit(new_pos));
                    }
                }
            }
        }

        graph
    }

    fn step_in_direction(pos: usize, direction: &Direction) -> Option<usize> {
        let row = pos / GRID_SIZE;
        let col = pos % GRID_SIZE;

        match direction {
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
}

#[derive(Debug, PartialEq)]
struct ParseTrailError;

impl FromStr for TrailMap {
    type Err = ParseTrailError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut trails = [Trail::Empty; GRID_SIZE * GRID_SIZE];
        let mut finish = None;

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let pos = (row * GRID_SIZE) + col;
                match ch {
                    '.' => {
                        if row == 0 {
                            trails[pos] = Trail::Start;
                        } else {
                            trails[pos] = Trail::Empty;
                            finish = Some(pos);
                        }
                    }
                    '#' => trails[pos] = Trail::Forest,
                    '>' => trails[pos] = Trail::SlopeWE,
                    'v' => trails[pos] = Trail::SlopeNS,
                    _ => return Err(ParseTrailError),
                }
            }
        }

        match finish {
            None => return Err(ParseTrailError),
            Some(finish) => trails[finish] = Trail::Finish,
        }

        Ok(Self { trails })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(trail_map) = TrailMap::from_str(input) {
        let graph = trail_map.graph();
        graph.longest_hike()
    } else {
        None
    }
}

#[must_use]
pub fn part_two(__input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(row: usize, col: usize) -> usize {
        (row * GRID_SIZE) + col
    }

    fn example_trail_map() -> TrailMap {
        let mut trails = [Trail::Empty; GRID_SIZE * GRID_SIZE];

        for (r, first_c, last_c) in [
            (0, 0, 22),
            (1, 0, 0),
            (1, 8, 16),
            (1, 20, 22),
            (2, 0, 6),
            (2, 8, 16),
            (2, 18, 18),
            (2, 20, 22),
            (3, 0, 2),
            (3, 8, 8),
            (3, 14, 16),
            (3, 18, 18),
            (3, 20, 22),
            (4, 0, 2),
            (4, 4, 8),
            (4, 10, 10),
            (4, 12, 12),
            (4, 14, 16),
            (4, 18, 18),
            (4, 20, 22),
            (5, 0, 2),
            (5, 8, 8),
            (5, 10, 10),
            (5, 12, 12),
            (5, 18, 18),
            (5, 22, 22),
            (6, 0, 2),
            (6, 4, 6),
            (6, 8, 8),
            (6, 10, 10),
            (6, 12, 20),
            (6, 22, 22),
            (7, 0, 2),
            (7, 6, 6),
            (7, 8, 8),
            (7, 10, 10),
            (7, 18, 18),
            (7, 22, 22),
            (8, 0, 4),
            (8, 6, 6),
            (8, 8, 8),
            (8, 10, 16),
            (8, 18, 18),
            (8, 20, 22),
            (9, 0, 0),
            (9, 6, 6),
            (9, 8, 8),
            (9, 10, 10),
            (9, 18, 18),
            (9, 22, 22),
            (10, 0, 0),
            (10, 2, 6),
            (10, 8, 8),
            (10, 10, 10),
            (10, 12, 20),
            (10, 22, 22),
            (11, 0, 0),
            (11, 2, 2),
            (11, 6, 6),
            (11, 10, 10),
            (11, 14, 16),
            (11, 22, 22),
            (12, 0, 0),
            (12, 2, 2),
            (12, 4, 4),
            (12, 6, 12),
            (12, 14, 16),
            (12, 18, 20),
            (12, 22, 22),
            (13, 0, 0),
            (13, 4, 4),
            (13, 8, 8),
            (13, 16, 16),
            (13, 18, 20),
            (13, 22, 22),
            (14, 0, 4),
            (14, 6, 6),
            (14, 8, 8),
            (14, 10, 12),
            (14, 14, 14),
            (14, 16, 16),
            (14, 18, 20),
            (14, 22, 22),
            (15, 0, 0),
            (15, 6, 6),
            (15, 10, 10),
            (15, 14, 14),
            (15, 16, 16),
            (15, 18, 18),
            (15, 22, 22),
            (16, 0, 0),
            (16, 2, 10),
            (16, 12, 14),
            (16, 16, 16),
            (16, 18, 18),
            (16, 20, 22),
            (17, 0, 0),
            (17, 4, 6),
            (17, 10, 10),
            (17, 14, 14),
            (17, 18, 18),
            (17, 20, 22),
            (18, 0, 2),
            (18, 4, 6),
            (18, 8, 8),
            (18, 10, 12),
            (18, 14, 18),
            (18, 20, 22),
            (19, 0, 0),
            (19, 4, 4),
            (19, 8, 8),
            (19, 10, 10),
            (19, 16, 16),
            (19, 20, 22),
            (20, 0, 0),
            (20, 2, 4),
            (20, 6, 8),
            (20, 10, 10),
            (20, 12, 14),
            (20, 16, 16),
            (20, 18, 18),
            (20, 20, 22),
            (21, 0, 0),
            (21, 6, 8),
            (21, 12, 14),
            (21, 18, 18),
            (21, 22, 22),
            (22, 0, 22),
        ] {
            for c in first_c..=last_c {
                trails[position(r, c)] = Trail::Forest;
            }
        }
        trails[position(0, 1)] = Trail::Start;
        trails[position(3, 10)] = Trail::SlopeWE;
        trails[position(3, 12)] = Trail::SlopeWE;
        trails[position(4, 3)] = Trail::SlopeNS;
        trails[position(4, 11)] = Trail::SlopeNS;
        trails[position(5, 4)] = Trail::SlopeWE;
        trails[position(6, 3)] = Trail::SlopeNS;
        trails[position(10, 21)] = Trail::SlopeNS;
        trails[position(11, 20)] = Trail::SlopeWE;
        trails[position(12, 5)] = Trail::SlopeNS;
        trails[position(12, 13)] = Trail::SlopeNS;
        trails[position(12, 21)] = Trail::SlopeNS;
        trails[position(13, 6)] = Trail::SlopeWE;
        trails[position(13, 12)] = Trail::SlopeWE;
        trails[position(13, 14)] = Trail::SlopeWE;
        trails[position(14, 5)] = Trail::SlopeNS;
        trails[position(14, 13)] = Trail::SlopeNS;
        trails[position(18, 13)] = Trail::SlopeNS;
        trails[position(18, 19)] = Trail::SlopeNS;
        trails[position(19, 12)] = Trail::SlopeWE;
        trails[position(19, 14)] = Trail::SlopeWE;
        trails[position(19, 18)] = Trail::SlopeWE;
        trails[position(20, 19)] = Trail::SlopeNS;
        trails[position(22, 21)] = Trail::Finish;

        TrailMap { trails }
    }

    fn example_trail_graph() -> TrailGraph {
        let mut graph = TrailGraph {
            start: position(0, 1),
            finish: position(22, 21),
            nodes: HashMap::new(),
        };
        graph.add_connection(position(0, 1), position(5, 3), 15);
        graph.add_connection(position(5, 3), position(3, 11), 22);
        graph.add_connection(position(5, 3), position(13, 5), 22);
        graph.add_connection(position(3, 11), position(13, 13), 24);
        graph.add_connection(position(3, 11), position(11, 21), 30);
        graph.add_connection(position(11, 21), position(19, 19), 10);
        graph.add_connection(position(13, 5), position(13, 13), 12);
        graph.add_connection(position(13, 5), position(19, 13), 38);
        graph.add_connection(position(13, 13), position(11, 21), 18);
        graph.add_connection(position(13, 13), position(19, 13), 10);
        graph.add_connection(position(19, 13), position(19, 19), 10);
        graph.add_connection(position(19, 19), position(22, 21), 5);
        graph
    }

    #[test]
    fn test_parse_trail_map() {
        assert_eq!(
            TrailMap::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_trail_map()),
        );
    }

    #[test]
    fn test_trail_map_graph() {
        let trail_map = example_trail_map();
        assert_eq!(trail_map.graph(), example_trail_graph(),);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
