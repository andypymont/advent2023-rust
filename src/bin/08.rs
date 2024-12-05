use std::collections::HashMap;
use std::str::FromStr;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq)]
struct ParseNetworkError;

type Location = [char; 3];

fn parse_location(input: &str) -> Result<Location, ParseNetworkError> {
    let mut parsed: Location = [' '; 3];

    for (ix, ch) in input.chars().enumerate() {
        if ix > 2 {
            return Err(ParseNetworkError);
        }
        parsed[ix] = ch;
    }

    Ok(parsed)
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ParseNetworkError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(ParseNetworkError),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    left: Location,
    right: Location,
}

impl Node {
    const fn step(&self, direction: &Direction) -> Location {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Network {
    directions: Vec<Direction>,
    graph: HashMap<Location, Node>,
}

impl Network {
    fn find_path_length(&self, origin: Location) -> Option<usize> {
        let mut steps = 0;
        let mut location = origin;

        while location[2] != 'Z' {
            let Some(node) = self.graph.get(&location) else {
                return None; // reached invalid location
            };

            location = node.step(&self.directions[steps % self.directions.len()]);
            steps += 1;
        }

        Some(steps)
    }

    fn find_all_path_lengths(&self) -> impl Iterator<Item = usize> + '_ {
        self.graph.keys().filter_map(|loc| {
            if loc[2] == 'A' {
                self.find_path_length(*loc)
            } else {
                None
            }
        })
    }
}

impl FromStr for Network {
    type Err = ParseNetworkError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some((directions_str, graph_str)) = input.split_once("\n\n") else {
            return Err(ParseNetworkError);
        };

        let mut directions = Vec::new();

        for dir in directions_str.chars() {
            let dir = dir.try_into()?;
            directions.push(dir);
        }

        let mut graph = HashMap::new();
        for line in graph_str.lines() {
            let Some((loc_str, node_str)) = line.split_once(" = ") else {
                return Err(ParseNetworkError);
            };

            let Some((left, right)) = node_str.split_once(", ") else {
                return Err(ParseNetworkError);
            };

            let Some(left) = left.strip_prefix('(') else {
                return Err(ParseNetworkError);
            };

            let Some(right) = right.strip_suffix(')') else {
                return Err(ParseNetworkError);
            };

            let loc = parse_location(loc_str)?;
            let left = parse_location(left)?;
            let right = parse_location(right)?;

            let node = Node { left, right };
            graph.insert(loc, node);
        }

        Ok(Self { directions, graph })
    }
}

const fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    if a == 0 || b == 0 {
        a | b
    } else {
        let shift = (a | b).trailing_zeros();

        a >>= a.trailing_zeros();
        b >>= b.trailing_zeros();

        while a != b {
            if a > b {
                a -= b;
                a >>= a.trailing_zeros();
            } else {
                b -= a;
                b >>= b.trailing_zeros();
            }
        }

        a << shift
    }
}

const fn lcm(a: usize, b: usize) -> usize {
    if a == 0 && b == 0 {
        0
    } else {
        a * (b / gcd(a, b))
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Network::from_str(input).map_or(None, |network| network.find_path_length(['A', 'A', 'A']))
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Network::from_str(input).map_or(None, |network| network.find_all_path_lengths().reduce(lcm))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_network() -> Network {
        let directions = vec![Direction::Left, Direction::Right];

        let mut graph = HashMap::new();
        graph.insert(
            ['A', 'A', 'A'],
            Node {
                left: ['B', 'B', 'B'],
                right: ['I', 'I', 'I'],
            },
        );
        graph.insert(
            ['B', 'B', 'B'],
            Node {
                left: ['E', 'E', 'K'],
                right: ['C', 'C', 'C'],
            },
        );
        graph.insert(
            ['C', 'C', 'C'],
            Node {
                left: ['D', 'D', 'D'],
                right: ['J', 'J', 'J'],
            },
        );
        graph.insert(
            ['D', 'D', 'D'],
            Node {
                left: ['E', 'E', 'K'],
                right: ['E', 'E', 'E'],
            },
        );
        graph.insert(
            ['E', 'E', 'E'],
            Node {
                left: ['F', 'F', 'F'],
                right: ['F', 'E', 'Z'],
            },
        );
        graph.insert(
            ['F', 'F', 'F'],
            Node {
                left: ['E', 'E', 'K'],
                right: ['G', 'G', 'G'],
            },
        );
        graph.insert(
            ['G', 'G', 'G'],
            Node {
                left: ['H', 'H', 'H'],
                right: ['E', 'E', 'K'],
            },
        );
        graph.insert(
            ['H', 'H', 'H'],
            Node {
                left: ['E', 'E', 'K'],
                right: ['Z', 'Z', 'Z'],
            },
        );
        graph.insert(
            ['Z', 'Z', 'Z'],
            Node {
                left: ['B', 'B', 'B'],
                right: ['E', 'E', 'K'],
            },
        );
        graph.insert(
            ['G', 'O', 'A'],
            Node {
                left: ['A', 'A', 'A'],
                right: ['E', 'E', 'K'],
            },
        );
        graph.insert(
            ['I', 'I', 'I'],
            Node {
                left: ['C', 'C', 'C'],
                right: ['E', 'E', 'K'],
            },
        );
        graph.insert(
            ['J', 'J', 'J'],
            Node {
                left: ['E', 'E', 'E'],
                right: ['E', 'E', 'K'],
            },
        );
        graph.insert(
            ['F', 'E', 'Z'],
            Node {
                left: ['A', 'A', 'A'],
                right: ['E', 'E', 'K'],
            },
        );
        graph.insert(
            ['E', 'E', 'K'],
            Node {
                left: ['G', 'O', 'A'],
                right: ['A', 'A', 'A'],
            },
        );

        Network { directions, graph }
    }

    #[test]
    fn test_parse_graph() {
        assert_eq!(
            Network::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_network()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_find_all_path_lengths() {
        let lengths: Vec<usize> = example_network().find_all_path_lengths().collect();
        assert_eq!(lengths.len(), 2);
        assert!(lengths.contains(&8));
        assert!(lengths.contains(&6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
