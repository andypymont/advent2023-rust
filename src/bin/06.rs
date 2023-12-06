advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn times_that_beat_record(&self) -> u32 {
        let distance = self.distance + 1; // need to beat record, not just match it
        let bsquared_minus_4ac: f64 = (self.time.pow(2) - (4 * distance)).into();
        let root = bsquared_minus_4ac.sqrt();

        let time: f64 = self.time.into();

        let lower = ((time - root) / 2.0).ceil() as u32;
        let upper = ((time + root) / 2.0).floor() as u32;

        upper - lower + 1
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
                let time = time.parse::<u32>().map_err(|_| ParseRacesError)?;
                times.push(time);
            }
        } else if let Some(distances_str) = line.strip_prefix("Distance: ") {
            for distance in distances_str.split_whitespace() {
                let distance = distance.parse::<u32>().map_err(|_| ParseRacesError)?;
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

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(races) = read_races(input) {
        Some(races.iter().map(Race::times_that_beat_record).product())
    } else {
        None
    }
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
