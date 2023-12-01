advent_of_code::solution!(1);

struct Digit {
    value: u32,
    word: bool,
}

const WORD_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_digit(line: &str, pos: usize, ch: char) -> Option<Digit> {
    if let Some(digit) = ch.to_digit(10) {
        Some(Digit {
            value: digit,
            word: false,
        })
    } else {
        let mut word_value: Option<u32> = None;
        for (value, word) in WORD_DIGITS.iter().enumerate() {
            if (pos + word.len()) > line.len() {
                continue;
            }

            if &&line[pos..pos + word.len()] == word {
                word_value = Some(value.try_into().unwrap_or(0));
                break;
            }
        }
        word_value.map(|value| Digit { value, word: true })
    }
}

fn digits_in_line(line: &str) -> Vec<Digit> {
    let mut digits = Vec::new();

    for (pos, ch) in line.chars().enumerate() {
        if let Some(digit) = parse_digit(line, pos, ch) {
            digits.push(digit);
        }
    }

    digits
}

fn calibration_value(line: &str, include_words: bool) -> u32 {
    let mut first: Option<u32> = None;
    let mut latest = 0;

    for digit in digits_in_line(line) {
        if include_words || !digit.word {
            latest = digit.value;
            if first.is_none() {
                first = Some(latest);
            }
        }
    }

    match first {
        None => 0,
        Some(first) => (first * 10) + latest,
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|x| calibration_value(x, false)).sum())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|x| calibration_value(x, true)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_value() {
        assert_eq!(calibration_value("1abc2", false), 12);
        assert_eq!(calibration_value("pqr3stu8vwx", false), 38);
        assert_eq!(calibration_value("a1b2c3d4e5f", false), 15);
        assert_eq!(calibration_value("treb7uchet", false), 77);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_true_calibration_value() {
        assert_eq!(calibration_value("two1nine", true), 29);
        assert_eq!(calibration_value("eighttwothree", true), 83);
        assert_eq!(calibration_value("4nineeightseven2", true), 42);
        assert_eq!(calibration_value("zoneight234", true), 14);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
