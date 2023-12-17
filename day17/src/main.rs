use std::{io::Read, collections::BinaryHeap, cmp::Reverse};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3,
}

struct Tile<const MAX_STEPS: usize> {
    heat_loss: u8,
    // Need to keep track of scores separately for each number of steps in each orientation
    g_scores: [[u16; MAX_STEPS]; 2],
}

fn best_path<const MIN_STEPS: usize, const MAX_STEPS: usize>(input: &str) -> u16 {
    let mut map: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().map(|c| Tile {
            heat_loss: c.to_digit(10).unwrap() as u8,
            g_scores: [[u16::MAX; MAX_STEPS]; 2],
        }).collect()
    }).collect();

    const START: (usize, usize) = (0, 0);
    let goal = (map[0].len() - 1, map.len() - 1);

    let h = |(x, y)| ((goal.0 - x) + (goal.1 - y)) as u16;

    let initial_f_score = h(START);
    let mut frontier = BinaryHeap::new();
    frontier.push((Reverse(initial_f_score), 0, Direction::Down, 0, START));

    // Getting to start is free
    map[START.1][START.0].g_scores = [[0; MAX_STEPS]; 2];

    while let Some((_, g_score, direction, steps, pos)) = frontier.pop() {
        let (x, y) = pos;
        let step_index = if steps == 0 {
            // Special case for starting position. After this, steps will always be at least 1
            0
        } else {
            steps - 1
        };

        if g_score != map[y][x].g_scores[direction as usize / 2][step_index] {
            // We've found a better way to this tile, skip it
            continue;
        }

        if pos == goal && steps >= MIN_STEPS {
            return g_score;
        }

        for (new_x, new_y, new_direction) in [
            (x + 1, y, Direction::Right),
            (x, y + 1, Direction::Down),
            (x.wrapping_sub(1), y, Direction::Left),
            (x, y.wrapping_sub(1), Direction::Up),
        ] {
            if new_x >= map[0].len() || new_y >= map.len() {
                // We went outside map
                continue;
            }

            let new_steps = if new_direction == direction {
                // Continuing in the same direction
                steps + 1
            } else if (new_direction <= Direction::Down) == (direction <= Direction::Down) {
                // 180 degree turn
                continue;
            } else if steps > 0 && steps < MIN_STEPS {
                // We haven't gone far enough to turn yet
                continue;
            } else {
                // 90 degree turn
                1
            };

            if new_steps > MAX_STEPS {
                // Gone too far in one direction
                continue;
            }

            let neighbor = &mut map[new_y][new_x];
            let tentative_g_score = g_score + neighbor.heat_loss as u16;
            let old_g_score = neighbor.g_scores[new_direction as usize / 2][new_steps - 1];

            if tentative_g_score < old_g_score {
                // Found a better way to this position, for this number of steps in this direction
                let f_score = tentative_g_score + h(pos);

                // No point in visiting this tile again with the same g_score and a higher number of steps in the
                // same direction.
                for i in new_steps - 1..MAX_STEPS {
                    let neighbor_g_score = &mut neighbor.g_scores[new_direction as usize / 2][i];
                    if tentative_g_score < *neighbor_g_score {
                        *neighbor_g_score = tentative_g_score;
                    }

                    // Optimization above doesn't work if we have a minimum number of steps we need to go before
                    // turning/stopping.
                    if MIN_STEPS > 1 {
                        break;
                    }
                }
                frontier.push((Reverse(f_score), tentative_g_score, new_direction, new_steps, (new_x, new_y)))
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
