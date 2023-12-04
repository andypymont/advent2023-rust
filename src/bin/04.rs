use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq)]
struct Scratchcard {
    card_no: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Scratchcard {
    fn score(&self) -> u32 {
        self.numbers.iter().fold(0, |score, number| {
            if !self.winners.contains(number) {
                return score;
            }

            if score == 0 {
                1
            } else {
                score << 1
            }
        })
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
        if let Some((card_desc, all_numbers)) = line.split_once(": ") {
            let card_no = if let Some(card_no_str) = card_desc.strip_prefix("Card ") {
                card_no_str
                    .trim()
                    .parse()
                    .map_err(|_| ParseScratchcardError)
            } else {
                Err(ParseScratchcardError)
            }?;

            if let Some((winners_str, numbers_str)) = all_numbers.split_once(" | ") {
                let winners = parse_number_list(winners_str)?;
                let numbers = parse_number_list(numbers_str)?;
                Ok(Scratchcard {
                    card_no,
                    winners,
                    numbers,
                })
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
    Some(input.lines().fold(0, |total, line| {
        if let Ok(scratchcard) = line.parse::<Scratchcard>() {
            total + scratchcard.score()
        } else {
            total
        }
    }))
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scratchcard() {
        assert_eq!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse(),
            Ok(Scratchcard {
                card_no: 1,
                winners: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            })
        );
    }

    #[test]
    fn test_parse_longer_scratchcard_with_more_whitespace() {
        assert_eq!(
            "Card  96: 73 42 99 21 12 29 77 94  1 26 | 46  5 63 64 83 75 74 86 69 89 79 57 60 48 37 13 96 43 72  4 23 98 59 80 92".parse(),
            Ok(Scratchcard {
                card_no: 96,
                winners: vec![73, 42, 99, 21, 12, 29, 77, 94, 1, 26],
                numbers: vec![46, 5, 63, 64, 83, 75, 74, 86, 69, 89, 79, 57, 60, 48, 37, 13, 96, 43, 72, 4, 23, 98, 59, 80, 92],
            })
        );
    }

    #[test]
    fn test_score_scratchcard() {
        let one = Scratchcard {
            card_no: 1,
            winners: vec![41, 48, 83, 86, 17],
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(one.score(), 8);

        let three = Scratchcard {
            card_no: 3,
            winners: vec![1, 21, 53, 59, 44],
            numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
        };
        assert_eq!(three.score(), 2);

        let five = Scratchcard {
            card_no: 6,
            winners: vec![31, 18, 13, 56, 72],
            numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
        };
        assert_eq!(five.score(), 0);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
