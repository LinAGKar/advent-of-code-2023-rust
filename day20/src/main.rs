use std::{io::Read, collections::{HashMap, hash_map::Entry::{Occupied, Vacant}, VecDeque}};

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool /* Current state */),
    Conjunction(u64 /* Bitfield with state of inputs */),
}

fn parse<'a>(input: &'a str) -> Vec<(ModuleType, Vec<usize>)> {
    let mut modules = Vec::new();
    let mut indexes_by_name = HashMap::<&'a _, _>::new();
    let mut next_index = 1;

    // Helper for translating names into contiguous integers, so we can use a vec instead of HashMap
    let mut get_index = |name: &'a _| {
        match indexes_by_name.entry(name) {
            Occupied(entry) => *entry.get(),
            Vacant(entry) => {
                let index = next_index;
                next_index += 1;
                entry.insert(index);
                index
            }
        }
    };

    // Parse modules
    for line in input.lines() {
        let (name, destinations) = line.split_once(" -> ").unwrap();
        let (module_type, index) = if name == "broadcaster" {
            (ModuleType::Broadcaster, 0)
        } else {
            (match &name[..1] {
                "%" => ModuleType::FlipFlop(false),
                // Set all bits to one, so non-included inputs won't keep this from being active
                "&" => ModuleType::Conjunction(u64::MAX),
                _ => panic!(),
            }, get_index(&name[1..]))
        };

        let destinations: Vec<_> = destinations.split(", ").map(|x| get_index(x)).collect();
        if index >= modules.len() {
            modules.resize(index + 1, (ModuleType::Broadcaster, Vec::new()));
        }
        modules[index] = (module_type, destinations);
    }

    modules
}

fn part_1<'a>(input: &'a str) -> u32 {
    let mut modules = parse(input);

    // Zero-initialize conjunction inputs
    for i in 0..modules.len() {
        for j in 0..modules[i].1.len() {
            let target = modules[i].1[j];
            if let ModuleType::Conjunction(states) = &mut modules[target].0 {
                *states &= !(1 << i);
            }
        }
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back((false, 0, usize::MAX));
        low_pulses += 1;

        while let Some((pulse, target, source)) = pulses.pop_front() {
            if let Some(resulting_pulse) = match &mut modules[target].0 {
                ModuleType::Broadcaster => Some(pulse),
                ModuleType::Conjunction(state) => {
                    if pulse {
                        *state |= 1 << source;
                    } else {
                        *state &= !(1 << source);
                    }
                    Some(*state != u64::MAX)
                }
                ModuleType::FlipFlop(state) => {
                    if pulse {
                        None
                    } else {
                        *state = !*state;
                        Some(*state)
                    }
                }
            } {
                if resulting_pulse {
                    high_pulses += modules[target].1.len() as u32;
                } else {
                    low_pulses += modules[target].1.len() as u32;
                }

                for &new_target in &modules[target].1 {
                    pulses.push_back((resulting_pulse, new_target, target));
                }
            }
        }
    }

    low_pulses * high_pulses
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn part_2(input: &str) -> u64 {
    let modules = parse(input);

    let mut cycler_members = Vec::new();
    modules[0].1.iter().map(|&cycler_start| {
        let mut cycle_controller = 0;
        let mut current_module = cycler_start;
        let mut exit = false;

        while !exit {
            cycler_members.push(current_module);
            exit = true;
            for &target in &modules[current_module].1 {
                match modules[target].0 {
                    ModuleType::Conjunction(_) => { cycle_controller = target; }
                    ModuleType::FlipFlop(_) => {
                        current_module = target;
                        exit = false;
                    }
                    ModuleType::Broadcaster => panic!(),
                }
            }
        }

        let period = cycler_members.iter().enumerate().fold(0u64, |acc, (bit, &module)| {
            if modules[module].1.contains(&cycle_controller) {
                acc | (1 << bit)
            } else {
                acc
            }
        });

        cycler_members.clear();
        period
    }).reduce(lcm).unwrap()

    // 0
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}
