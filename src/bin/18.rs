use std::ops::Add;
use std::str::FromStr;

advent_of_code::solution!(18);

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

#[derive(Debug, PartialEq)]
struct ParseInstructionError;

impl FromStr for Direction {
    type Err = ParseInstructionError;

    fn from_str(part: &str) -> Result<Self, Self::Err> {
        match part.chars().next() {
            Some('U') => Ok(Self::Up),
            Some('R') => Ok(Self::Right),
            Some('D') => Ok(Self::Down),
            Some('L') => Ok(Self::Left),
            _ => Err(ParseInstructionError),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut direction: Result<Direction, ParseInstructionError> = Err(ParseInstructionError);
        let mut distance: Result<i32, ParseInstructionError> = Err(ParseInstructionError);

        for (part_ix, part) in line.split_whitespace().enumerate() {
            match part_ix {
                0 => direction = Direction::from_str(part),
                1 => distance = part.parse().map_err(|_| ParseInstructionError),
                2 => (),
                _ => return Err(ParseInstructionError),
            }
        }

        let direction = direction?;
        let distance = distance?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn distance(self, other: Point) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl Add<&Instruction> for Point {
    type Output = Point;

    fn add(self, rhs: &Instruction) -> Self::Output {
        match rhs.direction {
            Direction::Up => Self(self.0, self.1 - rhs.distance),
            Direction::Right => Self(self.0 + rhs.distance, self.1),
            Direction::Down => Self(self.0, self.1 + rhs.distance),
            Direction::Left => Self(self.0 - rhs.distance, self.1),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Polygon {
    points: Vec<Point>,
}

#[derive(Debug, PartialEq)]
struct PolygonGeometryError;

impl Polygon {
    fn area(&self) -> u32 {
        let (a, b) = self
            .points
            .iter()
            .enumerate()
            .fold((0, 0), |(a, b), (ix, point)| {
                let next = self.points[(ix + 1) % self.points.len()];
                (a + (point.0 * next.1), b + (point.1 * next.0))
            });
        a.abs_diff(b) / 2
    }

    fn circumference(&self) -> u32 {
        let (_, circ) = self
            .points
            .iter()
            .fold((Point(0, 0), 0), |(prev, circ), point| {
                (*point, circ + point.distance(prev))
            });
        circ
    }

    fn area_including_circumference(&self) -> u32 {
        self.area() + (self.circumference() / 2 + 1)
    }

    fn from_instructions(instructions: &[Instruction]) -> Result<Self, PolygonGeometryError> {
        let mut points = Vec::new();
        let mut point = Point(0, 0);

        for instruction in instructions {
            point = point + instruction;
            points.push(point);
        }

        if point == Point(0, 0) {
            Ok(Self { points })
        } else {
            Err(PolygonGeometryError)
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        if let Ok(instruction) = line.parse() {
            instructions.push(instruction);
        } else {
            return None;
        }
    }

    if let Ok(polygon) = Polygon::from_instructions(&instructions) {
        Some(polygon.area_including_circumference())
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

    fn example_instructions() -> Vec<Instruction> {
        vec![
            Instruction {
                direction: Direction::Right,
                distance: 6,
            },
            Instruction {
                direction: Direction::Down,
                distance: 5,
            },
            Instruction {
                direction: Direction::Left,
                distance: 2,
            },
            Instruction {
                direction: Direction::Down,
                distance: 2,
            },
            Instruction {
                direction: Direction::Right,
                distance: 2,
            },
            Instruction {
                direction: Direction::Down,
                distance: 2,
            },
            Instruction {
                direction: Direction::Left,
                distance: 5,
            },
            Instruction {
                direction: Direction::Up,
                distance: 2,
            },
            Instruction {
                direction: Direction::Left,
                distance: 1,
            },
            Instruction {
                direction: Direction::Up,
                distance: 2,
            },
            Instruction {
                direction: Direction::Right,
                distance: 2,
            },
            Instruction {
                direction: Direction::Up,
                distance: 3,
            },
            Instruction {
                direction: Direction::Left,
                distance: 2,
            },
            Instruction {
                direction: Direction::Up,
                distance: 2,
            },
        ]
    }

    fn example_polygon() -> Polygon {
        Polygon {
            points: vec![
                Point(6, 0),
                Point(6, 5),
                Point(4, 5),
                Point(4, 7),
                Point(6, 7),
                Point(6, 9),
                Point(1, 9),
                Point(1, 7),
                Point(0, 7),
                Point(0, 5),
                Point(2, 5),
                Point(2, 2),
                Point(0, 2),
                Point(0, 0),
            ],
        }
    }

    #[test]
    fn test_instruction_from_str() {
        assert_eq!(
            Instruction::from_str("R 6 (#70c710)"),
            Ok(Instruction {
                direction: Direction::Right,
                distance: 6,
            }),
        );
        assert_eq!(
            Instruction::from_str("U 2 (#caa171)"),
            Ok(Instruction {
                direction: Direction::Up,
                distance: 2,
            }),
        );
    }

    #[test]
    fn test_polygon_from_instructions() {
        assert_eq!(
            Polygon::from_instructions(&example_instructions()),
            Ok(example_polygon()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
