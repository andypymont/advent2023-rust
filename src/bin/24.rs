use std::str::FromStr;

advent_of_code::solution!(24);

#[derive(Debug, PartialEq)]
struct Point2D(i64, i64);

#[derive(Debug, PartialEq)]
enum Intersection {
    None,
    Past,
    Future { x: i64, y: i64 },
}

#[derive(Debug, PartialEq)]
struct ParseHailstoneError;

#[derive(Debug, PartialEq)]
struct Hailstone2D {
    position: Point2D,
    velocity: Point2D,
}

impl FromStr for Point2D {
    type Err = ParseHailstoneError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut x: Result<i64, Self::Err> = Err(ParseHailstoneError);
        let mut y: Result<i64, Self::Err> = Err(ParseHailstoneError);

        for (pos, part) in text.split(", ").enumerate() {
            let value = part.trim().parse().map_err(|_| ParseHailstoneError);
            match pos {
                0 => x = value,
                1 => y = value,
                2 => (),
                _ => return Err(ParseHailstoneError),
            }
        }

        let x = x?;
        let y = y?;
        Ok(Point2D(x, y))
    }
}

impl FromStr for Hailstone2D {
    type Err = ParseHailstoneError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if let Some((position, velocity)) = text.split_once(" @ ") {
            let position = position.parse()?;
            let velocity = velocity.parse()?;
            Ok(Hailstone2D { position, velocity })
        } else {
            Err(ParseHailstoneError)
        }
    }
}

impl Hailstone2D {
    fn intersection(&self, other: &Self) -> Intersection {
        if self.velocity.0 == 0 && self.velocity.1 == 0 {
            return Intersection::None;
        }
        if other.velocity.0 == 0 && other.velocity.1 == 0 {
            return Intersection::None;
        }

        let x1 = self.position.0;
        let y1 = self.position.1;
        let x2 = self.position.0 + self.velocity.0;
        let y2 = self.position.1 + self.velocity.1;
        let x3 = other.position.0;
        let y3 = other.position.1;
        let x4 = other.position.0 + other.velocity.0;
        let y4 = other.position.1 + other.velocity.1;

        let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
        if denominator == 0 {
            // lines are parallel
            return Intersection::None;
        }

        // Calculate distance in tenths of the y1-y2 segment
        let ua = 10 * ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;
        let ub = 10 * ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denominator;

        if (ua < 0) || (ub < 0) {
            return Intersection::Past;
        }

        let x = ((x1 * 10) + (ua * (x2 - x1))) / 10;
        let y = ((y1 * 10) + (ua * (y2 - y1))) / 10;
        Intersection::Future { x, y }
    }
}

fn read_hailstones_2d(input: &str) -> Result<Vec<Hailstone2D>, ParseHailstoneError> {
    let mut hailstones = Vec::new();

    for line in input.lines() {
        let hailstone = line.parse()?;
        hailstones.push(hailstone);
    }

    Ok(hailstones)
}

#[derive(Debug, PartialEq)]
struct TestArea {
    min: i64,
    max: i64,
}

impl TestArea {
    fn is_value_in_range(&self, value: i64) -> bool {
        (value >= self.min) && (value <= self.max)
    }

    fn contains(&self, intersection: &Intersection) -> bool {
        match intersection {
            Intersection::None | Intersection::Past => false,
            Intersection::Future { x, y } => {
                self.is_value_in_range(*x) && self.is_value_in_range(*y)
            }
        }
    }
}

fn intersecting_pairs_in_area(hailstones: &[Hailstone2D], area: &TestArea) -> u32 {
    let mut count = 0;
    for (ix, first) in hailstones.iter().enumerate() {
        for second in &hailstones[(ix + 1)..] {
            count += u32::from(area.contains(&first.intersection(second)));
        }
    }
    count
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(hailstones) = read_hailstones_2d(input) {
        Some(intersecting_pairs_in_area(
            &hailstones,
            &TestArea {
                min: 200_000_000_000_000,
                max: 400_000_000_000_000,
            },
        ))
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

    fn example_hailstones_2d() -> Vec<Hailstone2D> {
        vec![
            Hailstone2D {
                position: Point2D(19, 13),
                velocity: Point2D(-2, 1),
            },
            Hailstone2D {
                position: Point2D(18, 19),
                velocity: Point2D(-1, -1),
            },
            Hailstone2D {
                position: Point2D(20, 25),
                velocity: Point2D(-2, -2),
            },
            Hailstone2D {
                position: Point2D(12, 31),
                velocity: Point2D(-1, -2),
            },
            Hailstone2D {
                position: Point2D(20, 19),
                velocity: Point2D(1, -5),
            },
        ]
    }

    #[test]
    fn test_read_hailstones_2d() {
        assert_eq!(
            read_hailstones_2d(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_hailstones_2d()),
        );
    }

    #[test]
    fn test_intersection_none() {
        let first = Hailstone2D {
            position: Point2D(18, 19),
            velocity: Point2D(-1, -1),
        };
        let second = Hailstone2D {
            position: Point2D(20, 25),
            velocity: Point2D(-2, -2),
        };
        assert_eq!(first.intersection(&second), Intersection::None,);
    }

    #[test]
    fn test_intersection_past() {
        let first = Hailstone2D {
            position: Point2D(19, 13),
            velocity: Point2D(-2, 1),
        };
        let second = Hailstone2D {
            position: Point2D(20, 19),
            velocity: Point2D(1, -5),
        };
        assert_eq!(first.intersection(&second), Intersection::Past,);
    }

    #[test]
    fn test_intersection_future() {
        let first = Hailstone2D {
            position: Point2D(19, 13),
            velocity: Point2D(-2, 1),
        };
        let second = Hailstone2D {
            position: Point2D(18, 19),
            velocity: Point2D(-1, -1),
        };
        let third = Hailstone2D {
            position: Point2D(20, 25),
            velocity: Point2D(-2, -2),
        };

        assert_eq!(
            first.intersection(&second),
            Intersection::Future { x: 14, y: 15 }
        );
        assert_eq!(
            first.intersection(&third),
            Intersection::Future { x: 11, y: 16 }
        )
    }

    #[test]
    fn test_intersecting_pairs_in_area() {
        assert_eq!(
            intersecting_pairs_in_area(&example_hailstones_2d(), &TestArea { min: 7, max: 27 }),
            2
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
