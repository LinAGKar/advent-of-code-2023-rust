use std::{io::Read, collections::HashMap};

fn parse_node(src: &str) -> u16 {
    src.chars().fold(0, |acc, c| {
        (acc << 5) | (c as u16 - 'A' as u16)
    })
}

fn part_1(input: &str) -> u16 {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    let start = parse_node("AAA");
    let goal = parse_node("ZZZ");
    let mut nodes = vec![(0, 0); goal as usize + 1];
    for line in lines.skip(1) {
        nodes[parse_node(&line[0..3]) as usize] = (parse_node(&line[7..10]), parse_node(&line[12..15]));
    }
    directions.chars().cycle().zip(1..).try_fold(start, |pos, (direction, n)| {
        let node = nodes[pos as usize];
        let new_pos = match direction {
            'L' => node.0,
            'R' => node.1,
            _ => panic!(),
        };
        if new_pos == goal {
            Err(n)
        } else {
            Ok(new_pos)
        }
    }).err().unwrap()
}

#[derive(Debug)]
struct Cycle {
    period: u16,
    offset: u16,
    goal_offsets: Vec<u16>,
}

fn part_2(input: &str) -> u64 {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    let mut nodes = vec![(0, 0); parse_node("ZZZ") as usize + 1];

    let starts: Vec<_> = lines.skip(1).filter_map(|line| {
        let pos = parse_node(&line[0..3]);
        nodes[pos as usize] = (parse_node(&line[7..10]), parse_node(&line[12..15]));
        if pos & 0x1F == 0 {
            Some(pos)
        } else {
            None
        }
    }).collect();

    let mut visited = HashMap::new();

    let mut cycles: Vec<_> = starts.into_iter().map(|start| {
        visited.clear();
        directions.chars().enumerate().cycle().enumerate().try_fold((start, Vec::new()), |
            (pos, mut offsets),
            (step, (step_in_cycle, direction)),
        | {
            visited.insert((pos, step_in_cycle), step as u16);
            let node = nodes[pos as usize];
            let new_pos = match direction {
                'L' => node.0,
                'R' => node.1,
                _ => panic!(),
            };
            if new_pos & 0x1F == 'Z' as u16 - 'A' as u16 {
                offsets.push(step as u16 + 1);
            }
            if let Some(&x) = visited.get(&(new_pos, step_in_cycle + 1)) {
                for i in &mut offsets {
                    *i -= x;
                }
                Err(Cycle {
                    period: step as u16 + 1 - x,
                    offset: x as u16,
                    goal_offsets: offsets,
                })
            } else {
                Ok((new_pos, offsets))
            }
        }).err().unwrap()
    }).collect();

    let starting_offset = cycles.iter().map(|cycle| cycle.offset).max().unwrap();
    for cycle in &mut cycles {
        let cycle_starting_offset = starting_offset - cycle.offset;
        let offset_in_cycle = (cycle_starting_offset % cycle.period) as u16;
        for goal_offset in &mut cycle.goal_offsets {
            *goal_offset = (*goal_offset as u16 + cycle.period as u16 - offset_in_cycle) % cycle.period as u16;
        }
        cycle.offset = 0;
    }

    let mut new_offsets = Vec::new();

    let full_cycle = cycles.into_iter().fold((1, vec![0]), |(acc_period, mut acc_offsets), cycle| {
        let mut pos = 0;
        let cycle_period = cycle.period as u64;
        new_offsets.clear();
        loop {
            for &i in &acc_offsets {
                if cycle.goal_offsets.contains(&(((pos + i) % cycle_period) as u16)) {
                    new_offsets.push(pos + i);
                }
            }
            pos += acc_period;
            if pos % cycle_period == 0 {
                std::mem::swap(&mut new_offsets, &mut acc_offsets);
                break (pos, acc_offsets);
            }
        }
    });

    starting_offset as u64 + full_cycle.1[0]
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
