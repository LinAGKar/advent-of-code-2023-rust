use std::io::Read;

fn sum_distances(positions: &[(u8, u8)], expansion_factor: u64) -> u64 {
    positions.windows(2).fold((0, 0, 0), |(
        // Sum of distances between each galaxy up to and including prev_pos
        total_distance,
        // Sum of distances from each previously visited galaxy to prev_pos
        distance_to_prev,
        // Number of galaxies before prev_pos
        galaxies_before_prev,
    ), pair| {
        let prev_pos = pair[0].0 as u64;
        let current_pos = pair[1].0 as u64;

        // Distance from previous to current galaxy, accounting for expansion when there are empty lines/columns
        let traveled_distance = (current_pos - prev_pos - 1) * expansion_factor + 1;

        // Number of galaxies before current_pos
        let galaxies_before_current = galaxies_before_prev + pair[0].1 as u64;

        // For each previous galaxy the distance increases equal to the traveled distances
        let distance_to_current = distance_to_prev + galaxies_before_current * traveled_distance as u64;

        // For each galaxy at current position, add distances between all previously visited galaxies and current galaxy
        // to total.
        let total_distance = total_distance + pair[1].1 as u64 * distance_to_current;

        (total_distance, distance_to_current, galaxies_before_current)
    }).0
}

fn solve(input: &str, expansion_factor: u64) -> u64 {
    // We're looking for the manhattan distance, and there are no obstacles, so the shortest distance is just the sum of
    // the distances on x and y axis, so we can sum up x and y separately. x and y positions need to be in order.
    let mut galaxies_by_column = vec![0; input.lines().count()];
    let mut ys = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut count = 0;
            if c == '#' {
                galaxies_by_column[x] += 1;
                count += 1;
            }
            if count > 0 {
                ys.push((y as u8, count));
            }
        }
    }

    let xs: Vec<_> = galaxies_by_column.into_iter().enumerate().filter_map(|(x, count)| {
        if count > 0 {
            Some((x as u8, count))
        } else {
            None
        }
    }).collect();

    sum_distances(&xs, expansion_factor) + sum_distances(&ys, expansion_factor)
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
