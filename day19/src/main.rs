use std::{io::Read, cmp::{min, max}, collections::{HashMap, hash_map::Entry::{Vacant, Occupied}}};

#[derive(Debug, Clone, Copy)]
enum Condition {
    True,
    Greater(u8, u16),
    Less(u8, u16),
}

#[derive(Debug, Clone, Copy)]
enum Target {
    Accept,
    Reject,
    Workflow(usize),
}

struct WorkflowParser<'a> {
    indexes_by_name: HashMap<&'a str, usize>,
    next_index: usize,
}

impl<'a> WorkflowParser<'a> {
    fn get_index(&mut self, name: &'a str) -> usize {
        match self.indexes_by_name.entry(name) {
            Vacant(entry) => {
                let index = self.next_index;
                entry.insert(index);
                self.next_index += 1;
                index
            },
            Occupied(entry) => *entry.get(),
        }
    }

    fn get_target(&mut self, target_str: &'a str) -> Target {
        match target_str {
            "A" => Target::Accept,
            "R" => Target::Reject,
            _ => Target::Workflow(self.get_index(target_str)),
        }
    }

    fn parse_val_name(name: &str) -> u8 {
        ["x", "m", "a", "s"].iter().position(|&x| x == name).unwrap() as u8
    }

    fn parse_workflows<T: Iterator<Item = &'a str>>(mut lines: T) -> (Vec<Vec<(Condition, Target)>>, usize) {
        let mut parser = Self {
            indexes_by_name: HashMap::new(),
            next_index: 0,
        };
        let mut workflows = Vec::new();

        while let Some(line) = lines.next().filter(|&x| !x.is_empty()) {
            let mut parts = line.split_terminator(&['{', '}', ',']);
            let index = parser.get_index(parts.next().unwrap());
            let rules: Vec<_> = parts.map(|part| {
                if let Some((condition, target_str)) = part.split_once(':') {
                    (
                        if let Some((val, num)) = condition.split_once('>') {
                            Condition::Greater(Self::parse_val_name(val), num.parse().unwrap())
                        } else if let Some((val, num)) = condition.split_once('<') {
                            Condition::Less(Self::parse_val_name(val), num.parse().unwrap())
                        } else {
                            panic!();
                        },
                        parser.get_target(target_str),
                    )
                } else {
                    (Condition::True, parser.get_target(part))
                }
            }).collect();

            if workflows.len() <= index {
                workflows.resize(index + 1, vec![]);
            }

            workflows[index] = rules;
        }

        let start_workflow = parser.get_index("in");

        (workflows, start_workflow)
    }
}

fn part_1<'a>(input: &'a str) -> u32 {
    let mut lines = input.lines();
    let (workflows, start_workflow) = WorkflowParser::parse_workflows(&mut lines);

    lines.filter_map(|line| {
        let mut values = [0; 4];
        for (src, dst) in line.trim_start_matches('{').trim_end_matches('}').split(',').map(|x| {
            x.split_once('=').unwrap().1.parse().unwrap()
        }).zip(&mut values) {
            *dst = src;
        }

        let mut workflow_index = start_workflow;
        let mut rule_index = 0;

        loop {
            let (condition, target) = workflows[workflow_index][rule_index];
            if match condition {
                Condition::Greater(val, num) => values[val as usize] > num,
                Condition::Less(val, num) => values[val as usize] < num,
                Condition::True => true,
            } {
                match target {
                    Target::Accept => { break Some(values.into_iter().map(|x| x as u32).sum::<u32>()); }
                    Target::Reject => { break None; }
                    Target::Workflow(index) => { workflow_index = index; }
                }
                rule_index = 0;
            } else {
                rule_index += 1;
            }
        }
    }).sum()
}

fn part_2(input: &str) -> u64 {
    let (workflows, start_workflow) = WorkflowParser::parse_workflows(input.lines());

    let mut possibilities = vec![(start_workflow, [[1, 4000]; 4])];
    let mut possible_combinations = 0;

    while let Some((index, mut ranges)) = possibilities.pop() {
        for &(condition, target) in &workflows[index] {
            let mut matching = ranges;
            match condition {
                Condition::Greater(val, num) => {
                    matching[val as usize][0] = max(matching[val as usize][0], num + 1);
                    ranges[val as usize][1] = min(ranges[val as usize][1], num);
                }

                Condition::Less(val, num) => {
                    matching[val as usize][1] = min(matching[val as usize][1], num - 1);
                    ranges[val as usize][0] = max(ranges[val as usize][0], num);
                }

                Condition::True => {
                    ranges = [[1, 0]; 4];
                }
            }

            match target {
                Target::Accept => {
                    possible_combinations += matching.into_iter().map(|x| if x[0] > x[1] { 0 } else { (x[1] - x[0] + 1) as u64 }).product::<u64>();
                }

                Target::Reject => {}
                Target::Workflow(index) => {
                    possibilities.push((index, matching));
                }
            }
        }
    }

    possible_combinations
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
