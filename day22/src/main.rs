use std::io::Read;

trait SupportCallbacks {
    fn brick_count(&mut self, count: usize);
    fn supported_by(&mut self, index: u16);
    fn brick_done(&mut self, index: u16);
}

fn calculate_supports<T: SupportCallbacks>(input: &str, callbacks: &mut T) {
    let mut bricks: Vec<_> = input.lines().map(|line: &str| {
        let mut brick = [[0u16; 3]; 2];
        for (src, dst) in line.split('~').zip(&mut brick) {
            for (src, dst) in src.split(',').zip(dst) {
                *dst = src.parse().unwrap();
            }
        }
        brick
    }).collect();

    callbacks.brick_count(bricks.len());

    bricks.sort_by_key(|[start, _]| start[2]);
    let mut size = [0; 3];
    for [_, end] in &bricks {
        for (&src, dst) in end.iter().zip(&mut size) {
            *dst = std::cmp::max(src as usize + 1, *dst);
        }
    }

    let mut map = vec![None; size.iter().copied().product::<usize>()];
    let width = size[0];
    let base_area: usize = size.iter().take(2).product();
    let pos_to_index = |[x, y, z]: [u16; 3]|x as usize + y as usize * width + z as usize * base_area;

    for (n, [start, end]) in bricks.iter().enumerate() {
        // Find height brick will come to rest on top of
        let z = (1..=start[2]).rev().find(|&z| {
            (start[0]..=end[0]).flat_map(|x| (start[1]..=end[1]).map(move |y| [x, y])).any(|[x, y]| {
                map[pos_to_index([x, y, z])].is_some()
            })
        }).unwrap_or(0);

        let height = end[2] + 1 - start[2];

        let mut prev_supported_by = u16::MAX;
        for [x, y] in (start[0]..=end[0])
                .flat_map(|x| (start[1]..=end[1]).map(move |y| [x, y])) {
            let mut index = pos_to_index([x, y, z]);

            if let Some(brick_index) = map[index] {
                if brick_index != prev_supported_by {
                    prev_supported_by = brick_index;
                    callbacks.supported_by(brick_index);
                }
            }
            for _ in 0..height {
                index += base_area;
                map[index] = Some(n as u16);
            }
        }

        callbacks.brick_done(n as u16);
    }
}

struct Part1State {
    removable: Vec<bool>,
    supported_by: Option<u16>,
}

impl SupportCallbacks for Part1State {
    fn brick_count(&mut self, count: usize) {
        self.removable.resize(count, true);
    }

    fn supported_by(&mut self, index: u16) {
        self.supported_by = match self.supported_by {
            None => Some(index),
            Some(prev_index) => Some(if prev_index == index { index } else { u16::MAX }),
        }
    }

    fn brick_done(&mut self, _: u16) {
        match self.supported_by {
            None | Some(u16::MAX) => {}
            Some(index) => { self.removable[index as usize] = false; }
        }
        self.supported_by = None;
    }
}

fn part_1(input: &str) -> usize {
    let mut state = Part1State {
        removable: Vec::new(),
        supported_by: None,
    };
    calculate_supports(input, &mut state);
    state.removable.into_iter().filter(|&x| x).count()
}

#[derive(Debug, Clone)]
struct Node {
    supported_by: Vec<u16>,
    total_supports: u16,
    partly_supports: Vec<u16>,
    forks: u8,
}

struct Part2State {
    graph: Vec<Node>,
    supported_by: Vec<u16>,
}

impl SupportCallbacks for Part2State {
    fn brick_count(&mut self, count: usize) {
        self.graph.resize(count, Node {
            supported_by: Vec::new(),
            total_supports: 0,
            partly_supports: Vec::new(),
            forks: 0,
        });
    }

    fn supported_by(&mut self, index: u16) {
        self.supported_by.push(index);
    }

    fn brick_done(&mut self, index: u16) {
        std::mem::swap(&mut self.graph[index as usize].supported_by, &mut self.supported_by);
    }
}

fn part_2(input: &str) -> u16 {
    let mut state = Part2State {
        graph: Vec::new(),
        supported_by: Vec::new(),
    };

    calculate_supports(input, &mut state);

    let mut graph = state.graph;
    let mut total_sum = 0;
    for index in (0..graph.len()).rev() {
        total_sum += graph[index].total_supports;
        let supported_by = graph[index].supported_by.len();
        let is_fork = supported_by > 1;
        if is_fork {
            graph[index].forks = supported_by as u8;
            for j in 0..graph[index].partly_supports.len() {
                let supported_index = graph[index].partly_supports[j];
                graph[supported_index as usize].forks += supported_by as u8 - 1;
            }
        }

        for i in 0..supported_by {
            let other_index = graph[index].supported_by[i] as usize;

            if is_fork {
                graph[other_index].partly_supports.push(index as u16);
            } else {
                graph[other_index].total_supports += graph[index].total_supports + 1;
            }

            for j in 0..graph[index].partly_supports.len() {
                let supported_index = graph[index].partly_supports[j];
                if graph[other_index].partly_supports.contains(&supported_index) {
                    graph[supported_index as usize].forks -= 1;
                    if graph[supported_index as usize].forks == 1 {
                        let pos = graph[other_index].partly_supports.iter().position(|&x| x == supported_index).unwrap();
                        graph[other_index].partly_supports.remove(pos);
                        graph[other_index].total_supports += graph[supported_index as usize].total_supports + 1;
                    }
                } else {
                    graph[other_index].partly_supports.push(supported_index);
                }
            }
        }
    }

    total_sum
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
