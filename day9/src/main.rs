use std::io::Read;

fn solve(input: &str, backwards: bool) -> i32 {
    let mut nums = Vec::new();
    let mut differences = Vec::new();
    let mut edge = Vec::new();

    input.lines().map(|line| {
        nums.clear();
        for num in line.split_ascii_whitespace() {
            nums.push(num.parse::<i32>().unwrap());
        }

        edge.clear();

        loop {
            differences.clear();
            for i in nums.windows(2) {
                differences.push(if backwards { i[0] - i[1] } else { i[1] - i[0] });
            }
            edge.push(nums[if backwards { 0 } else { nums.len() - 1 }]);
            if differences.iter().all(|&x| x == 0) {
                break edge.iter().copied().sum::<i32>();
            }
            std::mem::swap(&mut nums, &mut differences);
        }
    }).sum()
}

fn part_1(input: &str) -> i32 {
    solve(input, false)
}

fn part_2(input: &str) -> i32 {
    solve(input, true)
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
