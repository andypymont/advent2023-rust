use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

advent_of_code::solution!(22);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Debug, PartialEq)]
struct ParseBrickError;

impl FromStr for Cube {
    type Err = ParseBrickError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut x: Result<u32, Self::Err> = Err(ParseBrickError);
        let mut y: Result<u32, Self::Err> = Err(ParseBrickError);
        let mut z: Result<u32, Self::Err> = Err(ParseBrickError);

        for (pos, part) in text.split(',').enumerate() {
            let value = part.parse().map_err(|_| ParseBrickError);
            match pos {
                0 => x = value,
                1 => y = value,
                2 => z = value,
                _ => return Err(ParseBrickError),
            }
        }

        let x = x?;
        let y = y?;
        let z = z?;
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, PartialEq)]
struct Brick {
    cubes: Vec<Cube>,
    min_z: u32,
}

impl Brick {
    fn fall(&self) -> Self {
        Self {
            cubes: self
                .cubes
                .iter()
                .map(|cube| Cube {
                    x: cube.x,
                    y: cube.y,
                    z: cube.z - 1,
                })
                .collect(),
            min_z: self.min_z.saturating_sub(1),
        }
    }
}

impl FromStr for Brick {
    type Err = ParseBrickError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some((first, last)) = line.split_once('~') else {
            return Err(ParseBrickError);
        };
        let mut cube: Cube = first.parse()?;
        let last: Cube = last.parse()?;

        let mut cubes = Vec::new();
        let mut min_z = cube.z;
        while cube != last {
            min_z = min_z.min(cube.z);
            cubes.push(cube);
            let x = cube.x + u32::from(cube.x < last.x);
            let y = cube.y + u32::from(cube.y < last.y);
            let z = cube.z + u32::from(cube.z < last.z);
            cube = Cube { x, y, z };
        }
        cubes.push(cube);

        Ok(Self { cubes, min_z })
    }
}

#[derive(Debug, PartialEq)]
struct System {
    bricks: Vec<Brick>,
    occupied: BTreeMap<Cube, usize>,
}

impl System {
    fn fall(&mut self) -> usize {
        let mut drop_count = 0;

        for ix in 0..self.bricks.len() {
            let brick = &self.bricks[ix];
            if brick.min_z > 1 {
                let dropped = brick.fall();
                if dropped
                    .cubes
                    .iter()
                    .all(|cube| match self.occupied.get(cube) {
                        None => true,
                        Some(supporter) => supporter == &ix,
                    })
                {
                    drop_count += 1;
                    for cube in &brick.cubes {
                        self.occupied.remove(cube);
                    }
                    for cube in &dropped.cubes {
                        self.occupied.insert(*cube, ix);
                    }
                    self.bricks[ix] = dropped;
                }
            }
        }

        drop_count
    }

    fn settle(&mut self) {
        while self.fall() > 0 {
            // keep looping whilst bricks keep dropping
        }
    }

    fn unremovable_brick_indices(&self) -> BTreeSet<usize> {
        let mut unremovable = BTreeSet::new();

        for (ix, brick) in self.bricks.iter().enumerate() {
            let dropped = brick.fall();
            let mut supporters = BTreeSet::new();
            for cube in &dropped.cubes {
                if let Some(supporter) = self.occupied.get(cube) {
                    if supporter != &ix {
                        supporters.insert(supporter);
                    }
                }
            }
            if supporters.len() == 1 {
                // exactly one brick supports this one
                if let Some(supporter) = supporters.iter().next() {
                    unremovable.insert(**supporter);
                }
            }
        }

        unremovable
    }
}

impl FromStr for System {
    type Err = ParseBrickError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut bricks = Vec::new();
        let mut occupied = BTreeMap::new();
        for (ix, line) in text.lines().enumerate() {
            let brick: Brick = line.parse()?;
            for cube in &brick.cubes {
                occupied.insert(*cube, ix);
            }
            bricks.push(brick);
        }
        Ok(Self { bricks, occupied })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(mut system) = input.parse::<System>() {
        system.settle();
        Some(
            system
                .bricks
                .len()
                .saturating_sub(system.unremovable_brick_indices().len()),
        )
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

    fn example_system() -> System {
        let bricks = vec![
            Brick {
                cubes: vec![
                    Cube { x: 1, y: 0, z: 1 },
                    Cube { x: 1, y: 1, z: 1 },
                    Cube { x: 1, y: 2, z: 1 },
                ],
                min_z: 1,
            },
            Brick {
                cubes: vec![
                    Cube { x: 0, y: 0, z: 2 },
                    Cube { x: 1, y: 0, z: 2 },
                    Cube { x: 2, y: 0, z: 2 },
                ],
                min_z: 2,
            },
            Brick {
                cubes: vec![
                    Cube { x: 0, y: 2, z: 3 },
                    Cube { x: 1, y: 2, z: 3 },
                    Cube { x: 2, y: 2, z: 3 },
                ],
                min_z: 3,
            },
            Brick {
                cubes: vec![
                    Cube { x: 0, y: 0, z: 4 },
                    Cube { x: 0, y: 1, z: 4 },
                    Cube { x: 0, y: 2, z: 4 },
                ],
                min_z: 4,
            },
            Brick {
                cubes: vec![
                    Cube { x: 2, y: 0, z: 5 },
                    Cube { x: 2, y: 1, z: 5 },
                    Cube { x: 2, y: 2, z: 5 },
                ],
                min_z: 5,
            },
            Brick {
                cubes: vec![
                    Cube { x: 0, y: 1, z: 6 },
                    Cube { x: 1, y: 1, z: 6 },
                    Cube { x: 2, y: 1, z: 6 },
                ],
                min_z: 6,
            },
            Brick {
                cubes: vec![Cube { x: 1, y: 1, z: 8 }, Cube { x: 1, y: 1, z: 9 }],
                min_z: 8,
            },
        ];
        let mut occupied = BTreeMap::new();
        for (ix, brick) in bricks.iter().enumerate() {
            for cube in &brick.cubes {
                occupied.insert(*cube, ix);
            }
        }

        System { bricks, occupied }
    }

    fn example_system_settled() -> System {
        let bricks = vec![
            Brick {
                cubes: vec![
                    Cube { x: 1, y: 0, z: 1 },
                    Cube { x: 1, y: 1, z: 1 },
                    Cube { x: 1, y: 2, z: 1 },
                ],
                min_z: 1,
            },
            Brick {
                cubes: vec![
                    Cube { x: 0, y: 0, z: 2 },
                    Cube { x: 1, y: 0, z: 2 },
                    Cube { x: 2, y: 0, z: 2 },
                ],
                min_z: 2,
            },
            Brick {
                cubes: vec![
                    Cube { x: 0, y: 2, z: 2 },
                    Cube { x: 1, y: 2, z: 2 },
                    Cube { x: 2, y: 2, z: 2 },
                ],
                min_z: 2,
            },
            Brick {
                cubes: vec![
                    Cube { x: 0, y: 0, z: 3 },
                    Cube { x: 0, y: 1, z: 3 },
                    Cube { x: 0, y: 2, z: 3 },
                ],
                min_z: 3,
            },
            Brick {
                cubes: vec![
                    Cube { x: 2, y: 0, z: 3 },
                    Cube { x: 2, y: 1, z: 3 },
                    Cube { x: 2, y: 2, z: 3 },
                ],
                min_z: 3,
            },
            Brick {
                cubes: vec![
                    Cube { x: 0, y: 1, z: 4 },
                    Cube { x: 1, y: 1, z: 4 },
                    Cube { x: 2, y: 1, z: 4 },
                ],
                min_z: 4,
            },
            Brick {
                cubes: vec![Cube { x: 1, y: 1, z: 5 }, Cube { x: 1, y: 1, z: 6 }],
                min_z: 5,
            },
        ];
        let mut occupied = BTreeMap::new();
        for (ix, brick) in bricks.iter().enumerate() {
            for cube in &brick.cubes {
                occupied.insert(*cube, ix);
            }
        }

        System { bricks, occupied }
    }

    #[test]
    fn test_parse_system() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse(),
            Ok(example_system())
        );
    }

    #[test]
    fn test_settle_system() {
        let mut system = example_system();
        system.settle();
        assert_eq!(system, example_system_settled());
    }

    #[test]
    fn test_unremovable_brick_indices() {
        let system = example_system_settled();
        let mut expected = BTreeSet::new();
        expected.insert(0);
        expected.insert(5);
        assert_eq!(system.unremovable_brick_indices(), expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
