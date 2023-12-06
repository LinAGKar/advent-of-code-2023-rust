use std::io::Read;

fn get_matches<'a>(line: &'a str, winning: &mut Vec<&'a str>) -> usize {
    let mut words = line.split_whitespace().skip(2);
    winning.clear();
    while let Some(x) = words.next() {
        if x == "|" {
            break;
        }

        winning.push(x);
    }

    words.filter(|x| winning.contains(x)).count()

}

fn part_1(input: &str) -> u32 {
    let mut winning = Vec::new();

    input.lines().map(|line| {
        let matches = get_matches(line, &mut winning);
        if matches == 0 {
            0
        } else {
            2_u32.pow((matches - 1) as u32)
        }
    }).sum()
}

fn part_2(input: &str) -> u32 {
    let mut card_counts = vec![1u32];
    let mut count = 0;
    let mut winning = Vec::new();

    for (n, line) in input.lines().enumerate() {
        let end = n + get_matches(line, &mut winning) + 1;

        if end > card_counts.len() {
            card_counts.resize(end, 1);
        }

        for i in n + 1..end {
            card_counts[i] += card_counts[n];
        }

        count += card_counts[n];
    }

    count
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
