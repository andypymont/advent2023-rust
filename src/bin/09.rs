use std::str::FromStr;

advent_of_code::solution!(9);

#[derive(Debug, PartialEq)]
struct Sequence(Vec<i32>);

impl Sequence {
    const fn empty() -> Self {
        Self(Vec::new())
    }

    fn differences(&self) -> Self {
        Self(
            (1..self.0.len())
                .map(|ix| self.0[ix] - self.0[ix - 1])
                .collect(),
        )
    }

    fn next_item(&self) -> i32 {
        let last = self.0.last().unwrap_or(&0);
        if self.0.iter().all(|item| item == last) {
            *last
        } else {
            last + self.differences().next_item()
        }
    }

    fn prev_item(&self) -> i32 {
        let first = self.0.first().unwrap_or(&0);
        if self.0.iter().all(|item| item == first) {
            *first
        } else {
            first - self.differences().prev_item()
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseSequenceError;

impl FromStr for Sequence {
    type Err = ParseSequenceError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        for num_str in line.split_whitespace() {
            let num: i32 = num_str.parse().map_err(|_| ParseSequenceError)?;
            numbers.push(num);
        }
        Ok(Self(numbers))
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                Sequence::from_str(line)
                    .unwrap_or_else(|_| Sequence::empty())
                    .next_item()
            })
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                Sequence::from_str(line)
                    .unwrap_or_else(|_| Sequence::empty())
                    .prev_item()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sequence() {
        assert_eq!(
            "0 3 6 9 12 15".parse::<Sequence>(),
            Ok(Sequence(vec![0, 3, 6, 9, 12, 15])),
        );
        assert_eq!(
            "-4 -2 0 2 4 6".parse::<Sequence>(),
            Ok(Sequence(vec![-4, -2, 0, 2, 4, 6])),
        );
        assert_eq!(
            "14 15 16 17 NaN undefined".parse::<Sequence>(),
            Err(ParseSequenceError),
        );
    }

    #[test]
    fn test_diffences() {
        let sequence = Sequence(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(sequence.differences(), Sequence(vec![3, 3, 3, 3, 3]),);

        let sequence = Sequence(vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(sequence.differences(), Sequence(vec![2, 3, 4, 5, 6]),);
    }

    #[test]
    fn test_next_item() {
        let sequence = Sequence(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(sequence.next_item(), 18);

        let sequence = Sequence(vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(sequence.next_item(), 28);

        let sequence = Sequence(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(sequence.next_item(), 68);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_prev_item() {
        let sequence = Sequence(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(sequence.prev_item(), -3);

        let sequence = Sequence(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(sequence.prev_item(), 5);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
