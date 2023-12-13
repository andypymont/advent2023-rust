use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(12);

struct ArrangementCalculator {
    cache: HashMap<String, u64>,
}

impl ArrangementCalculator {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn possible_arrangements_for_line(&mut self, line: &str) -> u64 {
        if let Some((springs, groups)) = line.split_once(' ') {
            self.possible_arrangements(springs, groups)
        } else {
            0
        }
    }

    fn possible_arrangements_for_unfolded_line(&mut self, line: &str) -> u64 {
        if let Some((springs, groups)) = line.split_once(' ') {
            let mut repeated_springs = String::new();
            let mut repeated_groups = String::new();

            for ix in 1..=5 {
                repeated_springs.push_str(springs);
                repeated_groups.push_str(groups);
                if ix < 5 {
                    repeated_springs.push('?');
                    repeated_groups.push(',');
                }
            }

            self.possible_arrangements(&repeated_springs, &repeated_groups)
        } else {
            0
        }
    }

    fn possible_arrangements(&mut self, springs: &str, groups: &str) -> u64 {
        self.possible_arrangements_for_section(springs, groups, 0)
    }

    fn possible_arrangements_for_section(
        &mut self,
        springs: &str,
        groups: &str,
        collected: u64,
    ) -> u64 {
        let key = format!("{springs}--{groups}--{collected}");
        if let Some(value) = self.cache.get(&key) {
            *value
        } else {
            let value =
                self.calculate_possible_arrangements_for_section(springs, groups, collected);
            self.cache.insert(key, value);
            value
        }
    }

    fn calculate_possible_arrangements_for_section(
        &mut self,
        springs: &str,
        groups: &str,
        collected: u64,
    ) -> u64 {
        let spring = springs.chars().next();

        let (group_txt, later_groups) = if let Some((group_txt, others)) = groups.split_once(',') {
            (group_txt, others)
        } else if !groups.is_empty() {
            (groups, "")
        } else {
            ("", "")
        };
        let group = group_txt.parse();

        match (spring, group) {
            (None, Err(_)) => 1,
            (Some(_), Err(_)) => u64::from(!springs.contains('#')),
            (None, Ok(_)) => 0,
            (Some(first), Ok(group)) => match first {
                '#' => match (collected + 1).cmp(&group) {
                    Ordering::Greater => 0,
                    Ordering::Equal => match springs.chars().nth(1) {
                        Some('#') => 0,
                        Some(_) => {
                            self.possible_arrangements_for_section(&springs[2..], later_groups, 0)
                        }
                        None => u64::from(later_groups.is_empty()),
                    },
                    Ordering::Less => {
                        self.possible_arrangements_for_section(&springs[1..], groups, collected + 1)
                    }
                },
                '.' => {
                    if collected > 0 {
                        0
                    } else {
                        self.possible_arrangements_for_section(&springs[1..], groups, 0)
                    }
                }
                _ => {
                    self.possible_arrangements_for_section(
                        &springs.replacen('?', ".", 1),
                        groups,
                        collected,
                    ) + self.possible_arrangements_for_section(
                        &springs.replacen('?', "#", 1),
                        groups,
                        collected,
                    )
                }
            },
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let mut calc = ArrangementCalculator::new();
    Some(
        input
            .lines()
            .map(|line| calc.possible_arrangements_for_line(line))
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let mut calc = ArrangementCalculator::new();
    Some(
        input
            .lines()
            .map(|line| calc.possible_arrangements_for_unfolded_line(line))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_arrangements_simple_match() {
        let mut calc = ArrangementCalculator::new();
        assert_eq!(calc.possible_arrangements("#.#.###", "1,1,3"), 1);
        assert_eq!(calc.possible_arrangements(".#.###.#.######", "1,3,1,6"), 1);
    }

    #[test]
    fn test_possible_arrangements_no_match() {
        let mut calc = ArrangementCalculator::new();
        assert_eq!(calc.possible_arrangements("##..###", "1,1,3"), 0);
        assert_eq!(
            calc.possible_arrangements("#....######..#####.", "1,6,6"),
            0
        );
    }

    #[test]
    fn test_possible_arrangements_with_unknown() {
        let mut calc = ArrangementCalculator::new();
        assert_eq!(calc.possible_arrangements(".??..??...?##.", "1,1,3"), 4);
        assert_eq!(calc.possible_arrangements("?###????????", "3,2,1"), 10);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_possible_arrangements_for_unfolded_line() {
        let mut calc = ArrangementCalculator::new();
        assert_eq!(
            calc.possible_arrangements_for_unfolded_line(".??..??...?##. 1,1,3"),
            16_384,
        );
        assert_eq!(
            calc.possible_arrangements_for_unfolded_line("?###???????? 3,2,1"),
            506_250,
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525_152));
    }
}
