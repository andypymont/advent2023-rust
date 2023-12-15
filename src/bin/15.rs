advent_of_code::solution!(15);

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0, |acc, ch| (acc + (ch as usize)) * 17 % 256)
}

#[derive(Debug, PartialEq)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Debug, PartialEq)]
struct LightBoxArray {
    boxes: Vec<Vec<Lens>>,
}

impl LightBoxArray {
    fn new() -> Self {
        let mut boxes = Vec::new();
        for _ in 0..256 {
            boxes.push(Vec::new());
        }

        Self { boxes }
    }

    fn insert(&mut self, lens: Lens) {
        let box_ix = hash(&lens.label);
        let replace = self.boxes[box_ix]
            .iter()
            .enumerate()
            .find(|(_ix, l)| l.label == lens.label);
        match replace {
            Some((ix, _)) => self.boxes[box_ix][ix] = lens,
            None => self.boxes[box_ix].push(lens),
        }
    }

    fn remove(&mut self, label: &str) {
        self.boxes[hash(label)].retain(|lens| lens.label != label);
    }

    fn step(&mut self, step: &str) -> bool {
        if let Some(label) = step.strip_suffix('-') {
            self.remove(label);
            return true;
        }

        if let Some((label, focal_length)) = step.split_once('=') {
            if let Ok(focal_length) = focal_length.parse() {
                let label = label.to_string();
                let lens = Lens {
                    label,
                    focal_length,
                };
                self.insert(lens);
                return true;
            }
        }

        false
    }

    fn total_focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(box_ix, lightbox)| {
                lightbox
                    .iter()
                    .enumerate()
                    .map(move |(lens_ix, lens)| (1 + box_ix) * (1 + lens_ix) * lens.focal_length)
            })
            .sum()
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Some(input.trim().split(',').map(hash).sum())
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let mut array = LightBoxArray::new();
    for step in input.trim().split(',') {
        array.step(step);
    }
    Some(array.total_focusing_power())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_lightbox_array() {
        let mut array = LightBoxArray::new();

        array.step("rn=1");
        assert_eq!(
            array.boxes[0],
            vec![Lens {
                label: "rn".to_string(),
                focal_length: 1
            }],
        );

        array.step("cm-");
        array.step("qp=3");
        assert_eq!(
            array.boxes[0],
            vec![Lens {
                label: "rn".to_string(),
                focal_length: 1
            }],
        );
        assert_eq!(
            array.boxes[1],
            vec![Lens {
                label: "qp".to_string(),
                focal_length: 3
            }],
        );

        array.step("pc=4");
        array.step("ot=9");
        array.step("ab=5");
        assert_eq!(
            array.boxes[3],
            vec![
                Lens {
                    label: "pc".to_string(),
                    focal_length: 4
                },
                Lens {
                    label: "ot".to_string(),
                    focal_length: 9
                },
                Lens {
                    label: "ab".to_string(),
                    focal_length: 5
                },
            ],
        );
        array.step("ot=13");
        assert_eq!(
            array.boxes[3],
            vec![
                Lens {
                    label: "pc".to_string(),
                    focal_length: 4
                },
                Lens {
                    label: "ot".to_string(),
                    focal_length: 13
                },
                Lens {
                    label: "ab".to_string(),
                    focal_length: 5
                },
            ],
        );
        array.step("pc-");
        assert_eq!(
            array.boxes[3],
            vec![
                Lens {
                    label: "ot".to_string(),
                    focal_length: 13
                },
                Lens {
                    label: "ab".to_string(),
                    focal_length: 5
                },
            ],
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
