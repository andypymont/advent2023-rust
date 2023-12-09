use std::collections::HashMap;
use std::convert::TryFrom;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq)]
enum MapNode<'a> {
    Fork(&'a str, &'a str),
    DeadEnd,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
struct Congruence {
    value: usize,
    modulo: usize,
}

impl Congruence {
    fn multiplicative_inverse(&self) -> Option<usize> {
        let value = self.value.try_into().unwrap_or(1);
        let modulo = self.modulo.try_into().unwrap_or(1);

        let (gcd, x, _) = Self::euclid_gcd(value, modulo);
        if gcd != 1 {
            None
        } else if let Ok(inv) = ((x % modulo + modulo) % modulo).try_into() {
            Some(inv)
        } else {
            None
        }
    }

    fn euclid_gcd(modulo: isize, value: isize) -> (isize, isize, isize) {
        if modulo == 0 {
            (value, 0, 1)
        } else {
            let (gcd, x, y) = Self::euclid_gcd(value % modulo, modulo);
            (gcd, y - (value / modulo) * x, x)
        }
    }
}

#[derive(Debug, PartialEq)]
struct Network<'a> {
    directions: Vec<Direction>,
    map: HashMap<&'a str, MapNode<'a>>,
}

impl Network<'_> {
    fn find_a_to_z_loop<'a>(&'a self, start: &'a str) -> Option<Congruence> {
        let mut position = start;
        let mut steps = 0;
        let mut z: Option<&str> = None;
        let mut initial = 0;
        let mut cycle: Option<usize> = None;

        while cycle.is_none() {
            position = match self.map.get(position).unwrap_or(&MapNode::DeadEnd) {
                MapNode::Fork(left, right) => {
                    match self.directions.get(steps % self.directions.len()) {
                        Some(Direction::Left) => left,
                        Some(Direction::Right) => right,
                        None => return None,
                    }
                }
                MapNode::DeadEnd => return None,
            };
            if position.ends_with('Z') {
                if let Some(prev_z) = z {
                    if position != prev_z {
                        return None;
                    };
                    cycle = Some(steps - initial);
                } else {
                    z = Some(position);
                    initial = steps;
                }
            }
            steps += 1;
        }

        cycle.map(|cycle| Congruence {
            value: (initial + 1) % cycle,
            modulo: cycle,
        })
    }

    fn find_a_to_z_loops(&self) -> Vec<Congruence> {
        self.map
            .keys()
            .filter_map(|key| {
                if key.ends_with('A') {
                    self.find_a_to_z_loop(key)
                } else {
                    None
                }
            })
            .collect()
    }

    fn steps_to_reach(&self, start: &str, target: &str) -> Option<usize> {
        let mut position = start;
        let mut steps = 0;

        while position != target {
            position = match self.map.get(position).unwrap_or(&MapNode::DeadEnd) {
                MapNode::Fork(left, right) => {
                    match self.directions.get(steps % self.directions.len()) {
                        Some(Direction::Left) => left,
                        Some(Direction::Right) => right,
                        None => return None,
                    }
                }
                MapNode::DeadEnd => return None,
            };
            steps += 1;
        }

        Some(steps)
    }
}

#[derive(Debug, PartialEq)]
struct ParseNetworkError;

impl TryFrom<char> for Direction {
    type Error = ParseNetworkError;

    fn try_from(ch: char) -> Result<Direction, Self::Error> {
        match ch {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(ParseNetworkError),
        }
    }
}

impl<'a> TryFrom<&'a str> for Network<'a> {
    type Error = ParseNetworkError;

    fn try_from(input: &str) -> Result<Network, Self::Error> {
        if let Some((directions_text, map_text)) = input.split_once("\n\n") {
            let mut directions = Vec::new();
            for ch in directions_text.chars() {
                let direction = Direction::try_from(ch)?;
                directions.push(direction);
            }

            let mut map = HashMap::new();

            for line in map_text.lines() {
                let Some((key, node_desc)) = line.split_once(" = ") else {
                    return Err(ParseNetworkError);
                };

                let Some((left, right)) = node_desc.split_once(", ") else {
                    return Err(ParseNetworkError);
                };

                let Some(left) = left.strip_prefix('(') else {
                    return Err(ParseNetworkError);
                };
                let Some(right) = right.strip_suffix(')') else {
                    return Err(ParseNetworkError);
                };

                let node = if key == left && left == right {
                    MapNode::DeadEnd
                } else {
                    MapNode::Fork(left, right)
                };

                map.insert(key, node);
            }

            Ok(Network { directions, map })
        } else {
            Err(ParseNetworkError)
        }
    }
}

fn build_prime_sieve(max: usize) -> Vec<bool> {
    let mut sieve = vec![true; max];

    for ix in 2..max {
        if sieve[ix] {
            for non_prime in ((ix * ix)..max).step_by(ix) {
                sieve[non_prime] = false;
            }
        }
    }

    sieve
}

fn prime_factorise(congruence: &Congruence, sieve: &[bool]) -> Vec<(usize, Congruence)> {
    let mut factors = Vec::new();
    let mut remaining = congruence.modulo;

    for prime in
        sieve
            .iter()
            .enumerate()
            .filter_map(|(ix, is_prime)| if *is_prime && ix >= 2 { Some(ix) } else { None })
    {
        let mut current = 1;

        while remaining % prime == 0 {
            remaining /= prime;
            current *= prime;
        }

        if current > 1 {
            factors.push((
                prime,
                Congruence {
                    value: congruence.value,
                    modulo: current,
                },
            ));
        }

        if remaining == 1 {
            break;
        }
    }

    if remaining > 1 {
        factors.push((
            remaining,
            Congruence {
                value: congruence.value,
                modulo: remaining,
            },
        ));
    }

    factors
}

fn convert_congruences_to_coprime(congruences: &[Congruence]) -> Vec<Congruence> {
    let max_modulo = congruences.iter().map(|c| c.modulo).max().unwrap_or(4);
    let max_possible_prime_factor = {
        let mut left = 0;
        let mut right = max_modulo;

        while left < right {
            let candidate = (left + right) / 2;
            if (candidate * candidate) > max_modulo {
                right = candidate;
            } else {
                left = candidate + 1;
            }
        }

        right
    };

    let sieve = build_prime_sieve(max_possible_prime_factor);
    let mut best_factor_by_prime: HashMap<usize, Congruence> = HashMap::new();

    for (prime, congruence) in congruences.iter().flat_map(|c| prime_factorise(c, &sieve)) {
        best_factor_by_prime
            .entry(prime)
            .and_modify(|current| {
                if congruence.modulo > current.modulo {
                    *current = congruence.clone();
                }
            })
            .or_insert(congruence);
    }
    best_factor_by_prime.into_values().collect()
}

fn chinese_remainder_theorem(congruences: &[Congruence]) -> usize {
    let m: usize = congruences.iter().map(|c| c.modulo).product();
    let crt_sum: usize = congruences
        .iter()
        .filter_map(|c| {
            let a = c.value;
            let m_x = Congruence {
                value: m / c.modulo,
                modulo: c.modulo,
            };
            let Some(inverse) = m_x.multiplicative_inverse() else {
            return None;
        };
            let rv = a * m_x.value * inverse;
            Some(rv)
        })
        .sum();
    if crt_sum == 0 {
        m
    } else {
        crt_sum % m
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(network) = Network::try_from(input) {
        network.steps_to_reach("AAA", "ZZZ")
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    if let Ok(network) = Network::try_from(input) {
        let loops = network.find_a_to_z_loops();
        let coprime = convert_congruences_to_coprime(&loops);
        Some(chinese_remainder_theorem(&coprime))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let parsed = Network::try_from(input).expect("Input parsing returned Ok");
        assert_eq!(parsed.directions, vec![Direction::Left, Direction::Right,],);
        assert_eq!(parsed.map.len(), 10);
        assert_eq!(parsed.map.get("AAA"), Some(&MapNode::Fork("BBB", "OOB")));
        assert_eq!(parsed.map.get("OOB"), Some(&MapNode::DeadEnd));
        assert_eq!(parsed.map.get("BBB"), Some(&MapNode::Fork("OOB", "CCC")));
        assert_eq!(parsed.map.get("CCC"), Some(&MapNode::Fork("ZZZ", "OOB")));
        assert_eq!(parsed.map.get("ZZZ"), Some(&MapNode::Fork("OOB", "CCC")));
        assert_eq!(parsed.map.get("GOA"), Some(&MapNode::Fork("DDD", "OOB")));
        assert_eq!(parsed.map.get("DDD"), Some(&MapNode::Fork("FFF", "EEE")));
        assert_eq!(parsed.map.get("EEE"), Some(&MapNode::Fork("FEZ", "OOB")));
        assert_eq!(parsed.map.get("FFF"), Some(&MapNode::Fork("OOB", "FEZ")));
        assert_eq!(parsed.map.get("FEZ"), Some(&MapNode::Fork("DDD", "DDD")));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_find_a_to_z_loops() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let network = Network::try_from(input).expect("Input parsing returned Ok");

        let loops = network.find_a_to_z_loops();
        assert_eq!(loops.len(), 2);
        assert!(loops.contains(&Congruence {
            value: 1,
            modulo: 2
        }));
        assert!(loops.contains(&Congruence {
            value: 0,
            modulo: 3
        }));
    }

    #[test]
    fn test_multiplicative_inverse() {
        assert_eq!(
            Congruence {
                value: 3,
                modulo: 5
            }
            .multiplicative_inverse(),
            Some(2)
        );
        assert_eq!(
            Congruence {
                value: 11,
                modulo: 13
            }
            .multiplicative_inverse(),
            Some(6)
        );
        assert_eq!(
            Congruence {
                value: 11,
                modulo: 26
            }
            .multiplicative_inverse(),
            Some(19)
        );
        assert_eq!(
            Congruence {
                value: 35,
                modulo: 3
            }
            .multiplicative_inverse(),
            Some(2)
        );
        assert_eq!(
            Congruence {
                value: 4,
                modulo: 2
            }
            .multiplicative_inverse(),
            None
        );
    }

    fn primes_below_thirty() -> Vec<bool> {
        vec![
            true, true, true, true, false, true, false, true, false, false, false, true, false,
            true, false, false, false, true, false, true, false, false, false, true, false, false,
            false, false, false, true,
        ]
    }

    #[test]
    fn test_build_prime_sieve() {
        assert_eq!(build_prime_sieve(30), primes_below_thirty());
    }

    #[test]
    fn test_prime_factorise() {
        let primes = primes_below_thirty();
        assert_eq!(
            prime_factorise(
                &Congruence {
                    value: 100,
                    modulo: 60
                },
                &primes
            ),
            vec![
                (
                    2,
                    Congruence {
                        value: 100,
                        modulo: 4
                    }
                ),
                (
                    3,
                    Congruence {
                        value: 100,
                        modulo: 3
                    }
                ),
                (
                    5,
                    Congruence {
                        value: 100,
                        modulo: 5
                    }
                ),
            ],
        );
        assert_eq!(
            prime_factorise(
                &Congruence {
                    value: 57,
                    modulo: 210
                },
                &primes
            ),
            vec![
                (
                    2,
                    Congruence {
                        value: 57,
                        modulo: 2
                    }
                ),
                (
                    3,
                    Congruence {
                        value: 57,
                        modulo: 3
                    }
                ),
                (
                    5,
                    Congruence {
                        value: 57,
                        modulo: 5
                    }
                ),
                (
                    7,
                    Congruence {
                        value: 57,
                        modulo: 7
                    }
                ),
            ]
        );
        assert_eq!(
            prime_factorise(
                &Congruence {
                    value: 7,
                    modulo: 31
                },
                &primes
            ),
            vec![(
                31,
                Congruence {
                    value: 7,
                    modulo: 31
                }
            ),]
        );
    }

    #[test]
    fn test_convert_congruences_to_coprime() {
        let congruences = vec![
            Congruence {
                value: 10,
                modulo: 24,
            },
            Congruence {
                value: 18,
                modulo: 64,
            },
            Congruence {
                value: 1,
                modulo: 35,
            },
        ];
        let mut converted = convert_congruences_to_coprime(&congruences);
        converted.sort_unstable_by_key(|c| c.modulo);
        assert_eq!(
            converted,
            vec![
                Congruence {
                    value: 10,
                    modulo: 3
                },
                Congruence {
                    value: 1,
                    modulo: 5
                },
                Congruence {
                    value: 1,
                    modulo: 7
                },
                Congruence {
                    value: 18,
                    modulo: 64
                },
            ]
        );
    }

    #[test]
    fn test_chinese_remainder_theorem() {
        let congruences = vec![
            Congruence {
                value: 2,
                modulo: 3,
            },
            Congruence {
                value: 3,
                modulo: 5,
            },
            Congruence {
                value: 2,
                modulo: 7,
            },
        ];
        assert_eq!(chinese_remainder_theorem(&congruences), 23);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
