use std::collections::{HashMap, HashSet};
use std::str::FromStr;

advent_of_code::solution!(3);

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point(usize, usize);

#[derive(Debug, Eq, Hash, PartialEq)]
struct Part {
    number: u32,
    position: Point,
}

impl Part {
    fn adjacent_points(&self) -> HashSet<Point> {
        let mut adjacent = HashSet::new();

        let west = self.position.0 - 1;
        let east = self.position.0 + self.number.to_string().len();
        adjacent.insert(Point(west, self.position.1));
        adjacent.insert(Point(east, self.position.1));
        for x in west..=east {
            adjacent.insert(Point(x, self.position.1 - 1));
            adjacent.insert(Point(x, self.position.1 + 1));
        }
        adjacent
    }
}

#[derive(Debug)]
struct Schematic {
    symbols: HashMap<Point, char>,
    parts: HashSet<Part>,
}

impl Schematic {
    fn part_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.parts.iter().filter_map(|part| {
            if part.adjacent_points().iter().any(|pt| self.symbols.contains_key(pt)) {
                Some(part.number)
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
struct ParseSchematicError;

impl FromStr for Schematic {
    type Err = ParseSchematicError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut symbols = HashMap::new();
        let mut parts = HashSet::new();

        for (ix, line) in text.lines().enumerate() {
            let y = ix + 1;
            let mut number_began: Option<usize> = None;
            let mut number = 0;

            for (ix, ch) in line.chars().enumerate() {
                let x = ix + 1;
                if let Some(digit) = ch.to_digit(10) {
                    if number_began.is_some() {
                        number = (number * 10) + digit;
                    } else {
                        number_began = Some(x);
                        number = digit;
                    }
                } else {
                    if let Some(began_x) = number_began {
                        parts.insert(Part { number, position: Point(began_x, y) });
                        number_began = None;
                        number = 0;
                    }
                    if ch != '.' {
                        symbols.insert(Point(x, y), ch);
                    }
                }
            }

            if let Some(began_x) = number_began {
                parts.insert(Part { number, position: Point(began_x, y) });
            }
        }

        Ok(Schematic { symbols, parts})
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(schematic) = input.parse::<Schematic>() {
        Some(schematic.part_numbers().sum())
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

    fn example_schematic() -> Schematic {
        let mut symbols = HashMap::new();
        symbols.insert(Point(4, 2), '*');
        symbols.insert(Point(7, 4), '#');
        symbols.insert(Point(4, 5), '*');
        symbols.insert(Point(6, 6), '+');
        symbols.insert(Point(4, 9), '$');
        symbols.insert(Point(6, 9), '*');

        let mut parts = HashSet::new();
        parts.insert(Part { number: 467, position: Point(1, 1) });
        parts.insert(Part { number: 114, position: Point(6, 1) });
        parts.insert(Part { number: 35, position: Point(3, 3) });
        parts.insert(Part { number: 633, position: Point(7, 3) });
        parts.insert(Part { number: 617, position: Point(1, 5) });
        parts.insert(Part { number: 58, position: Point(8, 6) });
        parts.insert(Part { number: 592, position: Point(3, 7) });
        parts.insert(Part { number: 755, position: Point(7, 8) });
        parts.insert(Part { number: 664, position: Point(2, 10) });
        parts.insert(Part { number: 598, position: Point(6, 10) });

        Schematic { symbols, parts }
    }

    #[test]
    fn test_parse_schematic() {
        let parsed: Schematic = advent_of_code::template::read_file("examples", DAY).parse().unwrap();
        let expected = example_schematic();

        assert_eq!(parsed.symbols.len(), expected.symbols.len(), "number of symbols matches expected");
        for (point, symbol) in expected.symbols {
            assert_eq!(parsed.symbols.get(&point), Some(&symbol), "check for {symbol} at {point:?}");
        }

        assert_eq!(parsed.parts.len(), expected.parts.len(), "number of parts matches expected");
        for part in expected.parts {
            assert!(parsed.parts.contains(&part), "check for {part:?}");
        }
    }

    #[test]
    fn test_adjacent_points_single_digit() {
        let part = Part { number: 4, position: Point(1, 1) };
        let adjacent = part.adjacent_points();
        assert_eq!(adjacent.len(), 8, "8 adjacent points");
        assert!(adjacent.contains(&Point(0, 0)), "NW");
        assert!(adjacent.contains(&Point(1, 0)), "N");
        assert!(adjacent.contains(&Point(2, 0)), "NE");
        assert!(adjacent.contains(&Point(0, 1)), "W");
        assert!(adjacent.contains(&Point(2, 1)), "E");
        assert!(adjacent.contains(&Point(0, 2)), "SW");
        assert!(adjacent.contains(&Point(1, 2)), "S");
        assert!(adjacent.contains(&Point(2, 2)), "SE");
    }

    #[test]
    fn test_adjacent_points_three_digits() {
        let part = Part { number: 437, position: Point(1, 1) };
        let adjacent = part.adjacent_points();
        assert_eq!(adjacent.len(), 12, "12 adjacent points");
        assert!(adjacent.contains(&Point(0, 0)), "NW");
        assert!(adjacent.contains(&Point(1, 0)), "N 1 of 3");
        assert!(adjacent.contains(&Point(2, 0)), "N 2 of 3");
        assert!(adjacent.contains(&Point(3, 0)), "N 3 of 3");
        assert!(adjacent.contains(&Point(4, 0)), "NE");
        assert!(adjacent.contains(&Point(0, 1)), "W");
        assert!(adjacent.contains(&Point(4, 1)), "E");
        assert!(adjacent.contains(&Point(0, 2)), "SW");
        assert!(adjacent.contains(&Point(1, 2)), "S 1 of 3");
        assert!(adjacent.contains(&Point(2, 2)), "S 2 of 3");
        assert!(adjacent.contains(&Point(3, 2)), "S 3 of 3");
        assert!(adjacent.contains(&Point(4, 2)), "SE");
    }

    #[test]
    fn test_part_numbers() {
        let parts: Vec<u32> = example_schematic().part_numbers().collect();
        assert_eq!(parts.len(), 8, "8 valid part numbers (next to symbols)");
        assert_eq!(parts.iter().sum::<u32>(), 4361, "valid part numbers sum to 4361");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
