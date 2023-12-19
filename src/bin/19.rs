use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

advent_of_code::solution!(19);

#[derive(Debug, PartialEq)]
enum Property {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct WorkflowName(char, char, char);

#[derive(Debug, PartialEq)]
struct WorkStep {
    property: Property,
    comparison: Ordering,
    comparator: u32,
    target: WorkflowName,
}

#[derive(Debug, PartialEq)]
struct Workflow {
    name: WorkflowName,
    steps: Vec<WorkStep>,
    default: WorkflowName,
}

impl Workflow {
    fn process(&self, part: &Part) -> WorkflowName {
        for step in &self.steps {
            if part.matches(step) {
                return step.target;
            }
        }
        self.default
    }
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get(&self, property: &Property) -> u32 {
        match property {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }

    fn matches(&self, workstep: &WorkStep) -> bool {
        self.get(&workstep.property).cmp(&workstep.comparator) == workstep.comparison
    }

    fn total(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct WorkflowSystem {
    workflows: HashMap<WorkflowName, Workflow>,
    parts: Vec<Part>,
}

const ACCEPTED: WorkflowName = WorkflowName('A', ' ', ' ');

impl WorkflowSystem {
    fn process(&self, part: &Part) -> WorkflowName {
        let mut location = WorkflowName('i', 'n', ' ');

        while let Some(workflow) = self.workflows.get(&location) {
            location = workflow.process(part);
        }

        location
    }

    fn total_of_accepted_parts(&self) -> u32 {
        self.parts
            .iter()
            .filter_map(|part| {
                if self.process(part) == ACCEPTED {
                    Some(part.total())
                } else {
                    None
                }
            })
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseInputError;

impl FromStr for WorkflowName {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        Ok(WorkflowName(
            chars.next().unwrap_or(' '),
            chars.next().unwrap_or(' '),
            chars.next().unwrap_or(' '),
        ))
    }
}

impl FromStr for WorkStep {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((s, target_str)) = s.split_once(':') else {
            return Err(ParseInputError);
        };

        let mut chars = s.chars();
        let property = match chars.next() {
            Some('x') => Property::X,
            Some('m') => Property::M,
            Some('a') => Property::A,
            Some('s') => Property::S,
            _ => return Err(ParseInputError),
        };
        let comparison = match chars.next() {
            Some('>') => Ordering::Greater,
            Some('<') => Ordering::Less,
            _ => return Err(ParseInputError),
        };
        let comparator = s[2..].parse().map_err(|_| ParseInputError)?;
        let target = target_str.parse()?;

        Ok(Self {
            property,
            comparison,
            comparator,
            target,
        })
    }
}

impl FromStr for Workflow {
    type Err = ParseInputError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some(line) = line.strip_suffix('}') else {
            return Err(ParseInputError);
        };
        let Some((name_str, steps_str)) = line.split_once('{') else {
            return Err(ParseInputError);
        };

        let name = name_str.parse()?;
        let mut steps = Vec::new();
        let mut default: Result<WorkflowName, ParseInputError> = Err(ParseInputError);

        for step in steps_str.split(',') {
            if let Ok(step) = WorkStep::from_str(step) {
                steps.push(step);
            } else {
                default = WorkflowName::from_str(step);
            }
        }

        let default = default?;

        Ok(Self {
            name,
            steps,
            default,
        })
    }
}

impl FromStr for Part {
    type Err = ParseInputError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some(line) = line.strip_prefix('{') else {
            return Err(ParseInputError);
        };
        let Some(line) = line.strip_suffix('}') else {
            return Err(ParseInputError);
        };

        let mut x: Result<u32, ParseInputError> = Err(ParseInputError);
        let mut m: Result<u32, ParseInputError> = Err(ParseInputError);
        let mut a: Result<u32, ParseInputError> = Err(ParseInputError);
        let mut s: Result<u32, ParseInputError> = Err(ParseInputError);

        for element in line.split(',') {
            let Some((var, value_str)) = element.split_once('=') else {
                return Err(ParseInputError);
            };
            let value: u32 = value_str.parse().map_err(|_| ParseInputError)?;
            match var {
                "x" => x = Ok(value),
                "m" => m = Ok(value),
                "a" => a = Ok(value),
                "s" => s = Ok(value),
                _ => return Err(ParseInputError),
            }
        }

        let x = x?;
        let m = m?;
        let a = a?;
        let s = s?;
        Ok(Self { x, m, a, s })
    }
}

impl FromStr for WorkflowSystem {
    type Err = ParseInputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some((workflows_str, parts_str)) = input.split_once("\n\n") else {
            return Err(ParseInputError);
        };

        let mut workflows = HashMap::new();
        for workflow in workflows_str.lines() {
            let workflow: Workflow = workflow.parse()?;
            workflows.insert(workflow.name, workflow);
        }

        let mut parts = Vec::new();
        for part in parts_str.lines() {
            let part = part.parse()?;
            parts.push(part);
        }

        Ok(Self { workflows, parts })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(system) = WorkflowSystem::from_str(input) {
        Some(system.total_of_accepted_parts())
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

    fn example_system() -> WorkflowSystem {
        let mut workflows = HashMap::new();

        workflows.insert(
            WorkflowName('p', 'x', ' '),
            Workflow {
                name: WorkflowName('p', 'x', ' '),
                steps: vec![
                    WorkStep {
                        property: Property::A,
                        comparison: Ordering::Less,
                        comparator: 2006,
                        target: WorkflowName('q', 'k', 'q'),
                    },
                    WorkStep {
                        property: Property::M,
                        comparison: Ordering::Greater,
                        comparator: 2090,
                        target: WorkflowName('A', ' ', ' '),
                    },
                ],
                default: WorkflowName('r', 'f', 'g'),
            },
        );
        workflows.insert(
            WorkflowName('p', 'v', ' '),
            Workflow {
                name: WorkflowName('p', 'v', ' '),
                steps: vec![WorkStep {
                    property: Property::A,
                    comparison: Ordering::Greater,
                    comparator: 1716,
                    target: WorkflowName('R', ' ', ' '),
                }],
                default: WorkflowName('A', ' ', ' '),
            },
        );
        workflows.insert(
            WorkflowName('l', 'n', 'x'),
            Workflow {
                name: WorkflowName('l', 'n', 'x'),
                steps: vec![WorkStep {
                    property: Property::M,
                    comparison: Ordering::Greater,
                    comparator: 1548,
                    target: WorkflowName('A', ' ', ' '),
                }],
                default: WorkflowName('A', ' ', ' '),
            },
        );
        workflows.insert(
            WorkflowName('r', 'f', 'g'),
            Workflow {
                name: WorkflowName('r', 'f', 'g'),
                steps: vec![
                    WorkStep {
                        property: Property::S,
                        comparison: Ordering::Less,
                        comparator: 537,
                        target: WorkflowName('g', 'd', ' '),
                    },
                    WorkStep {
                        property: Property::X,
                        comparison: Ordering::Greater,
                        comparator: 2440,
                        target: WorkflowName('R', ' ', ' '),
                    },
                ],
                default: WorkflowName('A', ' ', ' '),
            },
        );
        workflows.insert(
            WorkflowName('q', 's', ' '),
            Workflow {
                name: WorkflowName('q', 's', ' '),
                steps: vec![WorkStep {
                    property: Property::S,
                    comparison: Ordering::Greater,
                    comparator: 3448,
                    target: WorkflowName('A', ' ', ' '),
                }],
                default: WorkflowName('l', 'n', 'x'),
            },
        );
        workflows.insert(
            WorkflowName('q', 'k', 'q'),
            Workflow {
                name: WorkflowName('q', 'k', 'q'),
                steps: vec![WorkStep {
                    property: Property::X,
                    comparison: Ordering::Less,
                    comparator: 1416,
                    target: WorkflowName('A', ' ', ' '),
                }],
                default: WorkflowName('c', 'r', 'n'),
            },
        );
        workflows.insert(
            WorkflowName('c', 'r', 'n'),
            Workflow {
                name: WorkflowName('c', 'r', 'n'),
                steps: vec![WorkStep {
                    property: Property::X,
                    comparison: Ordering::Greater,
                    comparator: 2662,
                    target: WorkflowName('A', ' ', ' '),
                }],
                default: WorkflowName('R', ' ', ' '),
            },
        );
        workflows.insert(
            WorkflowName('i', 'n', ' '),
            Workflow {
                name: WorkflowName('i', 'n', ' '),
                steps: vec![WorkStep {
                    property: Property::S,
                    comparison: Ordering::Less,
                    comparator: 1351,
                    target: WorkflowName('p', 'x', ' '),
                }],
                default: WorkflowName('q', 'q', 'z'),
            },
        );
        workflows.insert(
            WorkflowName('q', 'q', 'z'),
            Workflow {
                name: WorkflowName('q', 'q', 'z'),
                steps: vec![
                    WorkStep {
                        property: Property::S,
                        comparison: Ordering::Greater,
                        comparator: 2770,
                        target: WorkflowName('q', 's', ' '),
                    },
                    WorkStep {
                        property: Property::M,
                        comparison: Ordering::Less,
                        comparator: 1801,
                        target: WorkflowName('h', 'd', 'j'),
                    },
                ],
                default: WorkflowName('R', ' ', ' '),
            },
        );
        workflows.insert(
            WorkflowName('g', 'd', ' '),
            Workflow {
                name: WorkflowName('g', 'd', ' '),
                steps: vec![WorkStep {
                    property: Property::A,
                    comparison: Ordering::Greater,
                    comparator: 3333,
                    target: WorkflowName('R', ' ', ' '),
                }],
                default: WorkflowName('R', ' ', ' '),
            },
        );
        workflows.insert(
            WorkflowName('h', 'd', 'j'),
            Workflow {
                name: WorkflowName('h', 'd', 'j'),
                steps: vec![WorkStep {
                    property: Property::M,
                    comparison: Ordering::Greater,
                    comparator: 838,
                    target: WorkflowName('A', ' ', ' '),
                }],
                default: WorkflowName('p', 'v', ' '),
            },
        );

        let parts = vec![
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876,
            },
            Part {
                x: 1679,
                m: 44,
                a: 2067,
                s: 496,
            },
            Part {
                x: 2036,
                m: 264,
                a: 79,
                s: 2244,
            },
            Part {
                x: 2461,
                m: 1339,
                a: 466,
                s: 291,
            },
            Part {
                x: 2127,
                m: 1623,
                a: 2188,
                s: 1013,
            },
        ];
        WorkflowSystem { workflows, parts }
    }

    #[test]
    fn test_parse_input() {
        let example = example_system();
        let parsed =
            WorkflowSystem::from_str(&advent_of_code::template::read_file("examples", DAY))
                .expect("Input parsed without error");

        assert_eq!(example.parts, parsed.parts);
        assert_eq!(example.workflows.len(), parsed.workflows.len());
        for (name, workflow) in example.workflows {
            assert_eq!(parsed.workflows.get(&name), Some(&workflow));
        }
    }

    #[test]
    fn test_workflow_from_str() {
        assert_eq!(
            Workflow::from_str("px{a<2006:qkq,m>2090:A,rfg}"),
            Ok(Workflow {
                name: WorkflowName('p', 'x', ' '),
                steps: vec![
                    WorkStep {
                        property: Property::A,
                        comparison: Ordering::Less,
                        comparator: 2006,
                        target: WorkflowName('q', 'k', 'q'),
                    },
                    WorkStep {
                        property: Property::M,
                        comparison: Ordering::Greater,
                        comparator: 2090,
                        target: WorkflowName('A', ' ', ' '),
                    },
                ],
                default: WorkflowName('r', 'f', 'g'),
            }),
        );
        assert_eq!(
            Workflow::from_str("qs{s>3448:A,lnx}"),
            Ok(Workflow {
                name: WorkflowName('q', 's', ' '),
                steps: vec![WorkStep {
                    property: Property::S,
                    comparison: Ordering::Greater,
                    comparator: 3448,
                    target: WorkflowName('A', ' ', ' '),
                },],
                default: WorkflowName('l', 'n', 'x'),
            }),
        );
    }

    #[test]
    fn test_work_step_from_str() {
        assert_eq!(
            WorkStep::from_str("a>2006:qkq"),
            Ok(WorkStep {
                property: Property::A,
                comparison: Ordering::Greater,
                comparator: 2006,
                target: WorkflowName('q', 'k', 'q'),
            }),
        );
        assert_eq!(
            WorkStep::from_str("s<537:gd"),
            Ok(WorkStep {
                property: Property::S,
                comparison: Ordering::Less,
                comparator: 537,
                target: WorkflowName('g', 'd', ' '),
            }),
        );
    }

    #[test]
    fn test_part_from_str() {
        assert_eq!(
            Part::from_str("{x=787,m=2655,a=1222,s=2876}"),
            Ok(Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            }),
        );
        assert_eq!(
            Part::from_str("{x=2461,m=1339,a=466,s=291}"),
            Ok(Part {
                x: 2461,
                m: 1339,
                a: 466,
                s: 291
            }),
        );
    }

    #[test]
    fn test_part_matches_work_step() {
        let workstep = WorkStep {
            property: Property::S,
            comparison: Ordering::Less,
            comparator: 1351,
            target: WorkflowName('p', 'x', ' '),
        };
        let part = Part {
            x: 1679,
            m: 44,
            a: 2067,
            s: 496,
        };
        assert!(part.matches(&workstep));

        let part = Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
        };
        assert!(!part.matches(&workstep));

        let workstep = WorkStep {
            property: Property::A,
            comparison: Ordering::Greater,
            comparator: 1201,
            target: WorkflowName('z', 'x', 'y'),
        };
        assert!(part.matches(&workstep));
    }

    #[test]
    fn test_workflow_process() {
        let workflow = Workflow {
            name: WorkflowName('q', 'q', 'z'),
            steps: vec![
                WorkStep {
                    property: Property::S,
                    comparison: Ordering::Greater,
                    comparator: 2770,
                    target: WorkflowName('q', 's', ' '),
                },
                WorkStep {
                    property: Property::M,
                    comparison: Ordering::Less,
                    comparator: 1801,
                    target: WorkflowName('h', 'd', 'j'),
                },
            ],
            default: WorkflowName('R', ' ', ' '),
        };

        assert_eq!(
            workflow.process(&Part {
                x: 50,
                m: 1000,
                a: 1374,
                s: 2771
            }),
            WorkflowName('q', 's', ' '),
        );
        assert_eq!(
            workflow.process(&Part {
                x: 50,
                m: 1000,
                a: 1374,
                s: 2541
            }),
            WorkflowName('h', 'd', 'j'),
        );
        assert_eq!(
            workflow.process(&Part {
                x: 50,
                m: 1807,
                a: 1374,
                s: 2541
            }),
            WorkflowName('R', ' ', ' '),
        );
    }

    #[test]
    fn test_system_process() {
        let system = example_system();
        assert_eq!(
            system.process(&Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            }),
            WorkflowName('A', ' ', ' '),
        );
        assert_eq!(
            system.process(&Part {
                x: 1679,
                m: 44,
                a: 2067,
                s: 496
            }),
            WorkflowName('R', ' ', ' '),
        );
        assert_eq!(
            system.process(&Part {
                x: 2036,
                m: 264,
                a: 79,
                s: 2244
            }),
            WorkflowName('A', ' ', ' '),
        );
        assert_eq!(
            system.process(&Part {
                x: 2461,
                m: 1339,
                a: 466,
                s: 291
            }),
            WorkflowName('R', ' ', ' '),
        );
        assert_eq!(
            system.process(&Part {
                x: 2127,
                m: 1623,
                a: 2188,
                s: 1013
            }),
            WorkflowName('A', ' ', ' '),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19_114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
