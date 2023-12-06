use std::io::Read;

fn part_1(input: &str) -> u32 {
    input.lines().map(|line| {
        line.chars().find_map(|c| c.to_digit(10)).unwrap() * 10 +
        line.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
    }).sum()
}

const DIGITS: [[&str; 2]; 9] = [
    ["one", "1"], ["two", "2"], ["three", "3"], ["four", "4"], ["five", "5"], ["six", "6"], ["seven", "7"],
    ["eight", "8"], ["nine", "9"],
];

fn part_2(input: &str) -> usize {
    input.lines().map(|line| {
        DIGITS.iter().enumerate().flat_map(|(n, &digits)| {
            digits.into_iter().filter_map(move |digit| line.find(digit).map(|pos| (n + 1, pos)))
        }).min_by_key(|&(_, pos)| pos).unwrap().0 * 10 +
        DIGITS.iter().enumerate().flat_map(|(n, &digits)| {
            digits.into_iter().filter_map(move |digit| line.rfind(digit).map(|pos| (n + 1, pos)))
        }).max_by_key(|&(_, pos)| pos).unwrap().0
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
