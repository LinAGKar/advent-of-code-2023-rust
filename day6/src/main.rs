use std::io::Read;

fn calculate_race(time: f64, distance: f64) -> u32 {
    // t_a = available time
    // t_b = button time
    // v = velocity = t_b
    // l = traveled distance = (t_a - t_b) * v = (t_a - t_b) * t_b = t_a t_b - t_b^2
    // l_b = best distance
    // We want to find button held times where traveled distance=best distance, so:
    // l = l_b ⇔ l_b = t_a t_b - t_b^2 ⇔ 0 = -t_b^2 + t_a t_b - l_b ⇔ 0 = t_b^2 - t_a t_b + l_b ⇔
    // t_b = t_a/2 ± sqrt((t_a/2)^2 - l_b)

    let root = (time * time / 4.0 - distance).sqrt();

    // Round to floor, time needs to be integer, and in order to reach the needed distance it needs to be greater
    // than or equal to lower root and lower than or equal to higher root. And we don't care about the actual roots,
    // only about how many integers are within the bounds.

    // If time is odd, the centerpoint is gonna fall on a halfstep, so the rounding needs to be adjusted by a half.
    let even = (time as u16) % 2 == 0;
    let root = if even { root } else { root + 0.5 };
    // If the root is an integer, the roots will give us exactly the same distance as the best. And we need greater
    // distance, so exclude those points.
    let x = if root.fract() == 0.0 { root - 1.0 } else { root }.floor() as u32 * 2;
    if even { x + 1 } else { x }
}

fn part_1(input: &str) -> u32 {
    // Iterate over races
    let mut lines = input.lines().map(|line| {
        line.split_ascii_whitespace().skip(1).map(|num| num.parse::<f64>().unwrap())
    });
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    times.zip(distances).map(|(time, distance)| {
        calculate_race(time, distance)
    }).product()
}

fn part_2(input: &str) -> u32 {
    let mut nums = input.lines().map(|line| {
        line.splitn(2, ' ').nth(1).unwrap().chars().filter(|x| x.is_ascii_digit()).fold(0.0, |acc, x| {
            acc * 10.0 + x.to_digit(10).unwrap() as f64
        })
    });

    calculate_race(nums.next().unwrap(), nums.next().unwrap())
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
