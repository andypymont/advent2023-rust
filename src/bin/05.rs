use std::collections::BTreeSet;
use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
struct AlmanacMapEntry {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl AlmanacMapEntry {
    fn convert(&self, source: u64) -> Option<u64> {
        if self.source_start <= source && source < (self.source_start + self.length) {
            Some(source - self.source_start + self.dest_start)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct AlmanacMap(Vec<AlmanacMapEntry>);

#[derive(Debug, PartialEq)]
struct ValueRange {
    start: u64,
    length: u64,
}

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

    fn convert_range(&self, range: &ValueRange) -> Vec<ValueRange> {
        let mut slices = BTreeSet::new();
        let range_end = range.start + range.length;

        for entry in &self.0 {
            let source_end = entry.source_start + entry.length;

            if range_end < entry.source_start || range.start > source_end {
                continue;
            }

            if entry.source_start > range.start {
                slices.insert(entry.source_start);
            }

            if source_end < range_end {
                slices.insert(source_end);
            }
        }
        slices.insert(range_end);

        let mut output = Vec::new();
        let mut current = range.start;

        for position in slices {
            output.push(ValueRange {
                start: self.convert(current),
                length: position - current,
            });
            current = position;
        }

        output
    }
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |value, map| map.convert(value))
    }

    fn seed_ranges(&self) -> impl Iterator<Item = ValueRange> + '_ {
        (0..self.seeds.len()).step_by(2).map(|ix| ValueRange {
            start: self.seeds[ix],
            length: self.seeds[ix + 1],
        })
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

        Ok(AlmanacMapEntry {
            dest_start,
            source_start,
            length,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    if let Ok(almanac) = input.parse::<Almanac>() {
        almanac
            .seeds
            .iter()
            .map(|seed| almanac.seed_to_location(*seed))
            .min()
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    if let Ok(almanac) = input.parse::<Almanac>() {
        let mut current: Vec<ValueRange> = almanac.seed_ranges().collect();
        let mut future = Vec::new();

        for map in almanac.maps {
            for range in current {
                future.extend(map.convert_range(&range));
            }
            current = future;
            future = Vec::new();
        }

        current.iter().map(|range| range.start).min()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_almanac() -> Almanac {
        Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                AlmanacMap(vec![
                    AlmanacMapEntry {
                        dest_start: 50,
                        source_start: 98,
                        length: 2,
                    },
                    AlmanacMapEntry {
                        dest_start: 52,
                        source_start: 50,
                        length: 48,
                    },
                ]),
                AlmanacMap(vec![
                    AlmanacMapEntry {
                        dest_start: 0,
                        source_start: 15,
                        length: 37,
                    },
                    AlmanacMapEntry {
                        dest_start: 37,
                        source_start: 52,
                        length: 2,
                    },
                    AlmanacMapEntry {
                        dest_start: 39,
                        source_start: 0,
                        length: 15,
                    },
                ]),
                AlmanacMap(vec![
                    AlmanacMapEntry {
                        dest_start: 49,
                        source_start: 53,
                        length: 8,
                    },
                    AlmanacMapEntry {
                        dest_start: 0,
                        source_start: 11,
                        length: 42,
                    },
                    AlmanacMapEntry {
                        dest_start: 42,
                        source_start: 0,
                        length: 7,
                    },
                    AlmanacMapEntry {
                        dest_start: 57,
                        source_start: 7,
                        length: 4,
                    },
                ]),
                AlmanacMap(vec![
                    AlmanacMapEntry {
                        dest_start: 88,
                        source_start: 18,
                        length: 7,
                    },
                    AlmanacMapEntry {
                        dest_start: 18,
                        source_start: 25,
                        length: 70,
                    },
                ]),
                AlmanacMap(vec![
                    AlmanacMapEntry {
                        dest_start: 45,
                        source_start: 77,
                        length: 23,
                    },
                    AlmanacMapEntry {
                        dest_start: 81,
                        source_start: 45,
                        length: 19,
                    },
                    AlmanacMapEntry {
                        dest_start: 68,
                        source_start: 64,
                        length: 13,
                    },
                ]),
                AlmanacMap(vec![
                    AlmanacMapEntry {
                        dest_start: 0,
                        source_start: 69,
                        length: 1,
                    },
                    AlmanacMapEntry {
                        dest_start: 1,
                        source_start: 0,
                        length: 69,
                    },
                ]),
                AlmanacMap(vec![
                    AlmanacMapEntry {
                        dest_start: 60,
                        source_start: 56,
                        length: 37,
                    },
                    AlmanacMapEntry {
                        dest_start: 56,
                        source_start: 93,
                        length: 4,
                    },
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
            AlmanacMapEntry {
                dest_start: 50,
                source_start: 98,
                length: 2,
            },
            AlmanacMapEntry {
                dest_start: 52,
                source_start: 50,
                length: 48,
            },
        ]);
        assert_eq!(map.convert(79), 81);
        assert_eq!(map.convert(14), 14);
        assert_eq!(map.convert(55), 57);
        assert_eq!(map.convert(13), 13);
    }

    #[test]
    fn test_seed_to_location() {
        let almanac = example_almanac();
        assert_eq!(almanac.seed_to_location(79), 82);
        assert_eq!(almanac.seed_to_location(14), 43);
        assert_eq!(almanac.seed_to_location(55), 86);
        assert_eq!(almanac.seed_to_location(13), 35);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_seed_to_soil_map_range() {
        let map = AlmanacMap(vec![
            AlmanacMapEntry {
                dest_start: 50,
                source_start: 98,
                length: 2,
            },
            AlmanacMapEntry {
                dest_start: 52,
                source_start: 50,
                length: 48,
            },
        ]);
        assert_eq!(
            map.convert_range(&ValueRange {
                start: 50,
                length: 5
            }),
            vec![ValueRange {
                start: 52,
                length: 5
            }]
        );
    }

    #[test]
    fn test_seed_ranges() {
        let seed_ranges: Vec<ValueRange> = example_almanac().seed_ranges().collect();
        assert_eq!(
            seed_ranges,
            vec![
                ValueRange {
                    start: 79,
                    length: 14
                },
                ValueRange {
                    start: 55,
                    length: 13
                },
            ]
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
