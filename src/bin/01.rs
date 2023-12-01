advent_of_code::solution!(1);

struct Digit {
    value: usize,
    word: bool,
}

const NONWORD_DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

const WORD_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct LineDigitIterator {
    line: String,
    pos: usize,
}

impl LineDigitIterator {
    fn new(line: &str) -> Self {
        Self {
            line: line.to_string(),
            pos: 0,
        }
    }

    fn current_value(&self) -> Option<Digit> {
        if self.pos >= self.line.len() {
            return None;
        }

        let mut current_value = None;

        for (value, word) in NONWORD_DIGITS.iter().enumerate() {
            if &&self.line[self.pos..=self.pos] == word {
                current_value = Some(Digit { value, word: false });
                break;
            }
        }
        if current_value.is_none() {
            for (value, word) in WORD_DIGITS.iter().enumerate() {
                if (self.pos + word.len()) > self.line.len() {
                    continue;
                }

                if &&self.line[self.pos..self.pos + word.len()] == word {
                    current_value = Some(Digit { value, word: true });
                    break;
                }
            }
        }

        current_value
    }
}

impl Iterator for LineDigitIterator {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.line.len() {
            let digit = self.current_value();
            self.pos += 1;
            if digit.is_some() {
                return digit;
            }
        }
        None
    }
}

fn calibration_value(line: &str, include_words: bool) -> usize {
    let mut first: Option<usize> = None;
    let mut latest = 0;

    for digit in LineDigitIterator::new(line) {
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
pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| calibration_value(x, false)).sum())
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
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
