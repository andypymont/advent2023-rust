use std::str::FromStr;

advent_of_code::solution!(7);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: [usize; 5]) -> Self {
        let mut counts: [usize; 15] = [0; 15];
        for card in cards {
            counts[card] += 1;
        }
        let (values, max) = counts[1..].iter().fold((0, 0), |(values, max), value| {
            (values + usize::from(value > &0), max.max(*value))
        });

        match max + counts[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if values == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if values == 3 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: [usize; 5],
    bid: u32,
}

#[derive(Debug, PartialEq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some((cards_str, bid_str)) = line.split_once(' ') {
            if cards_str.len() != 5 {
                return Err(ParseHandError);
            }

            let mut cards = [0; 5];
            for (ix, card) in cards_str.chars().enumerate() {
                if ix >= 5 {
                    return Err(ParseHandError);
                }

                let card = match card {
                    'j' => Ok(0),
                    '2' => Ok(2),
                    '3' => Ok(3),
                    '4' => Ok(4),
                    '5' => Ok(5),
                    '6' => Ok(6),
                    '7' => Ok(7),
                    '8' => Ok(8),
                    '9' => Ok(9),
                    'T' => Ok(10),
                    'J' => Ok(11),
                    'Q' => Ok(12),
                    'K' => Ok(13),
                    'A' => Ok(14),
                    _ => Err(ParseHandError),
                }?;
                cards[ix] = card;
            }

            let bid = bid_str.parse().map_err(|_| ParseHandError)?;
            let hand_type = HandType::from_cards(cards);

            Ok(Hand {
                hand_type,
                cards,
                bid,
            })
        } else {
            Err(ParseHandError)
        }
    }
}

fn read_hands(input: &str) -> Result<Vec<Hand>, ParseHandError> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let hand = line.parse()?;
        hands.push(hand);
    }
    Ok(hands)
}

fn read_joker_hands(input: &str) -> Result<Vec<Hand>, ParseHandError> {
    read_hands(&input.replace('J', "j"))
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(mut hands) = read_hands(input) {
        hands.sort_unstable();
        Some(
            hands
                .iter()
                .enumerate()
                .map(|(ix, hand)| u32::try_from(ix + 1).unwrap_or(0) * hand.bid)
                .sum(),
        )
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    if let Ok(mut hands) = read_joker_hands(input) {
        hands.sort_unstable();
        Some(
            hands
                .iter()
                .enumerate()
                .map(|(ix, hand)| u32::try_from(ix + 1).unwrap_or(0) * hand.bid)
                .sum(),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_hands() -> Vec<Hand> {
        vec![
            Hand {
                cards: [3, 2, 10, 3, 13],
                hand_type: HandType::OnePair,
                bid: 765,
            },
            Hand {
                cards: [10, 5, 5, 11, 5],
                hand_type: HandType::ThreeOfAKind,
                bid: 684,
            },
            Hand {
                cards: [13, 13, 6, 7, 7],
                hand_type: HandType::TwoPair,
                bid: 28,
            },
            Hand {
                cards: [13, 10, 11, 11, 10],
                hand_type: HandType::TwoPair,
                bid: 220,
            },
            Hand {
                cards: [12, 12, 12, 11, 14],
                hand_type: HandType::ThreeOfAKind,
                bid: 483,
            },
        ]
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            "32T3K 765".parse(),
            Ok(Hand {
                cards: [3, 2, 10, 3, 13],
                hand_type: HandType::OnePair,
                bid: 765,
            }),
        );
        assert_eq!(
            "T55J5 684".parse(),
            Ok(Hand {
                cards: [10, 5, 5, 11, 5],
                hand_type: HandType::ThreeOfAKind,
                bid: 684,
            }),
        );
        assert_eq!(
            "KK677 28".parse(),
            Ok(Hand {
                cards: [13, 13, 6, 7, 7],
                hand_type: HandType::TwoPair,
                bid: 28,
            }),
        )
    }

    #[test]
    fn test_read_hands() {
        assert_eq!(
            read_hands(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_hands())
        );
    }

    #[test]
    fn test_sort_hands() {
        let mut hands = example_hands();
        hands.sort_unstable();
        assert_eq!(
            hands,
            vec![
                Hand {
                    cards: [3, 2, 10, 3, 13],
                    hand_type: HandType::OnePair,
                    bid: 765
                },
                Hand {
                    cards: [13, 10, 11, 11, 10],
                    hand_type: HandType::TwoPair,
                    bid: 220
                },
                Hand {
                    cards: [13, 13, 6, 7, 7],
                    hand_type: HandType::TwoPair,
                    bid: 28
                },
                Hand {
                    cards: [10, 5, 5, 11, 5],
                    hand_type: HandType::ThreeOfAKind,
                    bid: 684
                },
                Hand {
                    cards: [12, 12, 12, 11, 14],
                    hand_type: HandType::ThreeOfAKind,
                    bid: 483
                },
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_parse_joker_hand() {
        assert_eq!(
            "T55j5 684".parse(),
            Ok(Hand {
                cards: [10, 5, 5, 0, 5],
                hand_type: HandType::FourOfAKind,
                bid: 684,
            }),
        );
        assert_eq!(
            "KTjjT 220".parse(),
            Ok(Hand {
                cards: [13, 10, 0, 0, 10],
                hand_type: HandType::FourOfAKind,
                bid: 220,
            }),
        );
        assert_eq!(
            "234jj 137".parse(),
            Ok(Hand {
                cards: [2, 3, 4, 0, 0],
                hand_type: HandType::ThreeOfAKind,
                bid: 137,
            }),
        );
        assert_eq!(
            "2345j 9652".parse(),
            Ok(Hand {
                cards: [2, 3, 4, 5, 0],
                hand_type: HandType::OnePair,
                bid: 9652,
            }),
        )
    }

    #[test]
    fn test_read_joker_hands() {
        assert_eq!(
            read_joker_hands(&advent_of_code::template::read_file("examples", DAY)),
            Ok(vec![
                Hand {
                    cards: [3, 2, 10, 3, 13],
                    hand_type: HandType::OnePair,
                    bid: 765,
                },
                Hand {
                    cards: [10, 5, 5, 0, 5],
                    hand_type: HandType::FourOfAKind,
                    bid: 684,
                },
                Hand {
                    cards: [13, 13, 6, 7, 7],
                    hand_type: HandType::TwoPair,
                    bid: 28,
                },
                Hand {
                    cards: [13, 10, 0, 0, 10],
                    hand_type: HandType::FourOfAKind,
                    bid: 220,
                },
                Hand {
                    cards: [12, 12, 12, 0, 14],
                    hand_type: HandType::FourOfAKind,
                    bid: 483,
                },
            ]),
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
