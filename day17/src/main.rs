use std::{io::Read, collections::BinaryHeap, cmp::Reverse};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3,
    None = 4,
}

struct Tile {
    heat_loss: u8,
    // Need to keep track of scores separately in each orientation
    g_scores: [u16; 2],
}

fn best_path<const MIN_STEPS: usize, const MAX_STEPS: usize>(input: &str) -> u16 {
    let mut map: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().map(|c| Tile {
            heat_loss: c.to_digit(10).unwrap() as u8,
            g_scores: [u16::MAX; 2],
        }).collect()
    }).collect();

    const START: (usize, usize) = (0, 0);
    let goal = (map[0].len() - 1, map.len() - 1);

    let h = |(x, y)| ((goal.0 - x) + (goal.1 - y)) as u16;

    let initial_f_score = h(START);
    let mut frontier = BinaryHeap::new();
    frontier.push((Reverse(initial_f_score), 0, Direction::None, START));

    // Getting to start is free
    map[START.1][START.0].g_scores = [0; 2];

    while let Some((_, g_score, direction, pos)) = frontier.pop() {
        let (x, y) = pos;

        if g_score != map[y][x].g_scores[(direction as usize >> 1) & 0b1] {
            // We've found a better way to this tile, skip it
            continue;
        }

        if pos == goal {
            return g_score;
        }

        for new_direction in match direction {
            Direction::Up | Direction::Down => [Direction::Right, Direction::Left],
            Direction::Right | Direction::Left => [Direction::Up, Direction::Down],
            Direction::None => [Direction::Down, Direction::Right], // Special case for starting tile
        } {
            let mut tentative_g_score = g_score;
            for steps in 1..=MAX_STEPS {
                let (new_x, new_y) = match new_direction {
                    Direction::Up => (x, y.wrapping_sub(steps)),
                    Direction::Down => (x, y + steps),
                    Direction::Right => (x + steps, y),
                    Direction::Left => (x.wrapping_sub(steps), y),
                    Direction::None => panic!(),
                };

                if new_x >= map[0].len() || new_y >= map.len() {
                    // We went outside map
                    continue;
                }

                let neighbor = &mut map[new_y][new_x];
                tentative_g_score += neighbor.heat_loss as u16;

                if steps >= MIN_STEPS {
                    let old_g_score = neighbor.g_scores[new_direction as usize / 2];

                    if tentative_g_score < old_g_score {
                        // Found a better way to this position, in this direction
                        let f_score = tentative_g_score + h(pos);
                        neighbor.g_scores[new_direction as usize / 2] = tentative_g_score;
                        frontier.push((Reverse(f_score), tentative_g_score, new_direction, (new_x, new_y)))
                    }
                }
            }
        }
    }

    panic!()
}

fn part_1(input: &str) -> u16 {
    best_path::<1, 3>(input)
}

fn part_2(input: &str) -> u16 {
    best_path::<4, 10>(input)
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
