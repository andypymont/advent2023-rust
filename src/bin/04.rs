use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq)]
struct Scratchcard {
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Scratchcard {
    fn matches(&self) -> usize {
        self.numbers
            .iter()
            .filter(|x| self.winners.contains(x))
            .count()
    }

    fn score(&self) -> u32 {
        match self.matches() {
            0 => 0,
            x => 1 << (x - 1),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseScratchcardError;

fn parse_number_list(text: &str) -> Result<Vec<u32>, ParseScratchcardError> {
    let mut list = Vec::new();

    for number_str in text.split_whitespace() {
        let number = number_str.parse().map_err(|_| ParseScratchcardError)?;
        list.push(number);
    }

    Ok(list)
}

impl FromStr for Scratchcard {
    type Err = ParseScratchcardError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some((_card_desc, all_numbers)) = line.split_once(": ") {
            if let Some((winners_str, numbers_str)) = all_numbers.split_once(" | ") {
                let winners = parse_number_list(winners_str)?;
                let numbers = parse_number_list(numbers_str)?;
                Ok(Self { winners, numbers })
            } else {
                Err(ParseScratchcardError)
            }
        } else {
            Err(ParseScratchcardError)
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let mut total_score = 0;
    for scratchcard in input.lines().map(Scratchcard::from_str) {
        match scratchcard {
            Ok(scratchcard) => total_score += scratchcard.score(),
            Err(_) => return None,
        }
    }
    Some(total_score)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let mut copies = [1; 190];
    let mut total = 0;
    for (ix, scratchcard) in input.lines().map(Scratchcard::from_str).enumerate() {
        match scratchcard {
            Ok(scratchcard) => {
                total += copies[ix];
                for card in (ix + 1)..(ix + 1 + scratchcard.matches()) {
                    copies[card] += copies[ix];
                }
            }
            Err(_) => return None,
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scratchcard() {
        assert_eq!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse(),
            Ok(Scratchcard {
                winners: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            })
        );
    }

    #[test]
    fn test_scratchcard_matches() {
        let one = Scratchcard {
            winners: vec![41, 48, 83, 86, 17],
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(one.matches(), 4);

        let two = Scratchcard {
            winners: vec![13, 32, 20, 16, 61],
            numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
        };
        assert_eq!(two.matches(), 2);

        let four = Scratchcard {
            winners: vec![41, 92, 73, 84, 69],
            numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
        };
        assert_eq!(four.matches(), 1);

        let five = Scratchcard {
            winners: vec![87, 83, 26, 28, 32],
            numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
        };
        assert_eq!(five.matches(), 0);
    }

    #[test]
    fn test_score_scratchcard() {
        let one = Scratchcard {
            winners: vec![41, 48, 83, 86, 17],
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(one.score(), 8);

        let three = Scratchcard {
            winners: vec![1, 21, 53, 59, 44],
            numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
        };
        assert_eq!(three.score(), 2);

        let six = Scratchcard {
            winners: vec![31, 18, 13, 56, 72],
            numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
        };
        assert_eq!(six.score(), 0);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
