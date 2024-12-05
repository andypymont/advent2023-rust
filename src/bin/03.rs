use std::str::FromStr;

advent_of_code::solution!(3);

const GRID_SIZE: usize = 142;

#[derive(Debug, PartialEq)]
struct Part {
    number: u32,
    position: usize,
}

impl Part {
    fn adjacent_points(&self) -> Vec<usize> {
        let mut adjacent = Vec::new();

        let west = self.position - 1;
        let east = self.position + self.number.to_string().len();
        adjacent.push(west);
        adjacent.push(east);
        for pos in west..=east {
            adjacent.push(pos - GRID_SIZE);
            adjacent.push(pos + GRID_SIZE);
        }
        adjacent
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Gear {
    Empty,
    Single(u32),
    Valid(u32),
    Overloaded,
}

impl Gear {
    const fn connect(self, part_no: u32) -> Self {
        match self {
            Self::Empty => Self::Single(part_no),
            Self::Single(ratio) => Self::Valid(ratio * part_no),
            Self::Valid(_) | Self::Overloaded => Self::Overloaded,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Schematic {
    symbols: [char; GRID_SIZE * GRID_SIZE],
    parts: Vec<Part>,
}

impl Schematic {
    fn part_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.parts.iter().filter_map(|part| {
            if part
                .adjacent_points()
                .iter()
                .any(|pt| self.symbols[*pt] != '.')
            {
                Some(part.number)
            } else {
                None
            }
        })
    }

    fn total_gear_ratio(&self) -> u32 {
        let mut grid = [Gear::Empty; GRID_SIZE * GRID_SIZE];

        for part in &self.parts {
            for point in part.adjacent_points() {
                if self.symbols[point] == '*' {
                    grid[point] = grid[point].connect(part.number);
                }
            }
        }

        grid.iter()
            .map(|gear| match gear {
                Gear::Valid(ratio) => *ratio,
                _ => 0,
            })
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseSchematicError;

impl FromStr for Schematic {
    type Err = ParseSchematicError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut symbols = ['.'; GRID_SIZE * GRID_SIZE];
        let mut parts = Vec::new();

        for (ix, line) in text.lines().enumerate() {
            let y = ix + 1;
            let mut number_began: Option<usize> = None;
            let mut number = 0;

            for (ix, ch) in line.chars().enumerate() {
                let pos = (y * GRID_SIZE) + ix + 1;
                if let Some(digit) = ch.to_digit(10) {
                    if number_began.is_some() {
                        number = (number * 10) + digit;
                    } else {
                        number_began = Some(pos);
                        number = digit;
                    }
                } else {
                    if let Some(position) = number_began {
                        parts.push(Part { number, position });
                        number_began = None;
                        number = 0;
                    }
                    symbols[pos] = ch;
                }
            }

            if let Some(position) = number_began {
                parts.push(Part { number, position });
            }
        }

        Ok(Self { symbols, parts })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    Schematic::from_str(input).map_or(None, |schematic| Some(schematic.part_numbers().sum()))
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    Schematic::from_str(input).map_or(None, |schematic| Some(schematic.total_gear_ratio()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(x: usize, y: usize) -> usize {
        (y * GRID_SIZE) + x
    }

    fn example_schematic() -> Schematic {
        let mut symbols = ['.'; GRID_SIZE * GRID_SIZE];
        symbols[position(4, 2)] = '*';
        symbols[position(7, 4)] = '#';
        symbols[position(4, 5)] = '*';
        symbols[position(6, 6)] = '+';
        symbols[position(4, 9)] = '$';
        symbols[position(6, 9)] = '*';

        let parts = vec![
            Part {
                number: 467,
                position: position(1, 1),
            },
            Part {
                number: 114,
                position: position(6, 1),
            },
            Part {
                number: 35,
                position: position(3, 3),
            },
            Part {
                number: 633,
                position: position(7, 3),
            },
            Part {
                number: 617,
                position: position(1, 5),
            },
            Part {
                number: 58,
                position: position(8, 6),
            },
            Part {
                number: 592,
                position: position(3, 7),
            },
            Part {
                number: 755,
                position: position(7, 8),
            },
            Part {
                number: 664,
                position: position(2, 10),
            },
            Part {
                number: 598,
                position: position(6, 10),
            },
        ];

        Schematic { symbols, parts }
    }

    #[test]
    fn test_parse_schematic() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse(),
            Ok(example_schematic())
        );
    }

    #[test]
    fn test_adjacent_points_single_digit() {
        let part = Part {
            number: 4,
            position: position(1, 1),
        };
        let mut adjacent = part.adjacent_points();
        adjacent.sort_unstable();

        assert_eq!(
            adjacent,
            vec![
                position(0, 0),
                position(1, 0),
                position(2, 0),
                position(0, 1),
                position(2, 1),
                position(0, 2),
                position(1, 2),
                position(2, 2),
            ]
        );
    }

    #[test]
    fn test_adjacent_points_three_digits() {
        let part = Part {
            number: 437,
            position: position(1, 1),
        };
        let mut adjacent = part.adjacent_points();
        adjacent.sort_unstable();

        assert_eq!(
            adjacent,
            vec![
                position(0, 0),
                position(1, 0),
                position(2, 0),
                position(3, 0),
                position(4, 0),
                position(0, 1),
                position(4, 1),
                position(0, 2),
                position(1, 2),
                position(2, 2),
                position(3, 2),
                position(4, 2),
            ]
        );
    }

    #[test]
    fn test_part_numbers() {
        let parts: Vec<u32> = example_schematic().part_numbers().collect();
        assert_eq!(parts.len(), 8, "8 valid part numbers (next to symbols)");
        assert_eq!(
            parts.iter().sum::<u32>(),
            4361,
            "valid part numbers sum to 4361"
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
