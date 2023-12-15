advent_of_code::solution!(15);

fn hasha(step: &str) -> u32 {
    step.chars()
        .fold(0, |acc, ch| (acc + (ch as u32)) * 17 % 256)
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split(',').map(|step| hasha(step.trim())).sum())
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hasha() {
        assert_eq!(hasha("rn=1"), 30);
        assert_eq!(hasha("qp-"), 14);
        assert_eq!(hasha("ab=5"), 197);
        assert_eq!(hasha("ot=7"), 231);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
