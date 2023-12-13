use std::str::FromStr;

advent_of_code::solution!(13);

fn sequence_mirrors_after(sequence: &[usize]) -> impl Iterator<Item = usize> + '_ {
    (1..sequence.len()).filter(|ix| {
        let mut left: Vec<&usize> = sequence[..*ix].iter().collect();
        left.reverse();
        let right = &sequence[*ix..];

        let size = left.len().min(right.len());

        (0..size).all(|pos| *left[pos] == right[pos])
    })
}

#[derive(Debug, PartialEq)]
enum Mirror {
    Horizontal(usize),
    Vertical(usize),
}

impl Mirror {
    fn value(&self) -> usize {
        match self {
            Self::Horizontal(rows) => 100 * rows,
            Self::Vertical(cols) => *cols,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Pattern {
    cols: Vec<usize>,
    rows: Vec<usize>,
}

impl Pattern {
    fn find_mirrors(&self) -> impl Iterator<Item = Mirror> + '_ {
        let vert = sequence_mirrors_after(&self.cols).map(Mirror::Vertical);
        let horz = sequence_mirrors_after(&self.rows).map(Mirror::Horizontal);
        vert.chain(horz)
    }

    fn smudge(&self, r: usize, c: usize) -> Self {
        let mut cols = self.cols.clone();
        let mut rows = self.rows.clone();

        let flag_r = 1 << c;
        let flag_c = 1 << r;

        if rows[r] & flag_r == 0 {
            rows[r] |= flag_r;
            cols[c] |= flag_c;
        } else {
            rows[r] &= !flag_r;
            cols[c] &= !flag_c;
        }

        Self { cols, rows }
    }

    fn smudged_variants(&self) -> impl Iterator<Item = Self> + '_ {
        (0..self.rows.len()).flat_map(move |r| (0..self.cols.len()).map(move |c| self.smudge(r, c)))
    }
}

#[derive(Debug, PartialEq)]
struct ParsePatternError;

impl FromStr for Pattern {
    type Err = ParsePatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cols = Vec::new();
        let mut rows = Vec::new();

        for (r, line) in s.lines().enumerate() {
            let mut row = 0;

            for (c, ch) in line.chars().enumerate() {
                let value = match ch {
                    '#' => 1,
                    '.' => 0,
                    _ => return Err(ParsePatternError),
                };
                row |= value << c;
                let col = value << r;
                if cols.len() < (c + 1) {
                    cols.push(col);
                } else {
                    cols[c] |= col;
                }
            }

            rows.push(row);
        }

        Ok(Self { cols, rows })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let (errors, total) = input.split("\n\n").fold((0, 0), |(errors, total), line| {
        let Ok(pattern) = Pattern::from_str(line) else {
            return (errors + 1, total)
        };
        let Some(mirror) = pattern.find_mirrors().next() else {
            return (errors + 1, total)
        };
        (errors, total + mirror.value())
    });
    if errors > 0 {
        None
    } else {
        Some(total)
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let (errors, total) = input.split("\n\n").fold((0, 0), |(errors, total), line| {
        let Ok(pattern) = Pattern::from_str(line) else {
            return (errors + 1, total)
        };
        let Some(unsmudged) = pattern.find_mirrors().next() else {
            return (errors + 1, total)
        };
        for smudged in pattern.smudged_variants() {
            if let Some(mirror) = smudged.find_mirrors().find(|v| v != &unsmudged) {
                return (errors, total + mirror.value());
            }
        }

        (errors + 1, total)
    });
    if errors > 0 {
        None
    } else {
        Some(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_example_pattern() -> Pattern {
        Pattern {
            cols: vec![77, 12, 115, 33, 82, 82, 33, 115, 12],
            rows: vec![205, 180, 259, 259, 180, 204, 181],
        }
    }

    fn second_example_pattern() -> Pattern {
        Pattern {
            cols: vec![91, 24, 60, 60, 25, 67, 60, 60, 103],
            rows: vec![305, 289, 460, 223, 223, 460, 289],
        }
    }

    #[test]
    fn test_parse_pattern() {
        let pattern = "#.##..##.\n\
                       ..#.##.#.\n\
                       ##......#\n\
                       ##......#\n\
                       ..#.##.#.\n\
                       ..##..##.\n\
                       #.#.##.#.";
        assert_eq!(pattern.parse(), Ok(first_example_pattern()));
        let pattern = "#...##..#\n\
                       #....#..#\n\
                       ..##..###\n\
                       #####.##.\n\
                       #####.##.\n\
                       ..##..###\n\
                       #....#..#";
        assert_eq!(pattern.parse(), Ok(second_example_pattern()));
    }

    #[test]
    fn test_sequence_mirrors_after() {
        assert_eq!(
            sequence_mirrors_after(&[13, 5, 5, 13]).collect::<Vec<usize>>(),
            vec![2]
        );
        assert_eq!(
            sequence_mirrors_after(&[7, 1, 7, 7, 1, 7]).collect::<Vec<usize>>(),
            vec![3]
        );
        assert_eq!(
            sequence_mirrors_after(&[13, 14, 15, 16]).collect::<Vec<usize>>(),
            Vec::new()
        );
        assert_eq!(
            sequence_mirrors_after(&[13, 5, 5, 14]).collect::<Vec<usize>>(),
            Vec::new()
        );
    }

    #[test]
    fn test_pattern_find_mirrors() {
        let pattern = first_example_pattern();
        let mirrors: Vec<Mirror> = pattern.find_mirrors().collect();
        assert_eq!(mirrors, vec![Mirror::Vertical(5)]);

        let pattern = second_example_pattern();
        let mirrors: Vec<Mirror> = pattern.find_mirrors().collect();
        assert_eq!(mirrors, vec![Mirror::Horizontal(4)]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_smudged_variants() {
        let pattern = first_example_pattern();
        let variants: Vec<Pattern> = pattern.smudged_variants().collect();
        assert_eq!(variants.len(), 63);
        assert!(variants.iter().all(|v| v != &pattern));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
