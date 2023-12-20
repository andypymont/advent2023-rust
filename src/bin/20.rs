use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

advent_of_code::solution!(20);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ModuleName {
    Broadcaster,
    Other(char, char),
}

#[derive(Debug, PartialEq)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction(HashSet<ModuleName>),
}

impl ModuleType {
    fn new_conjunction() -> Self {
        Self::Conjunction(HashSet::new())
    }
}

#[derive(Debug, PartialEq)]
struct Module {
    name: ModuleName,
    module_type: ModuleType,
    destinations: HashSet<ModuleName>,
}

impl Module {
    fn send_signals(
        &self,
        pulse: Pulse,
        last_sent: &mut HashMap<ModuleName, Pulse>,
        signals: &mut VecDeque<Signal>,
    ) {
        last_sent.insert(self.name, pulse);
        signals.extend(self.destinations.iter().map(|destination| Signal {
            destination: *destination,
            pulse,
        }));
    }
}

#[derive(Debug, PartialEq)]
struct Signal {
    pulse: Pulse,
    destination: ModuleName,
}

impl Signal {
    fn initial() -> Self {
        Self {
            pulse: Pulse::Low,
            destination: ModuleName::Broadcaster,
        }
    }
}

#[derive(Debug, PartialEq)]
struct ModuleSystem {
    modules: HashMap<ModuleName, Module>,
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    if a == 0 || b == 0 {
        a | b
    } else {
        let shift = (a | b).trailing_zeros();

        a >>= a.trailing_zeros();
        b >>= b.trailing_zeros();

        while a != b {
            if a > b {
                a -= b;
                a >>= a.trailing_zeros();
            } else {
                b -= a;
                b >>= b.trailing_zeros();
            }
        }

        a << shift
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 && b == 0 {
        0
    } else {
        a * (b / gcd(a, b))
    }
}

impl ModuleSystem {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    fn find_lcm_of_signals_to_destinations(&self, destinations: &HashSet<ModuleName>) -> u64 {
        let mut signal_lcm = 1;
        let mut to_find = destinations.clone();

        let mut signals = VecDeque::new();
        let mut flipflop_state: HashMap<ModuleName, bool> = HashMap::new();
        let mut last_sent: HashMap<ModuleName, Pulse> = HashMap::new();

        for presses in 1.. {
            signals.push_back(Signal::initial());

            while let Some(signal) = signals.pop_front() {
                if signal.pulse == Pulse::Low && to_find.contains(&signal.destination) {
                    to_find.remove(&signal.destination);
                    signal_lcm = lcm(signal_lcm, presses);
                    if to_find.is_empty() {
                        return signal_lcm;
                    }
                }

                self.process_signal(&signal, &mut signals, &mut flipflop_state, &mut last_sent);
            }
        }

        signal_lcm
    }

    fn press_button_times(&self, times: usize) -> (u32, u32) {
        let mut low = 0;
        let mut high = 0;

        let mut signals = VecDeque::new();
        let mut flipflop_state: HashMap<ModuleName, bool> = HashMap::new();
        let mut last_sent: HashMap<ModuleName, Pulse> = HashMap::new();

        for _ in 0..times {
            signals.push_back(Signal::initial());

            while let Some(signal) = signals.pop_front() {
                match signal.pulse {
                    Pulse::Low => low += 1,
                    Pulse::High => high += 1,
                }

                self.process_signal(&signal, &mut signals, &mut flipflop_state, &mut last_sent);
            }
        }

        (low, high)
    }

    fn process_signal(
        &self,
        signal: &Signal,
        signals: &mut VecDeque<Signal>,
        flipflop_state: &mut HashMap<ModuleName, bool>,
        last_sent: &mut HashMap<ModuleName, Pulse>,
    ) {
        if let Some(module) = self.modules.get(&signal.destination) {
            match &module.module_type {
                ModuleType::Broadcaster => {
                    module.send_signals(signal.pulse, last_sent, signals);
                }
                ModuleType::FlipFlop => {
                    if signal.pulse == Pulse::Low {
                        if *flipflop_state.get(&module.name).unwrap_or(&false) {
                            flipflop_state.insert(module.name, false);
                            module.send_signals(Pulse::Low, last_sent, signals);
                        } else {
                            flipflop_state.insert(module.name, true);
                            module.send_signals(Pulse::High, last_sent, signals);
                        }
                    }
                }
                ModuleType::Conjunction(inputs) => {
                    if inputs
                        .iter()
                        .all(|input| last_sent.get(input).unwrap_or(&Pulse::Low) == &Pulse::High)
                    {
                        module.send_signals(Pulse::Low, last_sent, signals);
                    } else {
                        module.send_signals(Pulse::High, last_sent, signals);
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseModuleError;

impl FromStr for ModuleName {
    type Err = ParseModuleError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        if name == "broadcaster" {
            return Ok(Self::Broadcaster);
        }

        let mut chars = name.chars();
        if let (Some(a), Some(b), None) = (chars.next(), chars.next(), chars.next()) {
            Ok(Self::Other(a, b))
        } else {
            Err(ParseModuleError)
        }
    }
}

impl FromStr for Module {
    type Err = ParseModuleError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some((details, destinations_str)) = line.split_once(" -> ") else {
            return Err(ParseModuleError);
        };

        let (name, module_type) = if details == "broadcaster" {
            (ModuleName::Broadcaster, ModuleType::Broadcaster)
        } else if let Some(name) = details.strip_prefix('%') {
            let name = ModuleName::from_str(name)?;
            (name, ModuleType::FlipFlop)
        } else if let Some(name) = details.strip_prefix('&') {
            let name = ModuleName::from_str(name)?;
            (name, ModuleType::new_conjunction())
        } else {
            return Err(ParseModuleError);
        };

        let mut destinations = HashSet::new();
        for dest in destinations_str.split(", ") {
            let dest = ModuleName::from_str(dest)?;
            destinations.insert(dest);
        }

        Ok(Self {
            name,
            module_type,
            destinations,
        })
    }
}

impl FromStr for ModuleSystem {
    type Err = ParseModuleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut system = Self::new();

        for line in input.lines() {
            let module = Module::from_str(line)?;
            system.modules.insert(module.name, module);
        }

        let mut connections = Vec::new();
        for output in system.modules.values() {
            for input in &output.destinations {
                connections.push((output.name, *input));
            }
        }
        for (output, input) in connections {
            if let Some(input) = system.modules.get_mut(&input) {
                if let ModuleType::Conjunction(ref mut tracker) = &mut input.module_type {
                    tracker.insert(output);
                }
            }
        }

        Ok(system)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(system) = ModuleSystem::from_str(input) {
        let (low, high) = system.press_button_times(1000);
        Some(low * high)
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    if let Ok(system) = ModuleSystem::from_str(input) {
        let seek = ModuleName::Other('r', 'x');
        if let Some(module) = system
            .modules
            .values()
            .find(|m| m.destinations.contains(&seek))
        {
            if let ModuleType::Conjunction(sources) = &module.module_type {
                return Some(system.find_lcm_of_signals_to_destinations(sources));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_name_from_str() {
        assert_eq!(
            ModuleName::from_str("broadcaster"),
            Ok(ModuleName::Broadcaster),
        );
        assert_eq!(ModuleName::from_str("xy"), Ok(ModuleName::Other('x', 'y')),);
        assert_eq!(ModuleName::from_str("op"), Ok(ModuleName::Other('o', 'p')),);
    }

    #[test]
    fn test_module_from_str() {
        let mut destinations = HashSet::new();
        destinations.insert(ModuleName::Other('b', 'b'));
        assert_eq!(
            Module::from_str("%aa -> bb"),
            Ok(Module {
                name: ModuleName::Other('a', 'a'),
                module_type: ModuleType::FlipFlop,
                destinations,
            }),
        );

        let mut destinations = HashSet::new();
        destinations.insert(ModuleName::Other('c', 'c'));
        destinations.insert(ModuleName::Other('d', 'd'));
        assert_eq!(
            Module::from_str("&bb -> cc, dd"),
            Ok(Module {
                name: ModuleName::Other('b', 'b'),
                module_type: ModuleType::new_conjunction(),
                destinations,
            }),
        );

        let mut destinations = HashSet::new();
        destinations.insert(ModuleName::Other('a', 'a'));
        destinations.insert(ModuleName::Other('b', 'b'));
        destinations.insert(ModuleName::Other('c', 'c'));
        assert_eq!(
            Module::from_str("broadcaster -> aa, bb, cc"),
            Ok(Module {
                name: ModuleName::Broadcaster,
                module_type: ModuleType::Broadcaster,
                destinations,
            }),
        );
    }

    fn example_system() -> ModuleSystem {
        let mut system = ModuleSystem::new();

        let mut destinations = HashSet::new();
        destinations.insert(ModuleName::Other('a', 'a'));
        system.modules.insert(
            ModuleName::Broadcaster,
            Module {
                name: ModuleName::Broadcaster,
                module_type: ModuleType::Broadcaster,
                destinations,
            },
        );

        let mut destinations = HashSet::new();
        destinations.insert(ModuleName::Other('i', 'v'));
        destinations.insert(ModuleName::Other('c', 'n'));
        system.modules.insert(
            ModuleName::Other('a', 'a'),
            Module {
                name: ModuleName::Other('a', 'a'),
                module_type: ModuleType::FlipFlop,
                destinations,
            },
        );

        let mut destinations = HashSet::new();
        destinations.insert(ModuleName::Other('b', 'b'));
        let mut data = HashSet::new();
        data.insert(ModuleName::Other('a', 'a'));
        system.modules.insert(
            ModuleName::Other('i', 'v'),
            Module {
                name: ModuleName::Other('i', 'v'),
                module_type: ModuleType::Conjunction(data),
                destinations,
            },
        );

        let mut destinations = HashSet::new();
        destinations.insert(ModuleName::Other('c', 'n'));
        system.modules.insert(
            ModuleName::Other('b', 'b'),
            Module {
                name: ModuleName::Other('b', 'b'),
                module_type: ModuleType::FlipFlop,
                destinations,
            },
        );

        let mut destinations = HashSet::new();
        destinations.insert(ModuleName::Other('o', 'p'));
        let mut data = HashSet::new();
        data.insert(ModuleName::Other('a', 'a'));
        data.insert(ModuleName::Other('b', 'b'));
        system.modules.insert(
            ModuleName::Other('c', 'n'),
            Module {
                name: ModuleName::Other('c', 'n'),
                module_type: ModuleType::Conjunction(data),
                destinations,
            },
        );
        system
    }

    #[test]
    fn test_system_from_str() {
        let example_str = "broadcaster -> aa\n\
                           %aa -> iv, cn\n\
                           &iv -> bb\n\
                           %bb -> cn\n\
                           &cn -> op";

        assert_eq!(ModuleSystem::from_str(example_str), Ok(example_system()),)
    }

    #[test]
    fn test_module_send_signals() {
        let mut destinations = HashSet::new();
        destinations.insert(ModuleName::Other('a', 'a'));
        destinations.insert(ModuleName::Other('x', 'y'));
        let module = Module {
            name: ModuleName::Broadcaster,
            module_type: ModuleType::Broadcaster,
            destinations,
        };

        let mut signals = VecDeque::new();
        let mut last_sent = HashMap::new();
        module.send_signals(Pulse::High, &mut last_sent, &mut signals);

        assert_eq!(signals.len(), 2);
        let sent_to: HashSet<ModuleName> = signals
            .iter()
            .filter_map(|signal| {
                if signal.pulse == Pulse::High {
                    Some(signal.destination)
                } else {
                    None
                }
            })
            .collect();
        assert!(sent_to.contains(&ModuleName::Other('a', 'a')));
        assert!(sent_to.contains(&ModuleName::Other('x', 'y')));
    }

    #[test]
    fn test_system_press_button_times() {
        let system = example_system();
        assert_eq!(system.press_button_times(1000), (4250, 2750));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(219_152_390));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14_182_027));
    }
}
