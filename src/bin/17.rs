use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

advent_of_code::solution!(17);

const GRID_SIZE: usize = 141;

#[derive(Copy, Clone, Eq, Debug, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct JourneyState {
    position: usize,
    facing: Direction,
    heat_loss: u32,
}

impl Ord for JourneyState {
    fn cmp(&self, other: &Self) -> Ordering {
        // for use in max heap, so less heat loss = better/greater
        match self.heat_loss.cmp(&other.heat_loss) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => (self.position, self.facing).cmp(&(other.position, other.facing)),
        }
    }
}

impl PartialOrd for JourneyState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // for use in max heap, so less heat_loss = better/greater
        match self.heat_loss.cmp(&other.heat_loss) {
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => {
                (self.position, self.facing).partial_cmp(&(other.position, other.facing))
            }
        }
    }
}

#[derive(Debug)]
struct JourneyVisitTracker {
    visited: [u32; GRID_SIZE * GRID_SIZE * 4],
}

impl JourneyVisitTracker {
    fn new() -> Self {
        Self {
            visited: [u32::MAX; GRID_SIZE * GRID_SIZE * 4],
        }
    }

    fn minimum(&self, pos: usize) -> Option<u32> {
        let base = pos * 4;
        self.visited[base..base + 4].iter().min().copied()
    }

    fn visit(&mut self, state: &JourneyState) -> bool {
        let dir = match state.facing {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };
        let key = (4 * state.position) + dir;

        if self.visited[key] <= state.heat_loss {
            true
        } else {
            self.visited[key] = state.heat_loss;
            false
        }
    }
}

#[derive(Debug, PartialEq)]
struct City {
    grid: [u32; GRID_SIZE * GRID_SIZE],
}

impl City {
    fn minimal_heat_loss(&self, min_dist: usize, max_dist: usize) -> Option<u32> {
        let mut visited = JourneyVisitTracker::new();
        let mut queue = BinaryHeap::new();
        for state in self.initial_states(min_dist, max_dist) {
            if !visited.visit(&state) {
                queue.push(state);
            }
        }

        while let Some(state) = queue.pop() {
            for reachable in self.reachable_states(&state, min_dist, max_dist) {
                if !visited.visit(&reachable) {
                    queue.push(reachable);
                }
            }
        }

        visited.minimum((GRID_SIZE * GRID_SIZE) - 1)
    }

    fn states_in_directions<'a>(
        &'a self,
        position: usize,
        heat_loss: u32,
        directions: impl Iterator<Item = Direction> + 'a,
        min_dist: usize,
        max_dist: usize,
    ) -> impl Iterator<Item = JourneyState> + 'a {
        directions.flat_map(move |facing| {
            let mut states = Vec::new();
            let mut extra_loss = 0;
            for dist in 1..=max_dist {
                if let Some(position) = City::step(position, facing, dist) {
                    extra_loss += self.grid[position];
                    if dist >= min_dist {
                        states.push(JourneyState {
                            position,
                            facing,
                            heat_loss: heat_loss + extra_loss,
                        });
                    }
                }
            }
            states
        })
    }

    fn initial_states(
        &self,
        min_dist: usize,
        max_dist: usize,
    ) -> impl Iterator<Item = JourneyState> + '_ {
        self.states_in_directions(
            0,
            0,
            [Direction::East, Direction::South].into_iter(),
            min_dist,
            max_dist,
        )
    }

    fn reachable_states<'a>(
        &'a self,
        state: &'a JourneyState,
        min_dist: usize,
        max_dist: usize,
    ) -> impl Iterator<Item = JourneyState> + '_ {
        self.states_in_directions(
            state.position,
            state.heat_loss,
            [state.facing.turn_left(), state.facing.turn_right()].into_iter(),
            min_dist,
            max_dist,
        )
    }

    fn step(pos: usize, dir: Direction, dist: usize) -> Option<usize> {
        let row = pos / GRID_SIZE;
        let col = pos % GRID_SIZE;
        match dir {
            Direction::North => pos.checked_sub(GRID_SIZE * dist),
            Direction::East => {
                if (col + dist) < GRID_SIZE {
                    Some(pos + dist)
                } else {
                    None
                }
            }
            Direction::South => {
                if (row + dist) < GRID_SIZE {
                    Some(pos + (GRID_SIZE * dist))
                } else {
                    None
                }
            }
            Direction::West => {
                if col.checked_sub(dist).is_some() {
                    Some(pos - dist)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseCityError;

impl FromStr for City {
    type Err = ParseCityError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = [0; GRID_SIZE * GRID_SIZE];

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if let Some(value) = ch.to_digit(10) {
                    grid[(row * GRID_SIZE) + col] = value;
                } else {
                    return Err(ParseCityError);
                }
            }
        }

        Ok(City { grid })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(city) = City::from_str(input) {
        city.minimal_heat_loss(1, 3)
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    if let Ok(city) = City::from_str(input) {
        city.minimal_heat_loss(4, 10)
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

    fn example_city() -> City {
        let mut grid = [0; GRID_SIZE * GRID_SIZE];

        for (row, values) in [
            [2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
            [3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
            [3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
            [3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
            [4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
            [1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
            [4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
            [3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
            [4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
            [4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
            [1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
            [2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
            [4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3],
        ]
        .into_iter()
        .enumerate()
        {
            for (col, value) in values.into_iter().enumerate() {
                grid[(row * GRID_SIZE) + col] = value;
            }
        }

        City { grid }
    }

    #[test]
    fn test_direction_turn_left() {
        assert_eq!(Direction::North.turn_left(), Direction::West);
        assert_eq!(Direction::East.turn_left(), Direction::North);
        assert_eq!(Direction::South.turn_left(), Direction::East);
        assert_eq!(Direction::West.turn_left(), Direction::South);
    }

    #[test]
    fn test_direction_turn_right() {
        assert_eq!(Direction::North.turn_right(), Direction::East);
        assert_eq!(Direction::East.turn_right(), Direction::South);
        assert_eq!(Direction::South.turn_right(), Direction::West);
        assert_eq!(Direction::West.turn_right(), Direction::North);
    }

    #[test]
    fn test_city_step() {
        assert_eq!(
            City::step(position(2, 5), Direction::North, 1),
            Some(position(1, 5))
        );
        assert_eq!(
            City::step(position(3, 7), Direction::East, 1),
            Some(position(3, 8))
        );
        assert_eq!(
            City::step(position(1, 13), Direction::South, 1),
            Some(position(2, 13))
        );
        assert_eq!(
            City::step(position(0, 9), Direction::West, 1),
            Some(position(0, 8))
        );
    }

    #[test]
    fn test_city_step_oob() {
        assert_eq!(City::step(position(0, 4), Direction::North, 1), None);
        assert_eq!(
            City::step(position(4, GRID_SIZE - 1), Direction::East, 1),
            None
        );
        assert_eq!(
            City::step(position(GRID_SIZE - 1, 12), Direction::South, 1),
            None
        );
        assert_eq!(City::step(position(1, 0), Direction::West, 1), None);
    }

    #[test]
    fn test_parse_city() {
        let city: City = advent_of_code::template::read_file("examples", DAY)
            .parse()
            .expect("No errors during City parsing");
        assert_eq!(city, example_city());
    }

    #[test]
    fn test_journey_state_heap_order() {
        let a = JourneyState {
            position: position(3, 2),
            facing: Direction::East,
            heat_loss: 4,
        };
        let b = JourneyState {
            position: position(7, 15),
            facing: Direction::South,
            heat_loss: 2,
        };
        let c = JourneyState {
            position: position(7, 13),
            facing: Direction::South,
            heat_loss: 2,
        };

        let mut heap = BinaryHeap::new();
        heap.push(a.clone());
        heap.push(b.clone());
        heap.push(c.clone());

        assert_eq!(heap.pop(), Some(b));
        assert_eq!(heap.pop(), Some(c));
        assert_eq!(heap.pop(), Some(a));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_reachable_states() {
        let city = example_city();
        let state = JourneyState {
            position: position(1, 4),
            facing: Direction::East,
            heat_loss: 15,
        };
        let reachable: Vec<JourneyState> = city.reachable_states(&state, 1, 3).collect();
        assert_eq!(
            reachable,
            vec![
                JourneyState {
                    position: position(0, 4),
                    facing: Direction::North,
                    heat_loss: 19,
                },
                JourneyState {
                    position: position(2, 4),
                    facing: Direction::South,
                    heat_loss: 17,
                },
                JourneyState {
                    position: position(3, 4),
                    facing: Direction::South,
                    heat_loss: 22,
                },
                JourneyState {
                    position: position(4, 4),
                    facing: Direction::South,
                    heat_loss: 28,
                },
            ],
        );
    }

    #[test]
    fn test_journey_visit_tracker() {
        let mut jvt = JourneyVisitTracker::new();

        let state = JourneyState {
            position: position(2, 3),
            facing: Direction::East,
            heat_loss: 12,
        };
        assert_eq!(jvt.visit(&state), false);

        let diff_pos = JourneyState {
            position: position(3, 2),
            facing: Direction::East,
            heat_loss: 11,
        };
        assert_eq!(jvt.visit(&diff_pos), false);

        let diff_dir = JourneyState {
            position: position(2, 3),
            facing: Direction::South,
            heat_loss: 7,
        };
        assert_eq!(jvt.visit(&diff_dir), false);

        let less_lost = JourneyState {
            position: position(2, 3),
            facing: Direction::East,
            heat_loss: 8,
        };
        assert_eq!(jvt.visit(&less_lost), false);

        let more_lost = JourneyState {
            position: position(2, 3),
            facing: Direction::East,
            heat_loss: 13,
        };
        assert_eq!(jvt.visit(&more_lost), true);

        assert_eq!(jvt.minimum(position(2, 3)), Some(7));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_initial_states_mega_crucible() {
        let city = example_city();
        let initial: Vec<JourneyState> = city.initial_states(4, 10).collect();
        assert_eq!(
            initial,
            vec![
                JourneyState {
                    position: position(0, 4),
                    facing: Direction::East,
                    heat_loss: 12,
                },
                JourneyState {
                    position: position(0, 5),
                    facing: Direction::East,
                    heat_loss: 15,
                },
                JourneyState {
                    position: position(0, 6),
                    facing: Direction::East,
                    heat_loss: 17,
                },
                JourneyState {
                    position: position(0, 7),
                    facing: Direction::East,
                    heat_loss: 20,
                },
                JourneyState {
                    position: position(0, 8),
                    facing: Direction::East,
                    heat_loss: 21,
                },
                JourneyState {
                    position: position(0, 9),
                    facing: Direction::East,
                    heat_loss: 22,
                },
                JourneyState {
                    position: position(0, 10),
                    facing: Direction::East,
                    heat_loss: 25,
                },
                JourneyState {
                    position: position(4, 0),
                    facing: Direction::South,
                    heat_loss: 13,
                },
                JourneyState {
                    position: position(5, 0),
                    facing: Direction::South,
                    heat_loss: 14,
                },
                JourneyState {
                    position: position(6, 0),
                    facing: Direction::South,
                    heat_loss: 18,
                },
                JourneyState {
                    position: position(7, 0),
                    facing: Direction::South,
                    heat_loss: 21,
                },
                JourneyState {
                    position: position(8, 0),
                    facing: Direction::South,
                    heat_loss: 25,
                },
                JourneyState {
                    position: position(9, 0),
                    facing: Direction::South,
                    heat_loss: 29,
                },
                JourneyState {
                    position: position(10, 0),
                    facing: Direction::South,
                    heat_loss: 30,
                },
            ],
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
