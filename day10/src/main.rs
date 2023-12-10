use std::{io::Read, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn index(&self, width: isize) -> usize {
        width as usize * self.y as usize + self.x as usize
    }
}

// Take map, the width of the map, and a point, and return a reference to the tile in the map a that point
fn get_tile<'a>(map: &'a [[Point; 2]], pos: Point, width: isize) -> Option<&'a [Point; 2]> {
    if pos.y < 0 || pos.x < 0 || pos.x >= width {
        None
    } else {
        let index = pos.index(width);
        if index >= map.len() {
            None
        } else {
            Some(&map[index])
        }
    }
}


fn find_loop<F: FnMut(&[[Point; 2]], Point, isize)>(input: &str, mut pipe_callback: F) {
    let width = input.lines().next().unwrap().chars().count() as isize;

    // Build up a map of the area as a flat vector, which each tile containing the offsets to the tiles it's connected
    // to. Save starting position when we find it.
    let mut start = Point{x: 0, y: 0};
    let mut map: Vec<_> = input.chars().enumerate().filter_map(|(n, c)| {
        match c {
            '\n' => None,
            '.' => Some([Point{x: 0, y: 0}; 2]),
            '|' => Some([Point{x: 0, y: -1}, Point{x: 0, y: 1}]),
            '-' => Some([Point{x: -1, y: 0}, Point{x: 1, y: 0}]),
            'L' => Some([Point{x: 0, y: -1}, Point{x: 1, y: 0}]),
            'J' => Some([Point{x: -1, y: 0}, Point{x: 0, y: -1}]),
            '7' => Some([Point{x: -1, y: 0}, Point{x: 0, y: 1}]),
            'F' => Some([Point{x: 1, y: 0}, Point{x: 0, y: 1}]),
            'S' => {
                start = Point{x: n as isize % (width + 1), y: n as isize / (width + 1)};
                Some([Point{x: 0, y: 0}; 2])
            },
            _ => panic!(),
        }
    }).collect();

    // Calculate what directions starting tile connects to, by checking each tile around it to see if it connects back
    let mut x = 0;
    for diff in [
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
    ] {
        let new_pos = start + diff;
        if let Some(tile) = get_tile(&map, new_pos, width) {
            if tile.iter().any(|&diff| new_pos + diff == start) {
                map[start.index(width)][x] = diff;
                x += 1;
            }
        }
    }

    // Pick one arbitrary direction from start, and walk through the whole loop, calling a callback for each step
    // Initial value for came_from does not matter
    let mut came_from = start;
    let mut pos = start;
    loop {
        pipe_callback(&map, pos, width);

        // Check both points current tile connects to, and take the one we didn't just come from
        let new_pos = get_tile(&map, pos, width).unwrap().iter().find_map(|&diff| {
            let new_pos = pos + diff;
            if new_pos == came_from {
                None
            } else {
                Some(new_pos)
            }
        }).unwrap();

        if new_pos == start {
            break;
        }

        came_from = pos;
        pos = new_pos;
    }
}

fn part_1(input: &str) -> u16 {
    // Count how many steps we take to get through the loop
    let mut steps = 0;
    find_loop(input, |_map, _pos, _width| {
        steps += 1;
    });

    // Farthest point is halfway through the loop
    steps / 2
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Unknown,
    Pipe,
    Outside,
}

fn part_2(input: &str) -> usize {
    // Map with some padding around each tile, so we can squeeze between tiles
    let mut tile_map = Vec::new();
    let mut tile_map_width = 0;
    let mut tile_map_height = 0;
    let mut map_size = 0;

    // Count how many tiles loop takes up
    let mut pipe_count = 0;

    find_loop(input, |map, pos, width| {
        pipe_count += 1;

        if tile_map.len() == 0 {
            let height = map.len() / width as usize;
            tile_map = vec![Tile::Unknown; (height * 2 + 1) * (width as usize * 2 + 1)];
            tile_map_width = width * 2 + 1;
            tile_map_height = (tile_map.len() / tile_map_width as usize) as isize;
            map_size = map.len();
        }

        let tile_pos = pos.x as usize * 2 + 1 + tile_map_width as usize * (pos.y as usize * 2 + 1);
        tile_map[tile_pos] = Tile::Pipe;

        // Connect to pipes below and to right (pipes above and to left will connect to us)
        let pipe = get_tile(map, pos, width).unwrap();
        if pipe.contains(&Point { x: 0, y: 1}) {
            tile_map[tile_pos + tile_map_width as usize] = Tile::Pipe;
        }
        if pipe.contains(&Point { x: 1, y: 0}) {
            tile_map[tile_pos + 1] = Tile::Pipe;
        }
    });

    // Start from some point guaranteed to be outside loop, and cover the whole thing, in arbitrary order, to see how
    // much is outside
    let mut frontier = vec![Point { x: 0, y: 0 }];
    let mut outside_count = 0;
    while let Some(pos) = frontier.pop() {
        for diff in [
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: 1 },
        ] {
            let new_pos = pos + diff;
            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= tile_map_width || new_pos.y >= tile_map_height {
                continue;
            }

            let tile = &mut tile_map[new_pos.x as usize + new_pos.y as usize * tile_map_width as usize];

            // Check that this isn't a pipe, and that we haven't visited this already
            if *tile != Tile::Unknown {
                continue;
            }

            *tile = Tile::Outside;
            frontier.push(new_pos);
            // If this is not a padding tile, increment
            if new_pos.x % 2 == 1 && new_pos.y % 2 == 1 {
                outside_count += 1;
            }
        }
    }

    map_size - outside_count - pipe_count
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
