use std::cmp::Ordering;
use std::str::FromStr;

advent_of_code::solution!(12);

#[derive(Debug, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq)]
struct ConditionRecord {
    springs: Vec<Spring>,
    groups: Vec<u32>,
}

impl ConditionRecord {
    fn possible_arrangements_for_section(
        springs: &[Spring],
        groups: &[u32],
        collected: u32,
        sub: &Option<Spring>,
    ) -> u32 {
        let spring = if sub.is_some() {
            sub.as_ref()
        } else {
            springs.first()
        };
        let group = groups.first();

        match (spring, group) {
            (None, None) => 1,
            (Some(_), None) => u32::from(!springs.contains(&Spring::Damaged)),
            (None, Some(_)) => 0,
            (Some(first), Some(group)) => match first {
                Spring::Damaged => match (collected + 1).cmp(group) {
                    Ordering::Greater => 0,
                    Ordering::Equal => match springs.get(1) {
                        Some(Spring::Damaged) => 0,
                        Some(_) => Self::possible_arrangements_for_section(
                            &springs[2..],
                            &groups[1..],
                            0,
                            &None,
                        ),
                        None => u32::from(groups.len() == 1),
                    },
                    Ordering::Less => Self::possible_arrangements_for_section(
                        &springs[1..],
                        groups,
                        collected + 1,
                        &None,
                    ),
                },
                Spring::Operational => {
                    if collected > 0 {
                        0
                    } else {
                        Self::possible_arrangements_for_section(&springs[1..], groups, 0, &None)
                    }
                }
                Spring::Unknown => {
                    Self::possible_arrangements_for_section(
                        springs,
                        groups,
                        collected,
                        &Some(Spring::Operational),
                    ) + Self::possible_arrangements_for_section(
                        springs,
                        groups,
                        collected,
                        &Some(Spring::Damaged),
                    )
                }
            },
        }
    }

    fn possible_arrangements(&self) -> u32 {
        Self::possible_arrangements_for_section(&self.springs, &self.groups, 0, &None)
    }
}

#[derive(Debug, PartialEq)]
struct ParseConditionRecordError;

impl TryFrom<char> for Spring {
    type Error = ParseConditionRecordError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err(ParseConditionRecordError),
        }
    }
}

impl FromStr for ConditionRecord {
    type Err = ParseConditionRecordError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some((springs_txt, groups_txt)) = line.split_once(' ') {
            let mut springs = Vec::new();
            for ch in springs_txt.chars() {
                let spring = ch.try_into()?;
                springs.push(spring);
            }

            let mut groups = Vec::new();
            for group_txt in groups_txt.split(',') {
                let group = group_txt.parse().map_err(|_| ParseConditionRecordError)?;
                groups.push(group);
            }

            Ok(Self { springs, groups })
        } else {
            Err(ParseConditionRecordError)
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| match ConditionRecord::from_str(line) {
                Ok(record) => record.possible_arrangements(),
                Err(_) => 0,
            })
            .sum(),
    )
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_condition_record() {
        assert_eq!(
            ".??..??...?##. 1,1,3".parse(),
            Ok(ConditionRecord {
                springs: vec![
                    Spring::Operational,
                    Spring::Unknown,
                    Spring::Unknown,
                    Spring::Operational,
                    Spring::Operational,
                    Spring::Unknown,
                    Spring::Unknown,
                    Spring::Operational,
                    Spring::Operational,
                    Spring::Operational,
                    Spring::Unknown,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Operational,
                ],
                groups: vec![1, 1, 3],
            })
        );
        assert_eq!(
            "????.######..#####. 1,6,5".parse(),
            Ok(ConditionRecord {
                springs: vec![
                    Spring::Unknown,
                    Spring::Unknown,
                    Spring::Unknown,
                    Spring::Unknown,
                    Spring::Operational,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Operational,
                    Spring::Operational,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Operational,
                ],
                groups: vec![1, 6, 5],
            })
        );
    }

    #[test]
    fn test_possible_arrangements_simple_match() {
        let record = ConditionRecord {
            springs: vec![
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
            ],
            groups: vec![1, 1, 3],
        };
        assert_eq!(record.possible_arrangements(), 1);

        let record = ConditionRecord {
            springs: vec![
                Spring::Operational,
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
            ],
            groups: vec![1, 3, 1, 6],
        };
        assert_eq!(record.possible_arrangements(), 1);
    }

    #[test]
    fn test_possible_arrangements_no_match() {
        let record = ConditionRecord {
            springs: vec![
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
            ],
            groups: vec![1, 1, 3],
        };
        assert_eq!(record.possible_arrangements(), 0);

        let record = ConditionRecord {
            springs: vec![
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
            ],
            groups: vec![],
        };
        assert_eq!(record.possible_arrangements(), 0);
    }

    #[test]
    fn test_possible_arrangements_with_unknowns() {
        let record = ConditionRecord {
            springs: vec![
                Spring::Operational,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Operational,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Unknown,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
            ],
            groups: vec![1, 1, 3],
        };
        assert_eq!(record.possible_arrangements(), 4);

        let record = ConditionRecord {
            springs: vec![
                Spring::Unknown,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
            ],
            groups: vec![3, 2, 1],
        };
        assert_eq!(record.possible_arrangements(), 10);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
