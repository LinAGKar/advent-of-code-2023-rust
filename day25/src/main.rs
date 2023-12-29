use std::{io::Read, collections::{HashMap, hash_map::Entry::{Occupied, Vacant}}, cmp::{min, max}};

use rand::Rng;

#[derive(Debug, Clone)]
struct Node {
    count: u16,
}

impl Node {
    fn new() -> Self {
        Node {
            count: 1,
        }
    }
}

fn part_1<'a>(input: &'a str) -> u32 {
    let mut indexes_by_name = HashMap::<&'a _, _>::new();
    let mut next_index = 0;

    // Helper for translating names into contiguous integers, so we can use a vec instead of HashMap
    let mut get_index = |name: &'a _| {
        match indexes_by_name.entry(name) {
            Occupied(entry) => (false, *entry.get()),
            Vacant(entry) => {
                let index = next_index;
                next_index += 1;
                entry.insert(index);
                (true, index)
            }
        }
    };

    let mut nodes = Vec::new();
    let mut src_edges = Vec::new();
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        let (new, left) = get_index(left);
        if new {
            nodes.push(Node::new());
        }
        for right in right.split_ascii_whitespace() {
            let (new, right) = get_index(right);
            if new {
                nodes.push(Node::new());
            }
            src_edges.push((min(left, right), max(left, right)));
        }
    }

    let mut rng = rand::thread_rng();
    let mut edges = Vec::new();
    loop {
        for node in &mut nodes {
            node.count = 1;
        }
        edges.extend(src_edges.iter().copied());
        let mut node_count = nodes.len();
        while node_count > 2 {
            let index = rng.gen_range(0..edges.len());
            let (a, b) = edges.swap_remove(index);
            nodes[a].count += nodes[b].count;
            let mut i = 0;
            while i < edges.len() {
                let (c, d) = edges[i];
                if (c, d) == (a, b) {
                    edges.swap_remove(i);
                } else if c == b {
                    edges[i] = (min(a, d), max(a, d));
                    i += 1;
                } else if d == b {
                    edges[i] = (min(a, c), max(a, c));
                    i += 1;
                } else {
                    i += 1;
                }
            }
            node_count -= 1;
        }
        if edges.len() == 3 {
            let (a, b) = edges[0];
            break nodes[a].count as u32 * nodes[b].count as u32;
        }
        edges.clear();
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);
}
