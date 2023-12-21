use std::{io::Read, collections::{VecDeque, HashSet}};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Floor,
    Wall,
    Visited,
}

fn part_1(input: &str) -> u32 {
    let mut start = 0;
    let mut map: Vec<_> = input.chars().filter(|&c| c != '\n').enumerate().map(|(pos, c)| match c {
        '.' => Tile::Floor,
        '#' => Tile::Wall,
        'S' => {
            start = pos;
            Tile::Visited
        }
        _ => panic!(),
    }).collect();
    let width = input.lines().next().unwrap().chars().count();

    let height = map.len() / width;
    let mut open_set: VecDeque<_> = [(0, start / width, start % width)].into_iter().collect();
    let mut count = 0;
    while let Some((steps, y, x)) = open_set.pop_front() {
        if steps % 2 == 0 {
            count += 1;
        }
        if steps >= 64 {
            continue;
        }
        let new_steps = steps + 1;
        for (new_y, new_x) in [
            (y + 1, x),
            (y, x + 1),
            (y.wrapping_sub(1), x),
            (y, x.wrapping_sub(1)),
        ] {
            if new_y >= height || new_x >= width {
                continue;
            }
            let new_pos = new_y * width + new_x;
            if map[new_pos] != Tile::Floor {
                continue;
            }
            map[new_pos] = Tile::Visited;
            open_set.push_back((new_steps, new_y, new_x));
        }
    }

    count
}

const PART_2_STEPS: usize = 26501365;

fn _part_2_brute_force(input: &str) -> usize {
    let mut start = 0;
    let map: Vec<_> = input.chars().filter(|&c| c != '\n').enumerate().map(|(pos, c)| match c {
        '.' => Tile::Floor,
        '#' => Tile::Wall,
        'S' => {
            start = pos;
            Tile::Floor
        }
        _ => panic!(),
    }).collect();
    let width = input.lines().next().unwrap().chars().count();
    let height = map.len() / width;
    let (start_y, start_x) = ((start / width) as isize, (start % width) as isize);
    let mut visited: HashSet<_> = [(start_y, start_x)].into_iter().collect();
    let mut open_set: VecDeque<_> = [(0, start_y, start_x)].into_iter().collect();
    let mut count = 0;
    while let Some((steps, y, x)) = open_set.pop_front() {
        if steps % 2 == PART_2_STEPS % 2 {
            count += 1;
        }
        if steps >= PART_2_STEPS {
            continue;
        }
        let new_steps = steps + 1;
        for (new_y, new_x) in [
            (y + 1, x),
            (y, x + 1),
            (y - 1, x),
            (y, x - 1),
        ] {
            let new_pos = new_y.rem_euclid(height as isize) as usize * width + new_x.rem_euclid(width as isize) as usize;
            if map[new_pos] != Tile::Floor || visited.contains(&(new_y, new_x)) {
                continue;
            }
            visited.insert((new_y, new_x));
            open_set.push_back((new_steps, new_y, new_x));
        }
    }

    count
}

fn part_2(input: &str) -> usize {
    let mut start = 0;
    let mut map: Vec<_> = input.chars().filter(|&c| c != '\n').enumerate().map(|(pos, c)| match c {
        '.' => Tile::Floor,
        '#' => Tile::Wall,
        'S' => {
            start = pos;
            Tile::Visited
        }
        _ => panic!(),
    }).collect();
    let width = input.lines().next().unwrap().chars().count();
    let height = map.len() / width;
    assert!(width == height && width % 2 == 1 && (PART_2_STEPS - width / 2) % width == 0);
    let mut open_set: VecDeque<_> = [(0, start / width, start % width)].into_iter().collect();
    let mut center_even = 0;
    let mut center_odd = 0;
    let mut corner_tiles = 0;
    while let Some((steps, y, x)) = open_set.pop_front() {
        *match (steps > width / 2, steps % 2 == 0) {
            (false, false) => &mut center_odd,
            (false, true) => &mut center_even,
            (true, _) => &mut corner_tiles,
        } += 1;
        let new_steps = steps + 1;
        for (new_y, new_x) in [
            (y + 1, x),
            (y, x + 1),
            (y.wrapping_sub(1), x),
            (y, x.wrapping_sub(1)),
        ] {
            if new_y >= height || new_x >= width {
                continue;
            }
            let new_pos = new_y * width + new_x;
            if map[new_pos] != Tile::Floor {
                continue;
            }
            map[new_pos] = Tile::Visited;
            open_set.push_back((new_steps, new_y, new_x));
        }
    }

    let radius = (PART_2_STEPS - width / 2) / width;
    let center_even_diamonds = (1 + radius / 2 * 2).pow(2);
    let center_odd_diamonds = ((1 + radius) / 2 * 2).pow(2);
    let corner_diamonds = (radius * 2 + 1).pow(2) / 4;
    center_even_diamonds * if PART_2_STEPS % 2 == 0 { center_even } else { center_odd } +
        center_odd_diamonds * if PART_2_STEPS % 2 == 0 { center_odd } else { center_even } +
        corner_diamonds * corner_tiles
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
