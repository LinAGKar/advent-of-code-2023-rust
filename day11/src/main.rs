use std::io::Read;

fn sum_distances(positions: &[u8], expansion_factor: u64) -> u64 {
    positions.windows(2).enumerate().fold((0, 0), |(
            // Sum of distances between each galaxy up to and including prev_pos
            total_distance,
            // Sum of distances from each previously visited galaxy to prev_pos
            distance_to_prev,
        ), (n, pair)| {
        let prev_pos = pair[0] as u64;
        let current_pos = pair[1] as u64;

        // Distance from previous to current galaxy, accounting for expansion when there are empty lines/columns
        let traveled_distance = if current_pos == prev_pos { 0 } else {
            (current_pos - prev_pos - 1) * expansion_factor + 1
        };

        // Number of galaxies before current_pos
        let galaxies_before = n as u64 + 1;

        // For each previous galaxy the distance increases equal to the traveled distances
        let distance_to_current = distance_to_prev + galaxies_before * traveled_distance as u64;

        // Add distances between all previously visited galaxies and current galaxy to total
        let total_distance = total_distance + distance_to_current;

        (total_distance, distance_to_current)
    }).0
}

fn solve(input: &str, expansion_factor: u64) -> u64 {
    // We're looking for the manhattan distance, and there are no obstacles, so the shortest distance is just the sum of
    // the distances on x and y axis, so we can sum up x and y separately.
    let (mut x, y): (Vec<_>, Vec<_>) = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| {
            if c == '#' {
                Some((x as u8, y as u8))
            } else {
                None
            }
        })
    }).unzip();

    // sum_distances depends on positions being in order. y already is.
    x.sort();

    sum_distances(&x, expansion_factor) + sum_distances(&y, expansion_factor)
}

fn part_1(input: &str) -> u64 {
    solve(input, 2)
}

fn part_2(input: &str) -> u64 {
    solve(input, 1000000)
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
