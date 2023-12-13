use std::io::Read;

fn sum_notes(input: &str, check: fn(items: &[u32], pos: usize) -> bool) -> u32 {
    // Reuse these so we don't need to reallocate for every pattern
    let mut lines = Vec::new();
    let mut columns = Vec::new();

    // Find mirror position, using passed callback to check if it's correct
    let find_index = |items: &[u32]| {
        (1..items.len()).find(|&i| check(&items, i))
    };

    input.split("\n\n").map(|pattern| {
        // Transform each line into a bitfield
        lines.clear();
        lines.extend(pattern.lines().map(|line| line.chars().fold(0, |acc, c| {
            (acc << 1) | (c == '#') as u32
        })));

        // First try to find horizontal mirror
        find_index(&lines).map_or_else(|| {
            // Didn't find one

            // Transpose map
            let width = pattern.lines().next().unwrap().chars().count();
            columns.clear();
            columns.extend((0..width).map(|x| {
                lines.iter().map(|&line| (line >> (width - x - 1)) & 0x1).fold(0, |acc, bit| {
                    (acc << 1) | bit
                })
            }));

            // Try to find vertical mirror
            find_index(&columns).unwrap()
        }, |x| x * 100) as u32
    }).sum()
}

fn part_1(input: &str) -> u32 {
    sum_notes(
        input,
        // Compare each item before mirror with its counterpart after mirror, checking if they're identical
        |items, i| items[..i].iter().rev().zip(&items[i..]).all(|(&a, &b)| a == b),
    )
}

fn part_2(input: &str) -> u32 {
    sum_notes(
        input,
        // Compare each item before mirror with its counterpart after mirror
        // There should be exactly one item that differs by one bit, all the rest should differ by zero bits
        |items, i| items[..i].iter().rev().zip(&items[i..]).try_fold(false, |found_smudge, (&a, &b)| {
            match (found_smudge, (a ^ b).count_ones()) {
                (_, 0) => Some(found_smudge), // Identical
                (false, 1) => Some(true),     // One bit differed
                (true, 1) |                   // One bit differed, but we'd already seen such a row/column
                    (_, 2..) => None,         // More than one bit differed
            }
        },
    ).unwrap_or(false))
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
