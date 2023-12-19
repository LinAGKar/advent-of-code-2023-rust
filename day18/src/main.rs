use std::io::Read;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn calc_area<F: Fn(&str) -> (Direction, i64)>(input: &str, parse_line: F) -> i64 {
    let (_, _, circumference, area) = input.lines().fold((0, 0, 0, 0), |(x, y, circumference, area), line| {
        let (direction, distance) = parse_line(line);
        let (new_x, new_y) = match direction {
            Direction::Right => (x + distance, y),
            Direction::Down => (x, y - distance),
            Direction::Left => (x - distance, y),
            Direction::Up => (x, y + distance),
        };

        (new_x, new_y, circumference + distance, area + (y + new_y) * (x - new_x))
    });

    area.abs() / 2 + circumference / 2 + 1
}

fn part_1(input: &str) -> i64 {
    calc_area(input, |line| {
        let mut words = line.split_ascii_whitespace();
        let direction = words.next().unwrap();
        let distance = words.next().unwrap().parse().unwrap();
        (
            match direction {
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "U" => Direction::Up,
                _ => panic!(),
            },
            distance,
        )
    })
}

fn part_2(input: &str) -> i64 {
    calc_area(input, |line| {
        let instruction = i64::from_str_radix(line.split_ascii_whitespace().nth(2).unwrap().trim_matches(&['(', ')', '#'] as &[char]), 16).unwrap();
        let distance = instruction >> 4;
        let direction = instruction & 0x3;
        (
            match direction {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                _ => panic!(),
            },
            distance,
        )
    })
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
