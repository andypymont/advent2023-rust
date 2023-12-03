use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn maximums(&self) -> Draw {
        self.draws.iter().fold(
            Draw {
                red: 0,
                green: 0,
                blue: 0,
            },
            |a, b| Draw {
                red: a.red.max(b.red),
                green: a.green.max(b.green),
                blue: a.blue.max(b.blue),
            },
        )
    }

    fn possible(&self, bag: &Draw) -> bool {
        let max = self.maximums();
        max.red <= bag.red && max.green <= bag.green && max.blue <= bag.blue
    }

    fn power(&self) -> u32 {
        self.maximums().power()
    }
}

#[derive(Debug, PartialEq)]
struct ParseGameError;

impl FromStr for Draw {
    type Err = ParseGameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for colour_draw in text.split(", ") {
            if let Some((qty_str, colour)) = colour_draw.split_once(' ') {
                let quantity = qty_str.parse().map_err(|_| ParseGameError)?;
                match colour {
                    "red" => red = quantity,
                    "green" => green = quantity,
                    "blue" => blue = quantity,
                    _ => return Err(ParseGameError),
                }
            } else {
                return Err(ParseGameError);
            }
        }

        Ok(Draw { red, green, blue })
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if let Some((id_str, draws_str)) = text.split_once(": ") {
            if let Some(id_str) = id_str.strip_prefix("Game ") {
                let id = id_str.parse().map_err(|_| ParseGameError)?;
                let mut draws = Vec::new();

                for draw_str in draws_str.split("; ") {
                    let draw = draw_str.parse()?;
                    draws.push(draw);
                }

                Ok(Game { id, draws })
            } else {
                Err(ParseGameError)
            }
        } else {
            Err(ParseGameError)
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let bag = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    Some(
        input
            .lines()
            .filter_map(|line| match line.parse::<Game>() {
                Ok(game) => {
                    if game.possible(&bag) {
                        Some(game.id)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            })
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| match line.parse::<Game>() {
                Ok(game) => Some(game.power()),
                Err(_) => None,
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_draw() {
        assert_eq!(
            "3 blue, 4 red".parse(),
            Ok(Draw {
                red: 4,
                green: 0,
                blue: 3
            }),
        );
        assert_eq!(
            "17 green".parse(),
            Ok(Draw {
                red: 0,
                green: 17,
                blue: 0
            }),
        );
        assert_eq!(
            "1 red, 2 green, 5 blue".parse(),
            Ok(Draw {
                red: 1,
                green: 2,
                blue: 5
            }),
        );
    }

    #[test]
    fn test_parse_game_error() {
        assert_eq!(
            "Game 1: 3 blu, 4 red; 1 red, 2 green, and 6 blue; 2G".parse::<Game>(),
            Err(ParseGameError)
        );
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse(),
            Ok(Game {
                id: 1,
                draws: vec![
                    Draw {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Draw {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Draw {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            })
        );
        assert_eq!(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".parse(),
            Ok(Game {
                id: 4,
                draws: vec![
                    Draw {
                        red: 3,
                        green: 1,
                        blue: 6
                    },
                    Draw {
                        red: 6,
                        green: 3,
                        blue: 0
                    },
                    Draw {
                        red: 14,
                        green: 3,
                        blue: 15
                    },
                ]
            })
        );
    }

    #[test]
    fn test_game_maximums() {
        let game = Game {
            id: 3,
            draws: vec![
                Draw {
                    red: 4,
                    green: 0,
                    blue: 0,
                },
                Draw {
                    red: 5,
                    green: 2,
                    blue: 0,
                },
                Draw {
                    red: 0,
                    green: 1,
                    blue: 0,
                },
            ],
        };
        assert_eq!(
            game.maximums(),
            Draw {
                red: 5,
                green: 2,
                blue: 0
            }
        )
    }

    #[test]
    fn test_game_possible() {
        let test = Draw {
            red: 12,
            green: 13,
            blue: 14,
        };

        let one = Game {
            id: 1,
            draws: vec![
                Draw {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Draw {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Draw {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };
        assert_eq!(one.possible(&test), true);

        let three = Game {
            id: 3,
            draws: vec![
                Draw {
                    red: 20,
                    green: 8,
                    blue: 6,
                },
                Draw {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
                Draw {
                    red: 1,
                    green: 5,
                    blue: 0,
                },
            ],
        };
        assert_eq!(three.possible(&test), false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_power() {
        let one = Game {
            id: 1,
            draws: vec![
                Draw {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Draw {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Draw {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };
        assert_eq!(one.power(), 48);

        let three = Game {
            id: 3,
            draws: vec![
                Draw {
                    red: 20,
                    green: 8,
                    blue: 6,
                },
                Draw {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
                Draw {
                    red: 1,
                    green: 5,
                    blue: 0,
                },
            ],
        };
        assert_eq!(three.power(), 1560);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
