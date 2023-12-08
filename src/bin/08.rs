use std::collections::HashMap;
use std::convert::TryFrom;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq)]
enum MapNode<'a> {
    Fork(&'a str, &'a str),
    DeadEnd,
}

#[derive(Debug, PartialEq)]
struct Network<'a> {
    directions: &'a str,
    map: HashMap<&'a str, MapNode<'a>>,
}

impl Network<'_> {
    fn steps_to_reach(&self, start: &str, target: &str) -> Option<usize> {
        let mut position = start;
        let mut steps = 0;

        while position != target {
            position = match self.map.get(position).unwrap_or(&MapNode::DeadEnd) {
                MapNode::Fork(left, right) => {
                    match self.directions.chars().nth(steps % self.directions.len()) {
                        Some('L') => left,
                        Some('R') => right,
                        _ => return None,
                    }
                }
                MapNode::DeadEnd => return None,
            };
            steps += 1;
        }

        Some(steps)
    }
}

#[derive(Debug, PartialEq)]
struct ParseNetworkError;

impl<'a> TryFrom<&'a str> for Network<'a> {
    type Error = ParseNetworkError;

    fn try_from(input: &str) -> Result<Network, Self::Error> {
        if let Some((directions, map_text)) = input.split_once("\n\n") {
            let mut map = HashMap::new();

            for line in map_text.lines() {
                let Some((key, node_desc)) = line.split_once(" = ") else {
                    return Err(ParseNetworkError);
                };

                let Some((left, right)) = node_desc.split_once(", ") else {
                    return Err(ParseNetworkError);
                };

                let Some(left) = left.strip_prefix('(') else {
                    return Err(ParseNetworkError);
                };
                let Some(right) = right.strip_suffix(')') else {
                    return Err(ParseNetworkError);
                };

                let node = if key == left && left == right {
                    MapNode::DeadEnd
                } else {
                    MapNode::Fork(left, right)
                };

                map.insert(key, node);
            }

            Ok(Network { directions, map })
        } else {
            Err(ParseNetworkError)
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(network) = Network::try_from(input) {
        network.steps_to_reach("AAA", "ZZZ")
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

    #[test]
    fn test_parse_input() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let parsed = Network::try_from(input).expect("Input parsing returned Ok");
        assert_eq!(parsed.directions, "RL");
        assert_eq!(parsed.map.len(), 7);
        assert_eq!(parsed.map.get("AAA"), Some(&MapNode::Fork("BBB", "CCC")));
        assert_eq!(parsed.map.get("BBB"), Some(&MapNode::Fork("DDD", "EEE")));
        assert_eq!(parsed.map.get("CCC"), Some(&MapNode::Fork("ZZZ", "GGG")));
        assert_eq!(parsed.map.get("DDD"), Some(&MapNode::DeadEnd));
        assert_eq!(parsed.map.get("EEE"), Some(&MapNode::DeadEnd));
        assert_eq!(parsed.map.get("GGG"), Some(&MapNode::DeadEnd));
        assert_eq!(parsed.map.get("ZZZ"), Some(&MapNode::DeadEnd));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
