use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug, Clone)]
struct Node {
    edges: Vec<(u8, u16)>,
}

fn increment_pos([x, y]: [u8; 2], direction: Direction) -> [u8; 2] {
    match direction {
        Direction::Up => [x, y - 1],
        Direction::Right => [x + 1, y],
        Direction::Down => [x, y + 1],
        Direction::Left => [x - 1, y],
    }
}

fn invert_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
    }
}

fn direction_bit(direction: Direction) -> u8 {
    1 << direction as u8
}

fn longest_path(graph: &Vec<Node>, current_node: u8, goal: u8, visited: &mut Vec<u8>) -> Option<u16> {
    graph[current_node as usize].edges.iter().filter_map(|&(next_node, length)| {
        if next_node == goal {
            Some(length)
        } else if visited.contains(&next_node) {
            None
        } else {
            visited.push(next_node);
            let result = longest_path(graph, next_node, goal, visited);
            visited.pop();
            result.map(|x| x + length)
        }
    }).max()
}

fn find_longest_path(input: &str, slippery: bool) -> u16 {
    let width = input.lines().next().unwrap().chars().count();
    let map: Vec<_> = input.chars().filter_map(|c| {
        match (c, slippery) {
            ('\n', _) => None,
            ('.', _) => Some(Tile::Path),
            ('#', _) => Some(Tile::Forest),
            ('^', true) => Some(Tile::Slope(Direction::Up)),
            ('>', true) => Some(Tile::Slope(Direction::Right)),
            ('v', true) => Some(Tile::Slope(Direction::Down)),
            ('<', true) => Some(Tile::Slope(Direction::Left)),
            ('^' | '>' | 'v' | '<', false) => Some(Tile::Path),
            _ => panic!(),
        }
    }).collect();

    let mut node_map = vec![None; map.len()];
    node_map[1] = Some(0);
    node_map[map.len() - 2] = Some(1);
    let start = [1, 0];

    let pos_to_index = move |[x, y]: [u8; 2]| x as usize + y as usize * width;

    let mut graph = vec![Node { edges: Vec::new() }; 2];
    let mut directions_exited = vec![0; 2];
    let mut open_set = vec![(0u8, start, Direction::Down)];
    let mut next_steps = Vec::new();

    while let Some((start_node, start_pos, start_direction)) = open_set.pop() {
        if directions_exited[start_node as usize] & direction_bit(start_direction) != 0 {
            continue;
        }
        let mut forward_possible = true;
        let mut backward_possible = true;
        let mut pos = increment_pos(start_pos, start_direction);
        let mut direction = start_direction;
        let mut index = pos_to_index(pos);
        let mut length = 1;

        loop {
            if let Tile::Slope(slope_direction) = map[index] {
                if slope_direction == direction {
                    backward_possible = false;
                } else if slope_direction == invert_direction(direction) {
                    forward_possible = false;
                }
            }

            if let Some(end_node) = node_map[index] {
                directions_exited[start_node as usize] |= direction_bit(start_direction);
                directions_exited[end_node as usize] |= direction_bit(invert_direction(direction));
                if forward_possible {
                    graph[start_node as usize].edges.push((end_node, length));
                }
                if backward_possible {
                    graph[end_node as usize].edges.push((start_node, length));
                }
                break;
            }

            for new_direction in [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ] {
                if invert_direction(new_direction) == direction {
                    continue;
                }
                let new_pos = increment_pos(pos, new_direction);
                let new_index = pos_to_index(new_pos);
                if map[new_index] == Tile::Forest {
                    continue;
                }
                next_steps.push((new_index, new_pos, new_direction));
            }

            if next_steps.len() == 1 {
                let (new_index, new_pos, new_direction) = next_steps.pop().unwrap();
                index = new_index;
                pos = new_pos;
                direction = new_direction;
                length += 1;
            } else {
                let end_node = graph.len() as u8;
                node_map[index] = Some(end_node);
                graph.push(Node { edges: Vec::new() });
                directions_exited.push(0);
                directions_exited[start_node as usize] |= direction_bit(start_direction);
                directions_exited[end_node as usize] |= direction_bit(invert_direction(direction));
                if forward_possible {
                    graph[start_node as usize].edges.push((end_node, length));
                }
                if backward_possible {
                    graph[end_node as usize].edges.push((start_node, length));
                }

                open_set.extend(next_steps.iter().map(|&(_, _, direction)| {
                    (end_node, pos, direction)
                }));
                next_steps.clear();
                break;
            }
        }
    }

    longest_path(&graph, 0, 1, &mut Vec::new()).unwrap()
}

fn part_1(input: &str) -> u16 {
    find_longest_path(input, true)
}

fn part_2(input: &str) -> u16 {
    find_longest_path(input, false)
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
