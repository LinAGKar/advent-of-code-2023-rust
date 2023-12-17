use std::io::Read;

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

struct BucketQueue<T> {
    items: Vec<Vec<T>>,
    first_filled: usize,
}

impl<T> BucketQueue<T> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            first_filled: usize::MAX,
        }
    }

    fn push(&mut self, priority: usize, item: T) {
        if priority >= self.items.len() {
            self.items.resize_with(priority + 1, || Vec::new());
        }

        self.items[priority].push(item);
        if priority < self.first_filled {
            self.first_filled = priority;
        }
    }

    fn pop_front(&mut self) -> Option<(usize, T)> {
        if self.first_filled >= self.items.len() {
            None
        } else {
            let item = self.items[self.first_filled].pop();
            let priority = self.first_filled;
            if self.items[self.first_filled].is_empty() {
                self.first_filled = self.items.iter().enumerate().skip(self.first_filled + 1).find_map(|(n, bucket)| {
                    if bucket.is_empty() {
                        None
                    } else {
                        Some(n)
                    }
                }).unwrap_or(usize::MAX)
            }
            item.map(|x| (priority, x))
        }
    }
}

fn best_path<const MIN_STEPS: usize, const MAX_STEPS: usize>(input: &str) -> u16 {
    let mut map: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().map(|c| Tile {
            heat_loss: c.to_digit(10).unwrap() as u8,
            g_scores: [u16::MAX; 2],
        }).collect()
    }).collect();

    const START: (u8, u8) = (0, 0);
    let goal = (map[0].len() - 1, map.len() - 1);

    let mut frontier = BucketQueue::new();
    frontier.push(0, (Direction::None, START));

    // Getting to start is free
    map[START.1 as usize][START.0 as usize].g_scores = [0; 2];

    while let Some((g_score, (direction, (x, y)))) = frontier.pop_front() {
        let g_score = g_score as u16;
        let (x, y) = (x as usize, y as usize);

        if g_score != map[y][x].g_scores[(direction as usize >> 1) & 0b1] {
            // We've found a better way to this tile, skip it
            continue;
        }

        if (x, y) == goal {
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
                        neighbor.g_scores[new_direction as usize / 2] = tentative_g_score;
                        frontier.push(tentative_g_score as usize, (new_direction, (new_x as u8, new_y as u8)))
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
