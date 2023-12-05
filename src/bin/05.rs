use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
struct AlmanacMapEntry(u64, u64, u64);

impl AlmanacMapEntry {
    fn convert(&self, source: u64) -> Option<u64> {
        if self.1 <= source && source < (self.1 + self.2) {
            Some(source - self.1 + self.0)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct AlmanacMap(Vec<AlmanacMapEntry>);

impl AlmanacMap {
    fn convert(&self, source: u64) -> u64 {
        match self
            .0
            .iter()
            .map(|entry| entry.convert(source))
            .find_map(|e| e)
        {
            Some(dest) => dest,
            None => source,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn seed_to_soil_location(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |value, map| map.convert(value))
    }
}

#[derive(Debug, PartialEq)]
struct ParseAlmanacError;

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut seeds = Vec::new();
        let mut maps = Vec::new();

        for (ix, section) in text.split("\n\n").enumerate() {
            if ix == 0 {
                if let Some(seeds_str) = section.strip_prefix("seeds: ") {
                    for seed in seeds_str.split_whitespace().map(u64::from_str) {
                        let seed = seed.map_err(|_| ParseAlmanacError)?;
                        seeds.push(seed);
                    }
                } else {
                    return Err(ParseAlmanacError);
                }
            } else {
                let map: AlmanacMap = section.parse()?;
                maps.push(map);
            }
        }

        Ok(Self { seeds, maps })
    }
}

impl FromStr for AlmanacMap {
    type Err = ParseAlmanacError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut entries = Vec::new();

        for (ix, line) in text.lines().enumerate() {
            if ix == 0 {
                continue;
            }

            let entry: AlmanacMapEntry = line.parse()?;
            entries.push(entry);
        }

        Ok(Self(entries))
    }
}

impl FromStr for AlmanacMapEntry {
    type Err = ParseAlmanacError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut dest_start: Result<u64, Self::Err> = Err(ParseAlmanacError);
        let mut source_start: Result<u64, Self::Err> = Err(ParseAlmanacError);
        let mut length: Result<u64, Self::Err> = Err(ParseAlmanacError);

        for (ix, value) in text
            .split_whitespace()
            .map(|value| u64::from_str(value).map_err(|_| ParseAlmanacError))
            .enumerate()
        {
            match ix {
                0 => dest_start = value,
                1 => source_start = value,
                2 => length = value,
                _ => return Err(ParseAlmanacError),
            }
        }

        let dest_start = dest_start?;
        let source_start = source_start?;
        let length = length?;

        Ok(AlmanacMapEntry(dest_start, source_start, length))
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    if let Ok(almanac) = input.parse::<Almanac>() {
        almanac
            .seeds
            .iter()
            .map(|seed| almanac.seed_to_soil_location(*seed))
            .min()
    } else {
        None
    }
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_almanac() -> Almanac {
        Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                AlmanacMap(vec![
                    AlmanacMapEntry(50, 98, 2),
                    AlmanacMapEntry(52, 50, 48),
                ]),
                AlmanacMap(vec![
                    AlmanacMapEntry(0, 15, 37),
                    AlmanacMapEntry(37, 52, 2),
                    AlmanacMapEntry(39, 0, 15),
                ]),
                AlmanacMap(vec![
                    AlmanacMapEntry(49, 53, 8),
                    AlmanacMapEntry(0, 11, 42),
                    AlmanacMapEntry(42, 0, 7),
                    AlmanacMapEntry(57, 7, 4),
                ]),
                AlmanacMap(vec![
                    AlmanacMapEntry(88, 18, 7),
                    AlmanacMapEntry(18, 25, 70),
                ]),
                AlmanacMap(vec![
                    AlmanacMapEntry(45, 77, 23),
                    AlmanacMapEntry(81, 45, 19),
                    AlmanacMapEntry(68, 64, 13),
                ]),
                AlmanacMap(vec![AlmanacMapEntry(0, 69, 1), AlmanacMapEntry(1, 0, 69)]),
                AlmanacMap(vec![
                    AlmanacMapEntry(60, 56, 37),
                    AlmanacMapEntry(56, 93, 4),
                ]),
            ],
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse(),
            Ok(example_almanac())
        );
    }

    #[test]
    fn test_seed_to_soil_map() {
        let map = AlmanacMap(vec![
            AlmanacMapEntry(50, 98, 2),
            AlmanacMapEntry(52, 50, 48),
        ]);
        assert_eq!(map.convert(79), 81);
        assert_eq!(map.convert(14), 14);
        assert_eq!(map.convert(55), 57);
        assert_eq!(map.convert(13), 13);
    }

    #[test]
    fn test_seed_to_soil_location() {
        let almanac = example_almanac();
        assert_eq!(almanac.seed_to_soil_location(79), 82);
        assert_eq!(almanac.seed_to_soil_location(14), 43);
        assert_eq!(almanac.seed_to_soil_location(55), 86);
        assert_eq!(almanac.seed_to_soil_location(13), 35);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
