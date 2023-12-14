use std::{io::Read, collections::HashMap, boxed::Box};

fn part_1(input: &str) -> usize {
    let mut next_y = vec![0; input.lines().next().unwrap().chars().count()];
    let mut rock_count = 0;
    let mut total_y = 0;

    let height = input.lines().enumerate().inspect(|&(y, line)| {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => { next_y[x] = y + 1; }
                'O' => {
                    total_y += next_y[x];
                    rock_count += 1;
                    next_y[x] += 1;
                }
                _ => panic!(),
            }
        }
    }).count();

    height * rock_count - total_y
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Floor,
    Square,
    Round,
}

fn part_2(input: &str) -> usize {
    let mut map: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => Tile::Floor,
            '#' => Tile::Square,
            'O' => Tile::Round,
            _ => panic!(),
        }).collect()
    }).collect();

    let mut seen_at = HashMap::new();
    let mut next_y = Vec::<usize>::new();

    // Functions to get a tile using a rotated coordinate space
    let cycle_parts: &[(Box<dyn Fn(&mut Vec<Vec<Tile>>, usize, usize) -> &mut Tile>, _, _); 4] = &[
        (Box::new(|map: &mut Vec<Vec<Tile>>, x: usize, y: usize| &mut map[y][x]),
         map[0].len(), map.len()),
        (Box::new(|map: &mut Vec<Vec<Tile>>, x: usize, y: usize| &mut map[x][y]),
         map.len(), map[0].len()),
        (Box::new(|map: &mut Vec<Vec<Tile>>, x: usize, y: usize| { let h = map.len(); &mut map[h - 1 - y][x] }),
         map[0].len(), map.len()),
        (Box::new(|map: &mut Vec<Vec<Tile>>, x: usize, y: usize| { let w = map[0].len(); &mut map[x][w - 1 - y] }),
         map.len(), map[0].len()),
    ];

    let mut cycle = 0;
    const END: u32 = 1000000000;
    while cycle < END {
        // Handle tilts in each direction
        for (getter, width, height) in cycle_parts {
            next_y.clear();
            next_y.resize(*width, 0);
            for y in 0..*height {
                for x in 0..*width {
                    let item = getter(&mut map, x, y);
                    match *item {
                        Tile::Floor => {}
                        Tile::Square => {
                            next_y[x] = y + 1;
                        }
                        Tile::Round => {
                            *item = Tile::Floor;
                            *getter(&mut map, x, next_y[x]) = Tile::Round;
                            next_y[x] += 1;
                        }
                    }
                }
            }
        }

        // More compact representation of the current state, for saving in hashmap
        let key = map.iter().enumerate().flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, &tile)| {
                if tile == Tile::Round {
                    Some((x as u8, y as u8))
                } else {
                    None
                }
            })
        }).collect::<Vec<_>>();

        cycle += 1;
        if let Some(seen_at_cycle) = seen_at.insert(key, cycle) {
            // Current state was identical to one we'd already seen, we can skip forward
            let diff = cycle - seen_at_cycle;
            let remaining = END - cycle;
            let skipped = remaining / diff * diff;
            cycle += skipped;
        }
    }

    let height = map.len();

    map.into_iter().enumerate().flat_map(|(y, line)| line.into_iter().filter_map(move |tile| {
        if tile == Tile::Round {
            Some(height - y)
        } else {
            None
        }
    })).sum()
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
