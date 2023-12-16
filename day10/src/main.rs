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

fn find_loop<F: FnMut(&[[Point; 2]], Point, Point, isize)>(input: &str, mut pipe_callback: F) {
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
    let mut came_from = start + get_tile(&map, start, width).unwrap()[0];
    let mut pos = start;
    loop {
        pipe_callback(&map, came_from, pos, width);

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
    find_loop(input, |_map, _came_from, _pos, _width| {
        steps += 1;
    });

    // Farthest point is halfway through the loop
    steps / 2
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile2 {
    Pipe(i8),
    Floor,
}

fn part_2(input: &str) -> usize {
    let mut tile_map = Vec::new();
    let mut map_width = 0;
    let mut map_height = 0;

    find_loop(input, |map, came_from, pos, width| {
        if tile_map.len() == 0 {
            tile_map = vec![Tile2::Floor; map.len()];
            map_width = width as usize;
            map_height = tile_map.len() / map_width;
        }

        if tile_map[pos.y as usize * width as usize + pos.x as usize] == Tile2::Floor {
            tile_map[pos.y as usize * width as usize + pos.x as usize] = Tile2::Pipe(0);
        }
        if tile_map[came_from.y as usize * width as usize + came_from.x as usize] == Tile2::Floor {
            tile_map[came_from.y as usize * width as usize + came_from.x as usize] = Tile2::Pipe(0);
        }
        if pos.y > came_from.y {
            if let Tile2::Pipe(mov) = &mut tile_map[pos.y as usize * width as usize + pos.x as usize] {
                *mov += 1;
            }
            if let Tile2::Pipe(mov) = &mut tile_map[came_from.y as usize * width as usize + came_from.x as usize] {
                *mov += 1;
            }
        } else if pos.y < came_from.y {
            if let Tile2::Pipe(mov) = &mut tile_map[pos.y as usize * width as usize + pos.x as usize] {
                *mov -= 1;
            }
            if let Tile2::Pipe(mov) = &mut tile_map[came_from.y as usize * width as usize + came_from.x as usize] {
                *mov -= 1;
            }
        }
    });

    // for y in
    (0..map_height).map(|y| {
        let tile_map = &tile_map;
        (0..map_width).fold((0, 0), move |(mov_state, count), x| {
            // The pipe has two sides, one on the inside and one of the outside. So when we cross the pipe, we go from
            // the inside to the outside or vice versa.
            match (tile_map[y * map_width + x], mov_state) {
                (Tile2::Floor, 0) => (mov_state, count),
                (Tile2::Floor, _) => (mov_state, count + 1),
                (Tile2::Pipe(mov), _) => (mov_state + mov, count),
            }
        }).1
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
