use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Spring {
    Broken,
    Working,
    Unknown,
}

fn arrangements(input: &str, repetitions: usize) -> u64 {
    let mut springs_single_rep = Vec::new();
    let mut springs = Vec::<Spring>::new();
    let mut groups_single_rep = Vec::new();
    let mut groups = Vec::<usize>::new();
    let mut stack = Vec::new();
    let mut cache = Vec::new();

    input.lines().map(|line| {
        springs_single_rep.clear();
        springs.clear();
        groups_single_rep.clear();
        groups.clear();
        stack.clear();
        cache.clear();

        let mut parts = line.split_ascii_whitespace();

        springs_single_rep.extend(parts.next().unwrap().chars().map(|c| match c {
            '#' => Spring::Broken,
            '.' => Spring::Working,
            '?' => Spring::Unknown,
            _ => panic!(),
        }));

        groups_single_rep.extend(parts.next().unwrap().split(',').map(|num| num.parse::<usize>().unwrap()));

        springs.reserve((springs_single_rep.len() + 1 * repetitions) - 1);
        groups.reserve(groups_single_rep.len() * repetitions);
        for _ in 0..repetitions {
            if springs.len() > 0 {
                springs.push(Spring::Unknown);
            }
            springs.extend(&springs_single_rep);
            groups.extend(&groups_single_rep);
        }

        cache.resize((groups.len() - 1) * springs.len(), None);
        stack.reserve(groups.len() - 1);

        let mut count = 0;
        let mut pos = 0;

        loop {
            let mut indent = String::new();
            for _ in 0..stack.len() {
                indent.extend("  ".chars());
            }
            let len = groups[stack.len()];
            let end = pos + len;

            if end > springs.len() || (pos > 0 && springs[pos - 1] == Spring::Broken) {
                // There's a broken spring that's not included in a group, or we've gone past the end
                if let Some((x, y)) = stack.pop() {
                    pos = x;
                    cache[stack.len() * springs.len() + pos] = Some(count);
                    count = y + count;
                    pos += 1;
                    continue;
                } else {
                    break;
                }
            }

            if (end < springs.len() && springs[end] == Spring::Broken) ||
                springs[pos..end].iter().any(|&x| x == Spring::Working) {
                // Not a valid position
                pos += 1;
                continue;
            }

            if stack.len() == groups.len() - 1 {
                if springs[end..].iter().all(|&x| x != Spring::Broken) {
                    count += 1;
                }
                pos += 1;
            } else {
                if let Some(old) = cache[stack.len() * springs.len() + pos] {
                    count += old;
                    pos += 1;
                } else {
                    stack.push((pos, count));
                    count = 0;
                    pos = end + 1;
                }
            }
        }

        count
    }).sum()
}

fn part_1(input: &str) -> u64 {
    arrangements(input, 1)
}

fn part_2(input: &str) -> u64 {
    arrangements(input, 5)
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
