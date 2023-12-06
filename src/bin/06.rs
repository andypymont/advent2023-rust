advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn times_that_beat_record(&self) -> u64 {
        let mut left = 0;
        let mut right = self.time / 2;

        while left < right {
            let hold = (left + right) / 2;
            let distance = (self.time - hold) * hold;
            if distance <= self.distance {
                left = hold + 1;
            } else {
                right = hold;
            }
        }

        self.time - (2 * left) + 1
    }
}

#[derive(Debug, PartialEq)]
struct ParseRacesError;

fn read_races(input: &str) -> Result<Vec<Race>, ParseRacesError> {
    let mut times = Vec::new();
    let mut distances = Vec::new();

    for line in input.lines() {
        if let Some(times_str) = line.strip_prefix("Time: ") {
            for time in times_str.split_whitespace() {
                let time = time.parse::<u64>().map_err(|_| ParseRacesError)?;
                times.push(time);
            }
        } else if let Some(distances_str) = line.strip_prefix("Distance: ") {
            for distance in distances_str.split_whitespace() {
                let distance = distance.parse::<u64>().map_err(|_| ParseRacesError)?;
                distances.push(distance);
            }
        } else {
            return Err(ParseRacesError);
        }
    }

    if times.len() != distances.len() {
        return Err(ParseRacesError);
    }

    Ok(times
        .iter()
        .enumerate()
        .map(|(ix, time)| Race {
            time: *time,
            distance: distances[ix],
        })
        .collect())
}

fn read_single_race(input: &str) -> Result<Race, ParseRacesError> {
    let mut time: Result<u64, ParseRacesError> = Err(ParseRacesError);
    let mut distance: Result<u64, ParseRacesError> = Err(ParseRacesError);

    for line in input.lines() {
        if let Some(time_str) = line.strip_prefix("Time: ") {
            let time_str = time_str.replace(' ', "");
            time = time_str.parse().map_err(|_| ParseRacesError);
        } else if let Some(distance_str) = line.strip_prefix("Distance: ") {
            let distance_str = distance_str.replace(' ', "");
            distance = distance_str.parse().map_err(|_| ParseRacesError);
        } else {
            return Err(ParseRacesError);
        }
    }

    let time = time?;
    let distance = distance?;
    Ok(Race { time, distance })
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    if let Ok(races) = read_races(input) {
        Some(races.iter().map(Race::times_that_beat_record).product())
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    if let Ok(race) = read_single_race(input) {
        Some(race.times_that_beat_record())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_races() {
        assert_eq!(
            read_races(&advent_of_code::template::read_file("examples", DAY)),
            Ok(vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                },
            ])
        );
    }

    #[test]
    fn test_times_that_beat_record() {
        let one = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(one.times_that_beat_record(), 4);

        let two = Race {
            time: 15,
            distance: 40,
        };
        assert_eq!(two.times_that_beat_record(), 8);

        let three = Race {
            time: 30,
            distance: 200,
        };
        assert_eq!(three.times_that_beat_record(), 9);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_read_single_race() {
        assert_eq!(
            read_single_race(&advent_of_code::template::read_file("examples", DAY)),
            Ok(Race {
                time: 71530,
                distance: 940200,
            })
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
