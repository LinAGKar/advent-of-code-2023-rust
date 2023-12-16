use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    MirrorUR,
    MirrorUL,
    SplitterVert,
    SplitterHoriz,
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BeamDir {
    Right = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Up = 0b1000,
}

fn parse_map(input: &str) -> Vec<Vec<(Tile, u8)>> {
    input.lines().map(|line| {
        line.chars().map(|c| (match c {
            '\\' => Tile::MirrorUR,
            '/' => Tile::MirrorUL,
            '|' => Tile::SplitterVert,
            '-' => Tile::SplitterHoriz,
            '.' => Tile::Space,
            _ => panic!(),
        }, 0 /* Bitfield with directions of light beams entering tile */)).collect()
    }).collect()
}

fn energized_count(map: &mut Vec<Vec<(Tile, u8)>>, start: (BeamDir, usize, usize)) -> u16 {
    let mut beams = vec![start];
    let mut new_directions = Vec::with_capacity(2);
    let mut energized = 0;

    while let Some((direction, x, y)) = beams.pop() {
        let (tile, directions) = &mut map[y][x];

        if *directions & direction as u8 != 0 {
            // Light has already entered tile in this direction
            continue;
        }

        if *directions == 0 {
            // No light has entered this tile before
            energized += 1;
        }
        *directions |= direction as u8;

        // Calculate directions of light exiting this tile
        match *tile {
            Tile::MirrorUR => new_directions.push(match direction {
                BeamDir::Right => BeamDir::Down,
                BeamDir::Down => BeamDir::Right,
                BeamDir::Left => BeamDir::Up,
                BeamDir::Up => BeamDir::Left,
            }),

            Tile::MirrorUL => new_directions.push(match direction {
                BeamDir::Right => BeamDir::Up,
                BeamDir::Down => BeamDir::Left,
                BeamDir::Left => BeamDir::Down,
                BeamDir::Up => BeamDir::Right,
            }),

            Tile::SplitterVert => if direction as u8 & 0b1010 != 0 {
                new_directions.push(direction);
            } else {
                new_directions.extend(&[BeamDir::Up, BeamDir::Down]);
            }

            Tile::SplitterHoriz => if direction as u8 & 0b0101 != 0 {
                new_directions.push(direction);
            } else {
                new_directions.extend(&[BeamDir::Left, BeamDir::Right]);
            }

            Tile::Space => { new_directions.push(direction); }
        }

        for &new_direction in &new_directions {
            let (new_x, new_y) = match new_direction {
                BeamDir::Right => (x + 1, y),
                BeamDir::Down => (x, y + 1),
                BeamDir::Left => (x.wrapping_sub(1), y),
                BeamDir::Up => (x, y.wrapping_sub(1)),
            };

            if new_y >= map.len() || new_x >= map[new_y].len() {
                // Went outside map
                continue;
            }

            beams.push((new_direction, new_x, new_y));
        }

        new_directions.clear();
    }

    energized

}

fn part_1(input: &str) -> u16 {
    let mut map = parse_map(input);
    energized_count(&mut map, (BeamDir::Right, 0, 0))
}

fn part_2(input: &str) -> u16 {
    let mut map = parse_map(input);
    let height = map.len();
    let width = map[0].len();

    // Enter from every outer edge
    (0..height).flat_map(|y| {
        [(BeamDir::Right, 0, y), (BeamDir::Left, width - 1, y)].into_iter()
    }).chain((0..width).flat_map(|x| {
        [(BeamDir::Down, x, 0), (BeamDir::Up, x, height - 1)].into_iter()
    })).map(|start| {
        // Clean up from previous run
        for line in &mut map {
            for (_, directions) in line {
                *directions = 0;
            }
        }

        energized_count(&mut map, start)
    }).max().unwrap()
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
