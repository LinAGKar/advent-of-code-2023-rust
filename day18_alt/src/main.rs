use std::{io::Read, collections::BinaryHeap};

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Change {
    Start,
    Stop,
    None,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum RowState {
    Outside,
    Inside,
    StartBorder,
    StopBorder,
}

fn calc_area<F: Fn(&str) -> (Direction, i32)>(input: &str, parse_line: F) -> u64 {
    let (mut x, mut y) = (0, 0);
    let mut changes: BinaryHeap<_> = input.lines().filter_map(|line| {
        let (direction, distance) = parse_line(line);
        let old_y = y;
        match direction {
            Direction::Right => { x += distance; None },
            Direction::Down => { y -= distance; Some([(old_y, x, Change::Start), (y, x, Change::Stop)]) },
            Direction::Left => { x -= distance; None },
            Direction::Up => { y += distance; Some([(y, x, Change::Start), (old_y, x, Change::Stop)]) },
        }
    }).flatten().collect();

    let mut active_columns: Vec<_> = Vec::new();
    let mut row_active_columns: Vec<_> = Vec::new();
    let mut y = i32::MAX;
    let mut added_columns = Vec::new();
    let mut row_changes = Vec::new();
    let mut area = 0;

    while let Some(&(next_change_y, _, _)) = changes.peek() {
        let height = (y - 1 - next_change_y) as u64;
        let width: u64 = active_columns.chunks(2).map(|chunk| (chunk[1] - chunk[0] + 1) as u64).sum();
        area += height * width;

        while changes.peek().map_or(false, |&(y, _, _)| y == next_change_y) {
            let (_, x, change) = changes.pop().unwrap();
            row_changes.push((x, change));
            match change {
                Change::Start => { added_columns.push(x); }
                Change::Stop => { active_columns.remove(active_columns.iter().position(|&item| item == x).unwrap()); }
                Change::None => panic!(),
            }
        }

        row_changes.extend(active_columns.iter().map(|&x| (x, Change::None)));
        row_changes.sort();
        active_columns.extend(&added_columns);
        active_columns.sort();
        added_columns.clear();
        let mut state = RowState::Outside;
        row_active_columns.extend(row_changes.iter().filter_map(|&(x, change)| {
            let (new_state, include) = match (state, change) {
                (RowState::Outside, Change::None) => (RowState::Inside, true),
                (RowState::Outside, Change::Start) => (RowState::StartBorder, true),
                (RowState::Outside, Change::Stop) => (RowState::StopBorder, true),
                (RowState::Inside, Change::None) => (RowState::Outside, true),
                (RowState::Inside, Change::Start) => (RowState::StopBorder, false),
                (RowState::Inside, Change::Stop) => (RowState::StartBorder, false),
                (RowState::StartBorder, Change::Start) => (RowState::Outside, true),
                (RowState::StartBorder, Change::Stop) => (RowState::Inside, false),
                (RowState::StopBorder, Change::Start) => (RowState::Inside, false),
                (RowState::StopBorder, Change::Stop) => (RowState::Outside, true),
                (RowState::StartBorder | RowState::StopBorder, Change::None) => panic!(),
            };

            state = new_state;
            if include {
                Some(x)
            } else {
                None
            }
        }));
        row_changes.clear();
        area += row_active_columns.chunks(2).map(|chunk| (chunk[1] - chunk[0] + 1) as u64).sum::<u64>();
        row_active_columns.clear();
        y = next_change_y;
    }

    area
}

fn part_1(input: &str) -> u64 {
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

fn part_2(input: &str) -> u64 {
    calc_area(input, |line| {
        let instruction = i32::from_str_radix(line.split_ascii_whitespace().nth(2).unwrap().trim_matches(&['(', ')', '#'] as &[char]), 16).unwrap();
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
