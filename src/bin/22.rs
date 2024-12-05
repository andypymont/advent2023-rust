use std::cmp::Ordering;
use std::collections::BTreeSet;

advent_of_code::solution!(22);

const GRID_SIZE: usize = 10;
const GRID_HEIGHT: usize = 310;

fn cube(x: usize, y: usize, z: usize) -> usize {
    (z * GRID_SIZE * GRID_SIZE) + (y * GRID_SIZE) + x
}

fn read_cube(text: &str) -> Option<(usize, usize, usize)> {
    let mut x: Option<usize> = None;
    let mut y: Option<usize> = None;
    let mut z: Option<usize> = None;

    for (ix, part) in text.split(',').enumerate() {
        let value = match part.parse() {
            Ok(v) => Some(v),
            Err(_) => None,
        };
        match ix {
            0 => x = value,
            1 => y = value,
            2 => z = value,
            _ => return None,
        }
    }

    let x = x?;
    let y = y?;
    let z = z?;
    Some((x, y, z))
}

fn read_brick(line: &str) -> Option<Vec<usize>> {
    let (first, last) = line.split_once('~')?;
    let (mut x, mut y, mut z) = read_cube(first)?;
    let (last_x, last_y, last_z) = read_cube(last)?;

    if (x > last_x) || (y > last_y) || (z > last_z) {
        return None;
    }

    let mut brick = Vec::new();

    while (x, y, z) != (last_x, last_y, last_z) {
        brick.push(cube(x, y, z));
        x += usize::from(x < last_x);
        y += usize::from(y < last_y);
        z += usize::from(z < last_z);
    }
    brick.push(cube(x, y, z));

    Some(brick)
}

fn read_bricks(input: &str) -> Vec<Vec<usize>> {
    let mut bricks = Vec::new();

    for line in input.lines() {
        if let Some(brick) = read_brick(line) {
            bricks.push(brick);
        }
    }

    bricks.sort_unstable_by(
        |a: &Vec<usize>, b: &Vec<usize>| match (a.first(), b.first()) {
            (None, None) => Ordering::Equal,
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (Some(a), Some(b)) => a.cmp(b),
        },
    );

    bricks
}

#[derive(Debug, PartialEq)]
struct SupportGraph {
    supporters: Vec<BTreeSet<usize>>,
    supporting: Vec<BTreeSet<usize>>,
}

impl SupportGraph {
    fn removable_bricks(&self) -> usize {
        // a brick is removable if all the bricks it supports have multiple supporters
        self.supporting
            .iter()
            .map(|supporting| {
                usize::from(
                    supporting
                        .iter()
                        .all(|supported| self.supporters[*supported].len() > 1),
                )
            })
            .sum()
    }

    fn remove_brick(&self, ix: usize) -> usize {
        let mut removed = BTreeSet::new();
        let mut queue = Vec::new();
        queue.push(ix);

        while let Some(remove) = queue.pop() {
            // record removal of the brick
            removed.insert(remove);

            // iterate bricks supported by the one removed, find those with no remaining
            // supporters, and mark them for removal
            for brick in &self.supporting[remove] {
                if self.supporters[*brick]
                    .difference(&removed)
                    .next()
                    .is_none()
                {
                    queue.push(*brick);
                }
            }
        }

        removed.len().saturating_sub(1)
    }
}

impl From<Vec<Vec<usize>>> for SupportGraph {
    fn from(bricks: Vec<Vec<usize>>) -> Self {
        let mut occupied: Vec<Option<usize>> = vec![None; GRID_HEIGHT * GRID_SIZE * GRID_SIZE];

        let mut supporters = Vec::new();
        let mut supporting = Vec::new();

        for (brick_ix, mut brick) in bricks.into_iter().enumerate() {
            let mut supported_by = BTreeSet::new();

            // Lower the brick as much as possible
            loop {
                let beneath: Vec<usize> = brick
                    .iter()
                    .filter_map(|cube| cube.checked_sub(GRID_SIZE * GRID_SIZE))
                    .collect();
                if beneath.len() < brick.len() {
                    break;
                }

                supported_by.clear();
                for pos in &beneath {
                    if let Some(other_brick) = occupied[*pos] {
                        supported_by.insert(other_brick);
                    }
                }
                if !supported_by.is_empty() {
                    break;
                }
                brick = beneath;
            }

            // Record cube positions occupied by this brick
            for cube in brick {
                occupied[cube] = Some(brick_ix);
            }

            // Record supporters and create an empty supporting entry for later population
            supporters.push(supported_by);
            supporting.push(BTreeSet::new());
        }

        // Calculate supporting graph
        for (brick_ix, others) in supporters.iter().enumerate() {
            for other in others {
                supporting[*other].insert(brick_ix);
            }
        }

        Self {
            supporters,
            supporting,
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let bricks = read_bricks(input);
    if bricks.is_empty() {
        None
    } else {
        let graph = SupportGraph::from(bricks);
        Some(graph.removable_bricks())
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let bricks = read_bricks(input);
    if bricks.is_empty() {
        None
    } else {
        let graph = SupportGraph::from(bricks);
        Some(
            (0..graph.supporters.len())
                .map(|ix| graph.remove_brick(ix))
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_bricks() -> Vec<Vec<usize>> {
        vec![
            vec![cube(1, 0, 1), cube(1, 1, 1), cube(1, 2, 1)],
            vec![cube(0, 0, 2), cube(1, 0, 2), cube(2, 0, 2)],
            vec![cube(0, 2, 3), cube(1, 2, 3), cube(2, 2, 3)],
            vec![cube(0, 0, 4), cube(0, 1, 4), cube(0, 2, 4)],
            vec![cube(2, 0, 5), cube(2, 1, 5), cube(2, 2, 5)],
            vec![cube(0, 1, 6), cube(1, 1, 6), cube(2, 1, 6)],
            vec![cube(1, 1, 8), cube(1, 1, 9)],
        ]
    }

    fn example_graph() -> SupportGraph {
        let mut supporters = Vec::new();

        let a = BTreeSet::new();
        supporters.push(a);

        let mut b = BTreeSet::new();
        b.insert(0);
        supporters.push(b);

        let mut c = BTreeSet::new();
        c.insert(0);
        supporters.push(c);

        let mut d = BTreeSet::new();
        d.insert(1);
        d.insert(2);
        supporters.push(d);

        let mut e = BTreeSet::new();
        e.insert(1);
        e.insert(2);
        supporters.push(e);

        let mut f = BTreeSet::new();
        f.insert(3);
        f.insert(4);
        supporters.push(f);

        let mut g = BTreeSet::new();
        g.insert(5);
        supporters.push(g);

        let mut supporting = Vec::new();

        let mut a = BTreeSet::new();
        a.insert(1);
        a.insert(2);
        supporting.push(a);

        let mut b = BTreeSet::new();
        b.insert(3);
        b.insert(4);
        supporting.push(b);

        let mut c = BTreeSet::new();
        c.insert(3);
        c.insert(4);
        supporting.push(c);

        let mut d = BTreeSet::new();
        d.insert(5);
        supporting.push(d);

        let mut e = BTreeSet::new();
        e.insert(5);
        supporting.push(e);

        let mut f = BTreeSet::new();
        f.insert(6);
        supporting.push(f);

        let g = BTreeSet::new();
        supporting.push(g);

        SupportGraph {
            supporters,
            supporting,
        }
    }

    #[test]
    fn test_read_bricks() {
        assert_eq!(
            read_bricks(&advent_of_code::template::read_file("examples", DAY)),
            example_bricks()
        );
    }

    #[test]
    fn test_support_graph() {
        assert_eq!(SupportGraph::from(example_bricks()), example_graph());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_remove_brick() {
        let graph = example_graph();
        assert_eq!(graph.remove_brick(0), 6);
        assert_eq!(graph.remove_brick(1), 0);
        assert_eq!(graph.remove_brick(3), 0);
        assert_eq!(graph.remove_brick(5), 1);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
