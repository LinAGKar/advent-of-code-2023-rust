use std::collections::HashMap;
use std::io::Read;

fn part_1(input: &str) -> usize {
    let colors: HashMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)].into_iter().collect();

    input.lines().enumerate().filter_map(|(n, line)| {
        let mut iter = line.split_ascii_whitespace().skip(2);
        while let Some(x) = iter.next() {
            let count: u32 = x.parse().unwrap();
            let color = iter.next().unwrap().trim_end_matches([',', ';']);
            if count > colors[color] {
                return None;
            }
        }
        Some(n + 1)
    }).sum()
}

fn part_2(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut colors: HashMap<_, _> = [("red", 0), ("green", 0), ("blue", 0)].into_iter().collect();
        let mut iter = line.split_ascii_whitespace().skip(2);
        while let Some(x) = iter.next() {
            let count: u32 = x.parse().unwrap();
            let color = iter.next().unwrap().trim_end_matches([',', ';']);
            let max = colors.get_mut(color).unwrap();
            if count > *max {
                *max = count;
            }
        }
        colors.into_iter().map(|(_, count)| count).product::<u32>()
    }).sum()
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
